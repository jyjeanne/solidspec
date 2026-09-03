# Study: Integrating `okf-rs` into SolidSpec

**Status:** research note (no code changes)
**Author's other project:** [`jyjeanne/okf-rs`](https://github.com/jyjeanne/okf-rs) — a Rust CLI/toolkit that turns a
codebase into a git-native, AI-readable knowledge base using the **Open Knowledge Format (OKF)**.

## 1. What `okf-rs` is

`okf-rs` is a Cargo workspace of 18 small crates (`okf-core`, `okf-parser`, `okf-tree-sitter`, `okf-lsp`,
`okf-analyzer`, `okf-generator`, `okf-render`, `okf-validator`, `okf-search`, `okf-graph`, `okf-query`,
`okf-enrich`, `okf-arch`, `okf-dita`, `okf-mcp`, `okf-watch`, `okf-docs`, `okf-cli`) that extract a call graph
from source (via `tree-sitter`, optionally disambiguated by a real LSP such as `rust-analyzer`/`pyright`) and
emit it as **plain Markdown files with YAML frontmatter**, cross-linked by ordinary Markdown links — an
"OKF bundle" that lives in the repo like any other file.

Relevant capabilities:

- **Languages:** Rust, Python, TypeScript/JavaScript, Go, Java, C#, PHP, Kotlin, C/C++, Swift.
- **CLI surface:** `init`, `scan`, `generate` (`--lsp`, `--enrich`, `--check-determinism`, `--check-fresh`,
  `--no-cache`), `watch`, `validate` (`--ci`), `search` (ranked/semantic), `explore`, `coverage`, `graph`
  (callers/callees/cycles/API-surface/layers/communities/pattern-matching), `diff` (concept diff between git
  refs), `impact` (blast radius / risk metrics), `review` (renders impact analysis as PR-comment-ready
  Markdown), `docs` (export to HTML/consolidated-Markdown/PDF/GraphML/Obsidian).
- **Determinism:** no wall-clock timestamps, deterministic ordering, `--check-determinism` — output is
  byte-identical for identical source, so bundle diffs in a PR are meaningful and auditable without re-running
  the tool.
- **Incremental:** per-file content-hash caching; `watch` mode with debouncing.
- **MCP server (`okf-mcp`):** exposes `search`, `search_ranked`, `search_semantic`, `explore`, `coverage`, and
  a single consolidated `graph` tool (replacing 13 earlier tools to cut system-prompt token overhead) so an
  AI agent can query the graph instead of re-reading files. Has a `--benchmark` flag to measure token savings
  vs. grep-and-read.
- **Agent integration on `init`:** can write/track `CLAUDE.md`, `AGENTS.md`, `.github/copilot-instructions.md`.
- **Install:** prebuilt binaries (GitHub Releases), `cargo install --git https://github.com/jyjeanne/okf-rs okf-cli`,
  or build from source. Not published to crates.io as of this writing — it's pulled from git.

Design philosophy note worth keeping: the project explicitly favors "being wrong in public" — errors show up
as visible bundle diffs a human reviewer can catch, rather than being buried in an opaque database.

## 2. Where SolidSpec already touches this space

SolidSpec already generates a knowledge graph of **its own codebase** (not of projects it scaffolds) via
[graphify](https://github.com/Graphify-Labs/graphify), a separate Python tool:

- `scripts/generate-graph.sh` shells out to `graphify extract --code-only --cargo` + `graphify cluster-only`,
  writing `docs/graph/{graph.json,graph.html,GRAPH_REPORT.md,manifest.json}`.
- Requires `uv tool install graphifyy` (or `pipx`) — a **Python dependency** in an otherwise pure-Rust project.
- CLAUDE.md instructs contributors to query it with `graphify query|explain|affected|god-nodes|path` before
  broad greps, and to regenerate after significant refactors.
- It's dev/contributor tooling only — it ships nothing to end users of `solidspec init`.

Separately, and more centrally to the product: SolidSpec's whole reason to exist is orchestrating AI-agent
phases (`spec → plan → tasks → tests → implement → analyze → review → ship`) where a phase's quality depends on
the agent having accurate context about the *target codebase* (the one being spec'd, not SolidSpec itself).
Today that context comes only from whatever the agent greps/reads live — `analyze.rs`'s cross-artifact checks
and `review.rs`'s preflight checks are text/pattern-based over the spec artifacts, with no structural view of
the actual code the feature touches.

## 3. Two distinct integration opportunities

These are independent — either can be done alone.

### A. Replace `graphify` for SolidSpec's own contributor-facing knowledge graph

Swap `scripts/generate-graph.sh`'s `graphify extract/cluster-only` calls for `okf-rs generate` (+ `okf-rs
graph`/`okf-rs query`-equivalents). Concretely:

- Removes the Python/`uv`/`pipx` dependency for contributors — `okf-rs` is a Rust binary, consistent with a
  project whose `Cargo.toml` already enforces `nonstandard_style`/`redundant_clone` lints and is 100% Rust
  otherwise.
- Output becomes Markdown+YAML instead of `graph.json`/`graph.html` blobs — genuinely reviewable in a PR diff,
  which fits CLAUDE.md's own existing instruction to keep `docs/graph/` current and inspectable.
- Gains `impact`/`diff`/`review` commands graphify doesn't have — e.g. `okf-rs review` could generate a
  PR-comment-style blast-radius summary for SolidSpec's own PRs.
- Cost: rewriting `scripts/generate-graph.sh`, updating CLAUDE.md's query examples, and re-learning the
  `okf-rs` query vocabulary. Low risk — this path touches no shipped behavior, only contributor tooling.

This is a same-shape swap, not a new capability — worth doing for dependency hygiene but not the interesting
part of "benefits."

### B. Ship OKF as a first-class capability of the workflows SolidSpec scaffolds

This is where `okf-rs` is actually a good structural fit, because SolidSpec already has the extension points
for it:

1. **New preset/extension** (`src/presets/`, `src/extensions/`): an `okf` preset that, on `solidspec init`,
   drops an `okf.toml` into the target project and (optionally) registers `okf-mcp` alongside the agent
   command files SolidSpec already writes per-agent in `src/agents/registry.rs`. SolidSpec already detects
   and registers 20 agents' native command formats — adding "also register this project's MCP server for
   agents that support MCP" is the same shape of work `registry.rs` already does.

2. **Pipeline hook before `plan`/`tasks`** (`src/core/pipeline.rs`): run `okf-rs generate` (cached,
   incremental — cheap on repeat runs) before the planning phase so the plan-authoring agent can `explore`/
   `search` the real call graph of the codebase it's planning against, instead of grepping cold. This
   directly serves SolidSpec's stated problem (agents missing context, scope creep) with a mechanism
   `okf-rs` was purpose-built for (its own pitch is "every file an AI agent opens costs its full size in
   context tokens... that's fundamentally a lookup").

3. **`analyze` phase** (`src/core/analyzer.rs`): today this does textual cross-artifact consistency checks
   (e.g. FR-### traceability across spec/plan/tasks). `okf-rs impact`/`diff` against the target files named
   in `tasks.md` would let `analyze` also assert "the tasks correspond to real code the graph confirms
   exists / is reachable," and flag orphaned FR-### references or unexpected blast radius — a structural check
   layered on top of the existing textual one, not a replacement.

4. **`review` phase / ship gate** (`src/core/review.rs`, `security_review.rs`): `okf-rs review` already
   renders impact analysis as PR-comment-ready Markdown — this maps almost 1:1 onto SolidSpec's own
   `review-report.md` artifact. Could be surfaced as one lane of the 4-lane parallel ship gate SolidSpec's
   README describes.

5. **`security-first` schema**: `okf-rs graph`'s callers/callees/dead-code/blast-radius analysis is directly
   useful evidence for a security reviewer artifact — e.g. "does this auth-touching function have unexpected
   external callers."

## 4. Concrete benefits if integrated (product-level, option B)

- **Context efficiency for the agents SolidSpec drives.** This is SolidSpec's core value prop (structure
  around AI agents) and `okf-rs`'s core value prop (cut redundant context-token spend) are the same problem
  from two ends of the pipeline.
- **Traceability with a structural backstop.** SolidSpec already tracks FR-### requirement IDs through
  spec→plan→tasks textually; `okf-rs`'s resolved call graph gives a way to verify those references land on
  real, reachable code rather than trusting the artifact prose.
- **Git-native, auditable, deterministic** — matches SolidSpec's own philosophy of versioning every artifact
  in the repo; `okf-rs` bundle diffs are readable in the same PR as the spec/plan/tasks changes they justify.
- **MCP-first.** SolidSpec already deals with per-agent capability differences (`config.rs`'s `AGENTS` table);
  agents that support MCP (Claude Code, several others in the 20-agent list) get `okf-mcp` for free instead of
  each phase's prompt template having to explain "go grep the codebase."
- **Same language, same workspace shape.** `okf-rs`'s crates are already split for reuse by other Rust tools
  (its own README says as much) — a tighter integration than shelling out is at least plausible later (see
  risks below).

## 5. Risks / open questions

- **New external dependency, not yet on crates.io.** Today it's installed via `cargo install --git ...` or a
  prebuilt binary — same shape as the current `graphify` dependency (external binary, detected via `which`,
  the pattern SolidSpec already uses for agent CLIs in `src/agents/config.rs`), but it does mean SolidSpec
  would depend on another actively-developed project by the same author rather than a stable published crate.
  Pin a specific tag/rev if adopted.
- **Scope creep vs. "spec-driven ceremony first."** SolidSpec is deliberately about the *artifact* workflow;
  adding a required knowledge-graph generation step to every `init`/`plan` could slow the `minimal` schema's
  whole pitch ("four artifacts, minimal ceremony"). This argues for an **opt-in preset/extension**, not a
  default-on pipeline step — consistent with how `security-review` is already schema-specific rather than
  universal.
- **Overlap with `analyzer.rs`/`review.rs`, not a replacement.** The existing heuristics are deliberately
  fast and deterministic with no LLM/tool dependency (per CLAUDE.md's note on the vendored review skill vs.
  `review.rs`); `okf-rs` should be layered alongside them the same way the vendored `ai-spec-review-skill` is
  — additive, independently versioned, not merged into the Rust heuristics.
- **Workspace crate reuse (`okf-core` etc. as a library dependency) is a deeper integration** than invoking
  the CLI as a subprocess, and would tie SolidSpec's build to `okf-rs`'s internal crate APIs, which aren't
  described as stable/published. Not recommended as a first step.

## 6. Recommendation

1. **Low-risk first step (dependency hygiene):** swap `scripts/generate-graph.sh` from `graphify` to
   `okf-rs generate`, update the CLAUDE.md query section — removes the only non-Rust tool dependency in the
   project. Small, self-contained, no product behavior change.
2. **If there's appetite for the product feature:** add `okf` as an opt-in preset under `src/presets/`
   (mirrors the existing preset/extension plugin system) that scaffolds `okf.toml` + MCP registration on
   `solidspec init --preset okf`, without wiring it into any pipeline phase yet.
3. **Only after that's proven useful:** wire `okf-rs generate`/`impact`/`review` into `analyze` and `review`
   phases as an additional, clearly-labeled check — never replacing the existing deterministic heuristics.

Steps 2–3 are a meaningful feature addition, not a drive-by change — worth scoping as its own spec (fittingly,
via SolidSpec's own `spec-driven` workflow) rather than doing ad hoc, if you want to proceed.
