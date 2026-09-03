# Study: Simplifying SolidSpec toward OpenSpec's UX

**Status:** research note (no code changes)
**Comparison target:** [Fission-AI/OpenSpec](https://github.com/Fission-AI/OpenSpec) — a lightweight SDD
framework for AI coding agents, explicitly positioned as "fluid not rigid / iterative not waterfall / easy
not complex."

## 1. OpenSpec's actual surface

**Install:**
```bash
npm install -g @fission-ai/openspec@latest
cd your-project
openspec init
```
Two commands, no PATH wrangling, no toolchain beyond Node (already on most dev machines).

**CLI verbs (8 total, all one word, no subcommand trees):**
`init`, `list`, `show`, `validate`, `view`, `archive`, `update`, `config`.

**Artifacts (5, fixed — no methodology to pick):**
```
openspec/
├── specs/                # requirements + scenarios
├── changes/<name>/       # proposal.md, specs/, design.md, tasks.md
└── archive/              # completed changes
```
`<name>` is a slug the human picks, not an auto-numbered ID to track.

**Where the actual workflow logic lives:** almost entirely in 4 AI-agent slash commands
(`/opsx:propose <idea>`, `/opsx:apply`, `/opsx:archive`, `/opsx:explore`), each doing several steps at once
inside the agent's own turn. The CLI itself only inits/inspects/validates/archives — it is not where phase
orchestration happens. An "expanded" profile adds a few more slash commands (`/opsx:new`, `/opsx:continue`,
`/opsx:ff`, `/opsx:verify`, `/opsx:bulk-archive`, `/opsx:onboard`) for teams that want more control, but the
default profile stays at 4.

## 2. SolidSpec's actual surface, measured

**Install** (from the current README): git clone → `cargo build --release` → manually copy the binary to
`/usr/local/bin` or edit `PATH` in a shell profile (bash/zsh/PowerShell instructions spelled out in full).
No `cargo install solidspec` (not published to crates.io), no one-line install script — even though
`.github/workflows/release.yml` already builds prebuilt binaries per platform, the README never points at
them as an install path.

**CLI verbs:** 26 top-level `Commands` in `src/cli/mod.rs` — `init`, `intent`, `specify`, `clarify`, `plan`,
`security-review`, `tasks`, `implement`, `apex`, `tdd-tests`, `tdd-refactor`, `tests`, `evidence`, `analyze`,
`review`, `checklist`, `pipeline`, `preset`, `change`, `extension`, `okf`, `upgrade`, `completions`, `check`,
`status`, `ship` — several with their own subcommand trees (`preset add/remove/list/search/info`,
`extension add/remove/enable/disable/list/search/info`, `change propose/list/archive`, `okf generate/validate`)
and multi-flag surfaces (`pipeline` alone has 9 flags: `--new`, `--from`, `--to`, `--only`, `--force`,
`--dry-run`, `--auto`, `--no-agent`, `--schema`).

**Artifacts:** 7 workflow schemas, 4 to 11 artifacts each (`minimal` → `spec-driven` → `security-first` →
`tdd-driven` → `intent-driven` → `apex-driven` → `intent-apex`), all presented side-by-side in the README's
"Workflows and Methodologies" table *before* the install instructions — a methodology choice is asked of the
reader before they've run a single command.

**Feature identity:** `specs/<NNN>-<slug>/` numbered directories, resolved through a 4-level fallback
(explicit arg → `SOLIDSPEC_FEATURE` env var → git branch pattern `\d{3}-.*` → latest `specs/` dir) — a real
mental model to learn, documented as such in CLAUDE.md, versus OpenSpec's plain `changes/<name>/`.

**Where the workflow logic lives:** almost entirely in the Rust CLI itself — one subcommand per SDD phase,
each with its own flags, its own DAG-gate logic (`src/core/artifact_graph.rs`), its own generator
(`spec_parser.rs`, `task_generator.rs`, `test_generator.rs`, ...). The AI agent's job per phase is comparatively
narrow (fill in one template file); SolidSpec's Rust code carries the orchestration weight OpenSpec puts on
the agent's own slash-command turn.

## 3. Root cause, before the tactical list

The gap isn't really "OpenSpec has fewer commands" — it's that **OpenSpec put almost all workflow
orchestration inside a handful of AI-agent slash commands and kept the CLI to inspect/init/validate/archive**,
while SolidSpec put the orchestration inside the CLI itself (one subcommand per phase, one schema per
methodology, one artifact-graph engine to gate them). Every tactical simplification below chips at the
*symptom* (verbosity, choice overload, install friction); actually closing the gap to OpenSpec's felt
simplicity would mean shrinking how much of the phase logic lives in Rust subcommands versus how much lives
in one or two rich agent-side commands (`pipeline` already points this direction — see #2 and #9 below).
That's a bigger, riskier redesign than anything else here, so it's named once, not repeated per item.

## 4. Top 10 simplifications

Ranked by impact-for-effort, not by how they're listed above.

### 1. Ship a real one-line install
**Problem:** git clone + `cargo build --release` + manual PATH edit is the *only* documented path, despite
prebuilt binaries already existing via `release.yml`.
**Fix:** publish to crates.io (`cargo install solidspec`) and/or add an install script
(`curl -fsSL https://.../install.sh | sh`) that fetches the right GitHub Release asset — mirrors what
`rustup`/most Rust CLIs already do. Make this the *first* thing README shows, cut the manual-PATH walkthrough
down to a troubleshooting footnote.
**Effort:** low (crates.io publish is a `cargo publish` + a `[package]` license/metadata check; install
script is a day). **Impact:** highest — this is the very first friction a new user hits.

### 2. Collapse the front door to one command
**Problem:** 26 top-level commands is the CLI equivalent of OpenSpec's 8 — a new user has no obvious single
verb to start with beyond reading the whole README.
**Fix:** `solidspec pipeline --new "..." --auto` already *is* the one-command path — it's just not
presented as the front door. Add a short top-level alias, e.g. `solidspec go "description"` (or rename
`pipeline`'s common case), and restructure `--help`/README so the individual phase commands
(`specify`/`plan`/`tasks`/...) are visibly "what `go` runs for you, callable standalone for control" rather
than 20 peer-level entries.
**Effort:** low (a clap alias + doc restructuring, no behavior change). **Impact:** high — directly answers
"what do I even type."

### 3. Make the schema choice invisible by default
**Problem:** the README's workflow-comparison table (7 schemas, "Choosing a Workflow") appears before
install — a methodology decision is asked of a reader who hasn't typed a command yet.
**Fix:** `init`/`pipeline` already default to `spec-driven` silently. Move the 7-schema comparison table out
of the top of the README into an "Advanced: choosing a methodology" section reached only after the quick
start. Nothing about the CLI changes — just what a first-time reader is shown first.
**Effort:** trivial (doc reorg). **Impact:** high perceived-complexity reduction for near-zero engineering
risk.

### 4. Stop narrating the feature-ID model up front
**Problem:** `specs/<NNN>-<slug>/` plus its 4-level resolution order is real, useful machinery — but it's
explained prominently (README, CLAUDE.md, every command's `--help`) as if a user must understand it before
running `plan`/`tasks`. OpenSpec's `changes/<name>/` needs zero explanation.
**Fix:** since auto-detection already makes `feature_id` optional on nearly every command, stop mentioning
IDs in the quick-start path entirely ("run `solidspec plan` — it finds your feature automatically") and move
the ID-resolution mechanics to a reference doc, the same way OpenSpec never surfaces "how do I identify a
change" as a concept a user needs.
**Effort:** trivial (doc-only). **Impact:** medium — real users rarely pass an explicit ID today anyway;
this is closing the perception gap, not the actual gap.

### 5. Lead with one artifact set, not seven
**Problem:** `minimal` (4 artifacts) through `intent-apex` (11 artifacts) are all documented as equally
first-class in the README's opening comparison table.
**Fix:** document `spec-driven`'s artifact chain (`spec → plan → tasks → implement`, the same shape as
OpenSpec's `proposal → design → tasks`) as *the* workflow in the quick start; move `security-first`,
`tdd-driven`, `intent-driven`, `apex-driven`, `intent-apex` into a "Recipes for specific needs" doc a user
reaches only once they already know they need OWASP gating, real TDD, or intent tracing.
**Effort:** trivial (doc reorg, ties into #3). **Impact:** high — this is the single biggest source of
"SolidSpec feels complex" versus OpenSpec's fixed five-artifact shape.

### 6. Add plain-verb aliases for jargon commands
**Problem:** `specify`, `checklist`, `evidence`, `security-review`, `tdd-refactor` require learning SDD/IDSD
vocabulary before typing anything; OpenSpec's `list`/`show`/`validate`/`archive` need no glossary.
**Fix:** add non-breaking clap aliases: `new`/`create` → `specify`; `check`/`lint` → already `check` exists
for prerequisites, so this one needs a distinct name, e.g. keep `review` as-is but alias `checklist` under
`validate` to match the verb a user already expects from *any* spec tool. Keep the descriptive originals for
discoverability in `--help`; aliases are purely additive.
**Effort:** low (a handful of `#[command(alias = "...")]` attributes). **Impact:** medium — lowers the
vocabulary tax without touching any behavior or existing scripts.

### 7. Make `status` the answer to "what do I run next"
**Problem:** OpenSpec's `view` opens a dashboard that tells you where things stand; SolidSpec's `status`
does similar DAG-based reporting today but isn't positioned as *the* command to run when unsure — commands
instead each print their own "next step" hints inconsistently.
**Fix:** standardize every phase command's success output to end with one line: `Next: solidspec <verb>` (or
`Feature complete — try 'solidspec ship'`), and lead the README's quick-start with `solidspec status` as
step zero after `init`, the same way OpenSpec's docs lead with `view`.
**Effort:** low (a shared helper in `src/cli/ux.rs`, called from each command's success path).
**Impact:** medium-high — turns 26 commands into "one dashboard + whatever it tells you to run."

### 8. Quiet the agent-registration mechanics
**Problem:** `init`'s own output ("Registered commands for N agent(s): claude") and the README's 20-agent
directory/format table expose per-agent plumbing (Markdown vs Toml, flat vs directory-based, Copilot's dual
files) that OpenSpec keeps entirely invisible despite supporting 30+ tools.
**Fix:** collapse `init`'s output to one line ("AI agent commands ready — run `solidspec check` for
details"), move the full agent-format matrix from the main README into a dedicated `docs/agents.md`
reference. No functional change — SolidSpec still detects/writes for all 20 agents exactly as today.
**Effort:** trivial. **Impact:** medium — removes plumbing detail nobody asked to see at `init` time.

### 9. Shrink `pipeline`'s flag surface for the common case
**Problem:** 9 flags (`--new`, `--from`, `--to`, `--only`, `--force`, `--dry-run`, `--auto`, `--no-agent`,
`--schema`) on one command is a lot to scan versus OpenSpec's near-flagless `/opsx:propose`/`/opsx:apply`.
**Fix:** ship the common cases as short, flagless spellings — e.g. `solidspec go "description"` (full
auto-pipeline for a new feature, `--new --auto` baked in) and `solidspec continue` (resume from wherever the
DAG says work is ready) — while keeping today's `pipeline` with all its flags as the explicit, scriptable
form underneath. Mirrors OpenSpec's split between the 4-verb default profile and its flaggier "expanded"
profile.
**Effort:** medium (two new thin wrappers over existing `pipeline::run` logic). **Impact:** medium-high for
the everyday case, without removing any existing capability.

### 10. Restructure the README itself around the OpenSpec shape
**Problem:** today's README order is: problem statement → full 7-schema comparison table → each schema
explained in depth → *then* install → quick reference. OpenSpec's is: what it is → install (2 lines) → one
worked example → *then* deeper reference.
**Fix:** reorder to: one-paragraph pitch → install (#1) → one worked example using the single default
workflow (#3/#5) → "next steps" pointing at status (#7) → *then* the methodology comparison, agent matrix,
and full command reference as later sections for people who already want more.
**Effort:** low (pure reorg of existing content, nothing new to write). **Impact:** high — this is the
single document most people judge "is this simple?" from, and right now it front-loads exactly the
complexity a first-time reader shouldn't have to absorb yet.

## 5. What this list deliberately does not touch

None of the above removes a schema, a phase command, a flag, or an agent integration — SolidSpec's deeper
capability (constitution gates, DAG-based artifact resolution, 4-lane ship gate, IDSD/APEX methodologies,
extension/preset plugin systems) is real differentiated value OpenSpec doesn't have, and is not what's being
called "too verbose" here. Every item is about what's shown *first* and *by default*, not about cutting
capability — the same way OpenSpec itself keeps an "expanded" profile with more slash commands for teams
that want it, while defaulting new users to 4.

## 6. Suggested order if you want to act on this

1. #1 (install) and #10 (README reorder) first — zero risk, immediate first-impression impact, no code
   changes beyond publishing.
2. #3, #4, #5, #8 next — all doc-only, compound with #10.
3. #6 (aliases) and #7 (status/next-step hints) — small, additive Rust changes, no breaking changes.
4. #2 and #9 (a `go`/`continue` front door) last — the only items that add new CLI surface, so worth
   scoping as their own small spec-driven cycle rather than doing ad hoc, consistent with how prior
   `okf-rs` integration steps in this repo were sequenced.

## Implementation status

All 10 items above shipped, plus a follow-up architectural piece the user asked for explicitly: an
OpenSpec-style set of AI-agent slash commands (`/spcx:new`/`/spcx:apply`/`/spcx:finalise`/`/spcx:explore`,
spec-driven schema only) that each chain several of the per-phase commands, with the CLI's own front door
narrowed to init/status/validate/go/continue/schemas/pipeline/okf/ship.

- **#1 install** — `scripts/install.sh` (downloads the prebuilt GitHub Release binary, no toolchain) and
  `cargo install --git https://github.com/jyjeanne/solidspec` both verified working end-to-end. Publishing to
  crates.io itself is currently blocked: crates.io requires every dependency to be a published crate, and
  `okf-core`/`okf-analyzer`/`okf-generator`/`okf-validator`/`okf-parser` (vendored per
  `docs/okf-rs-integration-plan.md`) are git-only. Documented as a known limitation in the README, not
  silently worked around.
- **#2 / #9 front door** — `solidspec go "desc"` and `solidspec continue [id]`, thin wrappers over
  `pipeline::run` (`src/cli/go.rs`, `src/cli/continue_cmd.rs`). `pipeline` itself keeps its full flag surface
  unchanged.
- **#3 / #4 / #5 / #10** — README reordered to pitch → install → quick reference → a 3-step quick start
  (using `go`/`spcx:new`/`continue`/`status`, no numeric feature ID in sight) → *then* the full
  Workflows/Methodologies + Choosing-a-Workflow reference material, moved down from just after the pitch.
- A new **`solidspec schemas`** command (`src/cli/schemas.rs`) wires up `core::schema::list_available_schemas`
  — already fully implemented and tested but never called from any command before this — and prints each
  schema's name, artifact count, and a new one-line `use_case` field added to every `schemas/*/schema.yaml`
  and `WorkflowSchema`/`SchemaInfo`.
- **#6 aliases** — `solidspec validate` was briefly made the primary name for the old `analyze` command
  (`#[command(name = "validate", alias = "analyze")]`). **Reverted**: a follow-up study comparing it against
  `solidspec check` (which does something genuinely different — environment/project-setup verification, no
  feature argument, no findings/severity report) found the name collision confusing rather than
  clarifying — "validate" reads as a synonym for "check," when the command it named is actually a per-feature
  requirement-traceability analysis with a findings report and severity levels. `analyze` is the primary name
  again; `#[command(alias = "validate")]` keeps the brief rename non-breaking. The broader "rename every
  jargon verb" idea from item #6 was superseded by the slash-command layer below rather than done piecemeal.
- **#7 next-step hints** — `ArtifactGraph::first_ready` (`src/core/artifact_graph.rs`, new, tested) finds the
  next `Ready` artifact in topological order; `init`, `pipeline`/`go`/`continue`, and `status` all print
  `Next: solidspec <verb>` from it.
- **#8 quiet init** — `init`'s per-agent registration output collapsed from listing agent IDs and formats to
  one line ("AI agent commands ready" / "No AI agent detected — run 'solidspec check' for setup details").
- **Per-phase commands hidden, not removed** — `intent`, `specify`, `clarify`, `plan`, `security-review`,
  `tasks`, `implement`, `apex`, `tdd-tests`, `tdd-refactor`, `tests`, `evidence`, `checklist`, `review` all
  get `#[command(hide = true)]` in `src/cli/mod.rs`: absent from `solidspec --help`, fully functional when
  invoked directly, exactly as `pipeline`/`go`/`continue`/the slash commands already call them internally.
  Explicit choice (confirmed with the user before starting): non-breaking over a clean removal.
- **Slash commands** (`templates/commands/spcx/{new,apply,finalise,explore}.md`, wired in
  `src/agents/registry.rs`) — each body chains `solidspec pipeline ... --no-agent --auto` scaffolding with the
  agent filling in content per the existing per-phase instructions, scoped to the default `spec-driven`
  schema (per the user's own scoping choice — other schemas keep using `pipeline --schema <name>` directly).
  Registered for all 20 agents via the existing generic command-writing path (flat
  `solidspec-spcx-<name>` naming); Claude Code additionally gets genuine namespaced commands at
  `.claude/commands/spcx/<name>.md` (`/spcx:new`, not `/solidspec-spcx-new`) — the one agent format in this
  repo where a subdirectory is known to produce a real namespaced slash command. Extending true namespacing
  to other agents that support the same convention is unverified follow-up work, not done here.
- **Housekeeping fix found along the way**: `scripts/generate-graph.sh` now `rm -rf`s the bundle directory
  before regenerating — `write_bundle` only ever writes/updates current concepts, so a renamed function
  (several test functions were renamed while building this) left its old file behind as a new "orphaned
  concept" `validate --ci` failure. Clean rebuild each run avoids that permanently; the incremental-index
  *cache* (`.okf-cache.json`) is untouched, so re-parsing unchanged source files is still skipped.

Verified: `cargo build`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`, and the full test
suite (555 unit + all integration tests) clean; `./scripts/generate-graph.sh` runs clean end to end; manual
end-to-end runs of `go --no-agent` → `continue --no-agent` → `status` in a scratch project, and of
`solidspec init` with a real `.claude/` directory to confirm the namespaced `/spcx:*` files and the generic
`solidspec-spcx-*` files for a non-Claude agent (Cursor).
