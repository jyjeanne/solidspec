# SolidSpec Architecture

## Overview

SolidSpec is a Rust CLI built with a layered architecture that separates concerns into distinct module trees. The CLI layer handles user interaction, the core layer contains domain logic, and specialized modules handle agent integration, templates, presets, and extensions.

```
                        ┌──────────────┐
                        │    main.rs   │
                        │  (entrypoint)│
                        └──────┬───────┘
                               │
                        ┌──────▼───────┐
                        │   cli/mod    │  15 subcommands
                        │   (clap)     │
                        └──┬───┬───┬───┘
                           │   │   │
              ┌────────────┘   │   └────────────┐
              ▼                ▼                 ▼
        ┌──────────┐    ┌──────────┐     ┌──────────────┐
        │  core/   │    │ agents/  │     │  templates/  │
        │ (domain) │    │ (AI)     │     │  (render)    │
        └──────────┘    └──────────┘     └──────────────┘
              │                                 │
         ┌────┴────┐                    ┌───────┴──────┐
         ▼         ▼                    ▼              ▼
   ┌──────────┐ ┌──────────┐    ┌──────────┐   ┌──────────┐
   │ presets/ │ │extensions│    │ config/  │   │ embedded │
   │          │ │  /hooks  │    │          │   │ templates│
   └──────────┘ └──────────┘    └──────────┘   └──────────┘
```

## Module Tree

### `src/main.rs`

Entrypoint. Parses CLI args, initializes logger, dispatches to `cli::run()`.

### `src/cli/` — Command Layer

Thin handlers for each CLI subcommand. Each file maps to one command. No business logic — delegates to `core/`, `agents/`, `templates/`, `presets/`, `extensions/`.

| File | Command | Responsibility |
|------|---------|---------------|
| `mod.rs` | — | Clap `Cli` struct, `Commands` enum, dispatch |
| `init.rs` | `init` | Bootstrap project, register agents, create git repo |
| `specify.rs` | `specify` | Create feature branch + spec from template |
| `clarify.rs` | `clarify` | Identify markers, generate questions |
| `plan.rs` | `plan` | Generate plan + supporting docs, constitution checks |
| `tasks.rs` | `tasks` | Generate phased task breakdown |
| `implement.rs` | `implement` | Parse tasks, fire hooks, list pending work |
| `tests_cmd.rs` | `tests` | Generate test scaffolds from acceptance scenarios |
| `analyze.rs` | `analyze` | Run consistency analysis, print report |
| `checklist.rs` | `checklist` | Generate/append quality checklists |
| `review.rs` | `review` | Preflight spec quality review with dimension scoring |
| `pipeline.rs` | `pipeline` | Multi-agent pipeline orchestrator (7 phases) with agent CLI invocation |
| `change.rs` | `change` | Change-based workflow: propose, list, archive (delta specs) |
| `status.rs` | `status` | DAG-based artifact completion status (schema-driven) |
| `check.rs` | `check` | Verify prerequisites |
| `preset.rs` | `preset` | Preset CRUD subcommands |
| `extension.rs` | `extension` | Extension CRUD subcommands |
| `upgrade.rs` | `upgrade` | Refresh templates and agent commands |
| `completions.rs` | `completions` | Generate shell completions |
| `ux.rs` | — | Step tracker, status indicators (shared UI) |

### `src/core/` — Domain Logic

Pure business logic with no CLI dependency. Can be used as a library.

| File | Responsibility |
|------|---------------|
| `spec_parser.rs` | Parse `spec.md` into `ParsedSpec` (stories, requirements, markers, entities) |
| `artifact_graph.rs` | DAG engine: artifact nodes, Kahn's algorithm topological sort, completion detection, state computation |
| `schema.rs` | Workflow schema loading (YAML), 3-level resolution (project-local → built-in → default), 3 built-in schemas |
| `review.rs` | Preflight spec quality review, dimension scoring, placeholder detection |
| `review.rs` | Preflight spec quality review, dimension scoring, placeholder detection |
| `change.rs` | Change-based workflow: delta spec parser (ADDED/MODIFIED/REMOVED), archive merge engine, change metadata |
| `constitution.rs` | Load constitution, parse gates, check plan compliance |
| `task_generator.rs` | Generate `TaskList` from spec + plan, organize by phases |
| `test_generator.rs` | Extract Given/When/Then scenarios, detect framework, generate test scaffolds |
| `pipeline.rs` | Pipeline phase definitions, skip conditions, filtering, log generation |
| `analyzer.rs` | Cross-artifact consistency validation, severity heuristic |
| `feature.rs` | Feature numbering, branch name generation, 4-level resolution |
| `git.rs` | Git operations: init, branch creation, current branch detection |
| `errors.rs` | Typed error enum `SolidSpecError` with what/where/fix |
| `token.rs` | GitHub token resolution (CLI flag > env vars) |
| `vscode.rs` | Deep-merge `.vscode/settings.json` |

### `src/agents/` — AI Agent Integration

Manages 20 AI coding agents with data-driven configuration and CLI invocation.

| File | Responsibility |
|------|---------------|
| `config.rs` | `AGENTS` const table — 20 agents with ID, dir, format, placeholder, CLI binary/flags |
| `registry.rs` | Detection, registration, unregistration of commands (phase-specific prompts) |
| `registrar.rs` | Re-exports from registry |
| `formats.rs` | Markdown/TOML/Vibe-skill rendering, placeholder translation, path adjustment |
| `invoker.rs` | Non-interactive CLI invocation of AI agents, phase-specific prompt generation with 300s timeout |
| `personas.rs` | 8 role-based agent personas (Spec Writer, Architect, Implementer, Code Reviewer, etc.) with verification checklists |
| `guardrails.rs` | Anti-rationalization table + mandatory compliance checklist injected into every prompt |

**Agent-specific handling** (in `registry.rs`):
- **Copilot**: `.agent.md` + companion `.prompt.md`
- **Kimi**: Directory-based skills with dot-separator (`.kimi/skills/solidspec.specify/SKILL.md`)
- **Vibe**: Directory-based skills with hyphen-separator (`.vibe/skills/solidspec-specify/SKILL.md`), `user-invocable: true` frontmatter
- **OpenCode**: Directory-based skills with hyphen-separator (`.opencode/skills/solidspec-specify/SKILL.md`), `name:` + `description:` YAML frontmatter
- **Gemini/Tabnine**: TOML format with `{{args}}`

**CLI invocation** (in `invoker.rs`):
- Builds detailed, phase-specific prompts (not generic "execute the workflow")
- Invokes agent CLI via `std::process::Command` with non-interactive flags
- Returns `Success`/`NotAvailable`/`Failed` — pipeline falls back to handoff on failure
- Agent-specific invocation: `claude -p`, `vibe -p`, `codex exec`, `kimi --yolo`, etc.

### `src/templates/` — Template Engine

| File | Responsibility |
|------|---------------|
| `mod.rs` | Tera rendering (autoescape disabled), embedded template constants |
| `resolver.rs` | 4-layer resolution: overrides > presets > extensions > embedded |

**Resolution hierarchy:**
```
1. .solidspec/templates/overrides/    (project tweaks)
2. .solidspec/presets/<id>/templates/ (sorted by priority)
3. .solidspec/extensions/<id>/templates/
4. Embedded in binary (include_str!)
```

### `src/presets/` — Preset System

| File | Responsibility |
|------|---------------|
| `manifest.rs` | Parse + validate `preset.yml` (schema, semver, ID regex, template types) |
| `registry.rs` | `PresetRegistry` — JSON persistence, priority sort, search |
| `manager.rs` | Add/remove/list/search/info + recursive directory copy |

### `src/extensions/` — Extension System

| File | Responsibility |
|------|---------------|
| `manifest.rs` | Parse + validate `extension.yml` (commands, hooks, dependencies) |
| `registry.rs` | `ExtensionRegistry` — enable/disable, deep-copy, name resolution |
| `manager.rs` | Install (--dev), remove, enable, disable, list, search |
| `hooks.rs` | Cross-platform hook executor (sh/PowerShell/cmd fallback) |

### `src/config/` — Configuration

| File | Responsibility |
|------|---------------|
| `mod.rs` | `RootConfig` (solidspec.toml), `PipelineConfig` (per-phase agent mapping), `ProjectInternalConfig`, `InitOptions`, project root finder |

## Data Flow

### Specify → Plan → Tasks Pipeline

```
User description
      │
      ▼
┌─────────────┐    ┌──────────────┐    ┌───────────────┐
│   specify    │───>│    plan      │───>│    tasks      │
│             │    │              │    │               │
│ - branch    │    │ - research   │    │ - phases      │
│ - spec.md   │    │ - plan.md    │    │ - tasks.md    │
│ - checklist │    │ - data-model │    │ - after_tasks │
│             │    │ - contracts  │    │   hook        │
└─────────────┘    │ - quickstart │    └───────────────┘
                   │ - AGENT.md   │
                   └──────────────┘
```

### Template Resolution Flow

```
Command needs template
        │
        ▼
┌─ resolver::load_template() ──────────────────────┐
│                                                   │
│  1. Check overrides/  ──found──> return file      │
│  2. Check presets/ (by priority) ──found──> return │
│  3. Check extensions/ ──found──> return            │
│  4. Return embedded default                        │
│                                                   │
└───────────────────────────────────────────────────┘
        │
        ▼
  templates::render() (Tera, no HTML escaping)
        │
        ▼
  Write to specs/<feature>/
```

### Feature Resolution (4 levels)

```
resolve_feature(explicit_id, project_root)
        │
        ├── Level 1: explicit argument ──found──> return
        ├── Level 2: SOLIDSPEC_FEATURE env ──found──> return
        ├── Level 3: git branch (if \d{3}-.*) ──found──> return
        └── Level 4: latest specs/ directory ──found──> return
```

### Hook Execution Flow

```
Workflow command (tasks, implement)
        │
        ▼
fire_hooks(trigger, project_root, registry)
        │
        ├── For each enabled extension with matching trigger:
        │     │
        │     ├── Resolve hook file path
        │     ├── Check file exists
        │     └── Execute (platform-aware):
        │           ├── Windows .ps1 → powershell
        │           ├── Windows .sh  → sh, fallback cmd
        │           └── Unix         → sh
        │
        └── Failures logged as warnings (non-blocking)
```

### Pipeline Execution Flow

```
solidspec pipeline [--new "desc"] [--from X] [--to Y] [--auto] [--no-agent]
        │
        ▼
┌─ Resolve feature ──────────────────────────────┐
│  --new → next_feature_number() + branch name    │
│  else  → 4-level feature resolution             │
└─────────────────────┬───────────────────────────┘
                      │
        ▼─────────────┘
┌─ Check agent availability ─────────────────────┐
│  For each phase, check if agent CLI is in PATH  │
│  → AllCli (fully automated)                     │
│  → Mixed (some handoff)                         │
│  → Disabled (--no-agent, scaffold only)         │
└─────────────────────┬──────────────────────────┘
                      │
        ▼─────────────┘
┌─ For each phase in [specify→clarify→plan→tasks→tests→implement→analyze] ─┐
│                                                                           │
│  1. Resolve agent (per-phase config > default_agent)                      │
│  2. Check skip condition (artifact exists? force?)                        │
│  3. Generate scaffold:                                                    │
│     └── call cli::{phase}::run() → creates template files                │
│  4. Invoke AI agent (unless --no-agent):                                  │
│     ├── Auto phases → invoker::invoke_agent() with phase-specific prompt  │
│     │   ├── Success → agent fills templates with real content             │
│     │   ├── NotAvailable → fall back to handoff (show manual command)     │
│     │   └── Failed → fall back to handoff (log warning)                   │
│     └── Handoff phases (implement) → prompt user, wait for Enter          │
│  5. After specify with --new → re-detect feature dir                      │
│  6. Record PhaseResult (status, duration, output)                         │
│                                                                           │
└───────────────────────────────────────────────────────────────────────────┘
        │
        ▼
  Write specs/<feature>/pipeline-log.md
```

### Agent CLI Invocation Flow

```
Pipeline Auto phase (specify, plan, tasks, tests, analyze)
        │
        ▼
┌─ invoker::build_phase_prompt() ────────────────┐
│  Generate phase-specific instructions:          │
│  - specify: "Fill spec.md with user stories..." │
│  - plan: "Fill plan.md, research.md, ..."       │
│  - tasks: "Fill tasks.md with actionable..."    │
│  - tests: "Enhance test scaffolds..."           │
│  - analyze: "Validate consistency..."           │
└─────────────────────┬──────────────────────────┘
                      │
        ▼─────────────┘
┌─ invoker::invoke_agent() ──────────────────────┐
│  1. Look up AgentConfig for agent ID            │
│  2. Check cli_binary is non-empty               │
│  3. Check binary exists in PATH (which::which)  │
│  4. Build Command:                              │
│     ├── claude: claude -p "prompt" --allowedTools│
│     ├── vibe: vibe -p "prompt" --max-turns 25   │
│     ├── codex: codex exec "prompt"              │
│     ├── kimi: kimi --yolo "prompt"              │
│     └── others: {binary} {flag} "prompt"        │
│  5. Execute with current_dir = project_root     │
│  6. Return Success/NotAvailable/Failed          │
└─────────────────────────────────────────────────┘
```

### Spec-to-Test Generation Flow

```
solidspec tests [feature-id] [--framework X] [--output-dir Y]
        │
        ▼
┌─ test_generator::extract_scenarios(spec_text) ─┐
│  Parse Given/When/Then blocks from spec.md      │
│  Group by user story index                      │
└─────────────────────┬──────────────────────────┘
                      │
        ▼─────────────┘
┌─ Detect framework ──────────────────────────────┐
│  --framework flag > Cargo.toml/package.json/etc  │
│  Fallback: generic                               │
└─────────────────────┬───────────────────────────┘
                      │
        ▼─────────────┘
┌─ Render test scaffolds ─────────────────────────┐
│  Jest (.test.js) │ pytest (.py) │ Cargo (.rs)    │
│  Go (_test.go)   │ generic (.test.txt)           │
│  One file per user story                         │
└──────────────────────────────────────────────────┘
```

### Change-Based Workflow (Delta Specs)

```
solidspec change propose "Add social login"
        │
        ▼
┌─ create_change(feature_dir, title) ─────────────────┐
│  1. Generate slug from title (lowercase-hyphens)     │
│  2. Create specs/<id>/changes/<slug>/ directory      │
│  3. Write proposal.md (Why/What/Impact/Non-Goals)    │
│  4. Write delta-spec.md (ADDED/MODIFIED/REMOVED)     │
│  5. Write .change.yaml (metadata: status, created_at)│
└──────────────────────┬──────────────────────────────┘
                       │
        User edits proposal + delta-spec
                       │
                       ▼
┌─ archive_change(feature_dir, slug) ──────────────────┐
│  1. Parse delta-spec.md → DeltaSpec                   │
│  2. Read main spec.md                                 │
│  3. merge_deltas(): remove → modify → append          │
│  4. Write merged spec.md                              │
│  5. Move change to changes/archive/<slug>/            │
│  6. Update metadata → status: archived                │
└───────────────────────────────────────────────────────┘
```

### DAG Status Flow

```
solidspec status [feature-id] [--schema X]
        │
        ▼
┌─ schema::load_graph(name, root) ──────────────────┐
│  1. Check .solidspec/workflows/<name>/schema.yaml  │
│  2. Fall back to built-in schema                   │
│  3. Fall back to default (spec-driven)             │
└────────────────────┬───────────────────────────────┘
                     │
                     ▼
┌─ graph.detect_completion(feature_dir) ────────────┐
│  Scan filesystem for generated artifacts           │
│  spec.md → spec done, plan.md → plan done, etc.   │
└────────────────────┬───────────────────────────────┘
                     │
                     ▼
┌─ graph.compute_states(completed) ─────────────────┐
│  For each artifact:                                │
│    Done  — file exists                             │
│    Ready — all deps done, file missing             │
│    Blocked — waiting on deps                       │
└────────────────────┬───────────────────────────────┘
                     │
                     ▼
          Print status table (topological order)

## Key Design Decisions

### 1. Data-driven agent config

All 20 agents are defined in a single `AGENTS` const array. Adding a new agent requires only adding an `AgentConfig` entry — no new code files needed. Special behaviors (Copilot, Kimi, Cursor) are handled by ID checks in `registry.rs`.

### 2. Template auto-escaping disabled

Tera's HTML auto-escaping is explicitly turned off (`tera.autoescape_on(vec![])`) because SolidSpec generates Markdown, not HTML. Without this, `&` becomes `&amp;` in all generated artifacts.

### 3. Constitution gate stripping

When checking plan compliance, the `## Constitution Check` section of the plan itself is stripped before analysis to prevent false positives from the gate checklist text.

### 4. Private registry fields

Both `PresetRegistry` and `ExtensionRegistry` keep their `entries` HashMap private. Access is via methods that return deep copies, preventing accidental mutation of internal state.

### 5. Platform-aware hooks

The hook executor detects Windows vs Unix at compile time (`cfg!(windows)`) and uses appropriate shell: PowerShell for `.ps1`, `sh` with `cmd` fallback for others on Windows, `sh` on Unix.

### 6. Branch-first specify

`specify` creates the git branch before writing any files, so artifacts land on the correct branch. If branch creation fails, files go to the current branch with a warning.

### 7. Pipeline re-detection after specify

When `pipeline --new` runs, the specify phase creates its own feature directory with independent numbering. After specify completes, the pipeline re-resolves the feature directory to pick up the actual name, avoiding mismatches between pre-computed and actual directory names.

### 8. Given/When/Then extraction

Test generation parses acceptance scenarios from spec.md using regex-based extraction of Given/When/Then blocks. Scenarios are grouped by user story index (sorted) to produce one test file per story, ensuring deterministic output regardless of HashMap iteration order.

### 9. Phase-specific agent prompts

Each pipeline phase has a detailed, unique prompt (in `invoker.rs`) telling the AI agent exactly what to fill in. The `implement` command in `registry.rs` has enriched 7-step instructions. Non-implement commands have per-phase instructions (not generic "execute the workflow"). This ensures agents produce useful content when invoked programmatically via CLI.

### 10. Graceful CLI fallback

Agent CLI invocation returns a three-variant enum (`Success`/`NotAvailable`/`Failed`). When an agent's CLI is not installed or fails, the pipeline falls back to handoff mode (shows the manual `/solidspec-*` command to run). This means the pipeline never crashes due to a missing agent — it degrades gracefully.

### 11. Vibe directory-based skills

Mistral Vibe uses a skills system with directory-based discovery (`.vibe/skills/<name>/SKILL.md`), unlike most agents that use flat command files. The `SKILL.md` requires `user-invocable: true` and `allowed-tools:` in YAML frontmatter. SolidSpec generates these with the correct format so skills appear in Vibe's slash command list.

### 12. Prompt layering (personas + guardrails + context)

Every agent prompt is assembled in 4 layers: (1) Project context from `solidspec.toml` [context], (2) Role-based persona with output format + mission checklist, (3) Phase-specific instructions, (4) Anti-rationalization table + mandatory compliance checklist. This ensures agents get consistent guardrails regardless of which phase they're executing.

### 13. Agent subprocess timeout

Agent CLI invocation uses a `try_wait()` polling loop with a 300-second default timeout. If the agent process doesn't complete within the deadline, it's killed and the pipeline falls back to handoff mode. This prevents hung agent processes from blocking the entire pipeline indefinitely.

### 14. Delta spec brownfield workflow

Instead of creating a new feature from scratch for every change, SolidSpec supports lightweight change folders with delta specs (ADDED/MODIFIED/REMOVED requirements). The `archive` command merges deltas into the main spec and moves the change to `archive/`. This enables spec-driven development for existing (brownfield) systems.

### 15. DAG-based artifact status

The `solidspec status` command uses Kahn's algorithm topological sort to compute which artifacts are ready to create. It detects completion via filesystem scanning (glob-based) and shows a table of Blocked/Ready/Done states. Users can work on any ready artifact in any order — dependencies are enablers, not gates.

### 16. OpenCode skill system support

SolidSpec generates directory-based skills for OpenCode (`.opencode/skills/solidspec-specify/SKILL.md`) with the required `name:` + `description:` YAML frontmatter per OpenCode's skill discovery protocol. Skills are auto-discovered via the `skill` tool.

### 17. Schema-driven workflows

Workflows are defined in YAML schema files rather than hardcoded in Rust. Three built-in schemas (spec-driven, minimal, security-first) are shipped with the binary. Users can create project-local overrides in `.solidspec/workflows/<name>/schema.yaml`. Resolution follows a 3-level cascade (project-local → built-in → default).

## File Counts

| Category | Files | Tests |
|----------|-------|-------|
| CLI commands | 19 | 19 |
| Core domain | 16 | 130 |
| Agents | 8 | 58 |
| Templates | 2 | 22 |
| Presets | 4 | 28 |
| Extensions | 5 | 40 |
| Config | 1 | 9 |
| main.rs | 1 | — |
| **Total** | **56** | **306** |

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` + `clap_complete` | CLI parsing + shell completions |
| `serde` + `toml` + `serde_yaml` + `serde_json` | Config serialization |
| `tera` | Template rendering |
| `git2` | Git operations (libgit2 bindings) |
| `regex` | Spec parsing, feature numbering |
| `semver` | Version validation |
| `thiserror` + `anyhow` | Error handling |
| `console` | Colored output |
| `which` | CLI tool detection |
| `chrono` | Timestamps |
| `log` + `env_logger` | Logging |
