# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Test (all)
cargo test

# Test (single test by name)
cargo test test_name

# Test (single integration test file)
cargo test --test pipeline

# Lint
cargo clippy -- -D warnings

# Format check
cargo fmt --check

# Run the CLI
cargo run -- <subcommand>
```

The project enforces `nonstandard_style = deny` and `redundant_clone = deny` via `Cargo.toml` lints. Clippy must pass cleanly.

## Architecture

SolidSpec is a Rust CLI tool (Rust 2024 edition) that scaffolds Specification-Driven Development (SDD) workflows. It generates artifact files that AI coding agents fill in, guided by slash commands registered per-agent.

### Module layout

- **`src/cli/`** — One module per subcommand. `mod.rs` defines `Cli` (clap `Parser`) and the `Commands` enum, then dispatches to each module's `run()` function.
- **`src/core/`** — All business logic, no I/O dependencies on CLI layer:
  - `artifact_graph.rs` — DAG engine (Kahn's algorithm) for artifact dependency resolution and completion detection via filesystem.
  - `schema.rs` — Loads workflow schemas (YAML → `WorkflowSchema` → `ArtifactGraph`). 3-level resolution: project-local `.solidspec/workflows/<name>/schema.yaml` → built-in → default.
  - `pipeline.rs` — Phase list constants, skip logic (checks for existing artifact files), phase type (auto vs. handoff), and log writing.
  - `feature.rs` — Feature ID resolution (4-level: explicit arg → `SOLIDSPEC_FEATURE` env var → git branch pattern `\d{3}-.*` → latest `specs/` dir).
  - `spec_parser.rs`, `task_generator.rs`, `test_generator.rs`, `analyzer.rs`, `review.rs` — Parse and generate the per-phase artifacts.
  - `constitution.rs` — Reads `.solidspec/constitution.md` for architecture guardrails.
- **`src/agents/`** — Manages 20 AI agent integrations:
  - `config.rs` — Static `AGENTS` table mapping agent IDs to their command directory, file format (`Markdown`/`Toml`), CLI binary, argument placeholder, etc.
  - `registry.rs` — Detects agents in a repo, writes/deletes command files with per-agent format differences (flat `.md`/`.toml` for most; directory-based `SKILL.md` for Kimi/OpenCode; dual `.agent.md` + `.prompt.md` for Copilot).
  - `invoker.rs` — Executes the agent CLI non-interactively for automated pipeline runs.
  - `formats.rs` — Renders command file bodies for each format; `guardrails.rs` appends a compliance footer to all command bodies.
- **`src/config/`** — `solidspec.toml` (`RootConfig`) and `.solidspec/config.toml` (`ProjectInternalConfig`). `find_project_root()` walks up from cwd looking for `solidspec.toml` or `.solidspec/`.
- **`src/templates/`** — Tera (Jinja2-compatible) rendering. Templates and shell scripts are `include_str!`-embedded in the binary at compile time. `resolver.rs` applies a 3-level override: project-local → embedded default. Templates are never overwritten on copy; scripts always overwrite.
- **`src/extensions/`**, **`src/presets/`** — Plugin/preset catalog systems, loaded from project-internal config.

### Workflow schemas

Seven built-in schemas in `schemas/`:
- `minimal` (`solidspec init`'s actual default when `--schema` is omitted) — 4 artifacts: spec → plan → tasks → implement
- `spec-driven` — 9 artifacts: spec → clarify → plan → tasks → tests → implement → analyze → review → ship
- `security-first` — adds a `security-review` artifact between plan and tasks
- `tdd-driven` — real failing tests (RED) before implementation, `tdd-refactor` phase after
- `intent-driven` — IDSD mode, adds `intent` as phase 0 before spec
- `apex-driven` — SDD with APEX replacing the manual `implement` handoff
- `intent-apex` — IDSD + APEX, maximum rigor

Custom schemas live at `.solidspec/workflows/<name>/schema.yaml`. Run `solidspec schemas` for this same list with each one's use case, from the terminal.

### Feature directories

All feature artifacts live under `specs/<NNN>-<slug>/` (e.g., `specs/001-auth-system/`). Key files per feature: `spec.md`, `plan.md`, `tasks.md`, `tests/`, `analysis-report.md`, `review-report.md`, optionally `intent.md`.

### Agent command registration

`solidspec init` auto-detects which agents are present (by checking their config dir or CLI binary) and writes slash-command files to each agent's command directory. The `implement` phase is always a `Handoff` (requires human confirmation in interactive pipeline runs); all other phases are `Auto`.

Command bodies are embedded from `templates/commands/<phase>.md` (`src/agents/registry.rs`'s `command_body()`), written with the canonical `$ARGUMENTS` placeholder and translated per-agent by `formats::translate_placeholder`. A project can override a command's body by placing a file at `.solidspec/templates/overrides/commands/<phase>.md` — it takes precedence over the embedded default and still gets the compliance-guardrails footer appended.

### Testing

Unit tests live in `#[cfg(test)]` blocks inside each source file and use `tempfile::TempDir` for isolated filesystem state. Integration tests are in `tests/pipeline.rs` and use `assert_cmd` + `predicates` to run the compiled binary end-to-end.

### Vendored agent skills vs. `solidspec review`

`.opencode/skills/ai-spec-review-skill/` is a vendored, independently-versioned third-party skill (own LICENSE, README, CONTRIBUTING.md — not authored by this project) that performs a 16-dimension AI-agent-driven review of Markdown specs via `scripts/review_spec.py`. It is **intentionally independent** from `src/core/review.rs`'s `solidspec review` heuristics, not a duplicate to be merged: the Rust side does fast, deterministic, no-LLM checks scoped to SDD artifact conventions (placeholders, FR-### traceability, section completeness); the vendored skill does a much broader, LLM-driven engineering review (security, architecture, dependencies, UX, ...) with no knowledge of SolidSpec's artifact format. Update the vendored skill by re-vendoring from upstream, not by hand-editing it to match `review.rs`.

## Knowledge graph (native `solidspec okf`)

A knowledge graph of the codebase lives in `docs/graph/`, generated by `solidspec okf generate` — [okf-rs](https://github.com/jyjeanne/okf-rs)'s generator/analyzer/validator crates, vendored as pinned git dependencies in `Cargo.toml` and wrapped in-process by `src/core/okf.rs` (**no external `okf-rs` binary** — see `docs/okf-rs-integration-plan.md`'s step 2). Extraction is local tree-sitter AST parsing: no LLM calls, nothing leaves the machine. The output is an **OKF bundle**: one plain Markdown file with YAML frontmatter per concept (struct/function/module/...), cross-linked by ordinary Markdown links — it lives in the repo like any other file and its diffs are readable in a PR.

- `docs/graph/knowledge/` — the OKF bundle
- `docs/graph/GRAPH_REPORT.md` — topology summary from `solidspec okf generate`'s own concept-kind breakdown

### Regenerating

Regenerate after significant code changes (new modules, moved functions, refactors):

```bash
./scripts/generate-graph.sh
```

This just builds and runs `solidspec okf generate`/`solidspec okf validate --ci` — no separate tool to install. The incremental-index cache (`.okf-cache.json`, gitignored) speeds up repeat runs — only changed files are re-parsed.

### Querying (use this before broad greps)

`search`/`explore`/`graph`/`diff`/`impact` aren't vendored yet (only `generate`/`validate` are — those need tantivy/graph-algorithm crates this repo doesn't otherwise pull in). Querying the bundle still needs the external CLI:

```bash
# One-time install (not published to crates.io — install from git)
cargo install --git https://github.com/jyjeanne/okf-rs okf-cli

# Ranked full-text search over signatures/descriptions — where is X handled
okf-rs search "how agent command files are written" docs/graph/knowledge --ranked

# One-call context for a symbol: signature, callers, callees, blast radius, public-API/cycle membership
okf-rs explore ArtifactGraph docs/graph/knowledge

# Callers/callees, cycles, isolated concepts, public API, cross-module deps, layers, communities
okf-rs graph callers "classes/src/core/artifact_graph/ArtifactGraph" docs/graph/knowledge
okf-rs graph stats docs/graph/knowledge

# Shortest call path between two concept ids (get exact ids from `okf-rs search`)
okf-rs graph path "classes/src/cli/Cli" "classes/src/core/artifact_graph/ArtifactGraph" docs/graph/knowledge

# Deterministic blast-radius / risk analysis between two git refs (use before a refactor PR)
okf-rs impact HEAD~5 HEAD
```

Run `okf-rs explore <symbol>` on the symbols you are about to modify (it includes blast radius in one call), and check `okf-rs graph stats`/`graph communities` when reviewing changes — edits to concepts with a large blast radius or high fan-in deserve extra test coverage.
