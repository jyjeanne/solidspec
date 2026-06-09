# SolidSpec Architecture

## Overview

SolidSpec is a Rust CLI built with a layered architecture that separates concerns into distinct module trees. The CLI layer handles user interaction, the core layer contains domain logic, and specialized modules handle agent integration, templates, presets, and extensions.

Two workflow modes share the same infrastructure:

- **SDD** (Specification-Driven Development) — 8-phase pipeline, default schema
- **IDSD** (Intent-Driven Specification Development) — 10-phase pipeline, opt-in via `--schema intent-driven`. Adds intent capture (phase 0), evidence collection (phase 8), drift detection, and a full traceability graph (INT-XXX → FR-XXX → T-XXX → test file)

```
                        ┌──────────────┐
                        │    main.rs   │
                        │  (entrypoint)│
                        └──────┬───────┘
                               │
                        ┌──────▼───────┐
                        │   cli/mod    │  19 subcommands
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
| `intent.rs` | `intent` | **[IDSD]** Capture ICE model into `intent.md`; creates feature dir, writes intent template |
| `specify.rs` | `specify` | Create feature branch + spec from template; selects IDSD spec template when schema is `intent-driven` |
| `clarify.rs` | `clarify` | Identify markers, generate questions |
| `plan.rs` | `plan` | Generate plan + supporting docs, constitution checks; injects `intent_goal/constraints/evidence` vars in IDSD mode |
| `tasks.rs` | `tasks` | Generate phased task breakdown |
| `implement.rs` | `implement` | Parse tasks, fire hooks, list pending work |
| `tests_cmd.rs` | `tests` | Generate test scaffolds from acceptance scenarios |
| `evidence.rs` | `evidence` | **[IDSD]** Cross-reference evidence criteria against implemented test scaffolds; write `evidence-report.md`; optionally rewrite `intent.md` Status |
| `analyze.rs` | `analyze` | Run consistency analysis, print report; shows trace tree and intent coverage in IDSD mode |
| `checklist.rs` | `checklist` | Generate/append quality checklists |
| `review.rs` | `review` | Preflight spec quality review with dimension scoring (8 dimensions in IDSD mode) |
| `pipeline.rs` | `pipeline` | Multi-agent pipeline orchestrator (8 SDD / 10 IDSD phases) with agent CLI invocation |
| `change.rs` | `change` | Change-based workflow: propose, list, archive (delta specs) |
| `status.rs` | `status` | DAG-based artifact completion status; shows intent drift in IDSD mode |
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
| `intent_parser.rs` | **[IDSD]** Parse `intent.md` into `IntentSpec` (ICE model: goal, constraints, evidence, risks, open questions). `IntentStatus` enum (Draft/Active/Satisfied/Drifted). `EvidenceCriterion` and `IntentDrift` types |
| `artifact_graph.rs` | DAG engine: Kahn's topological sort, completion detection, state computation. **[IDSD]** `TraceGraph` / `TraceLink` / `TraceLinkType` — builds full `INT-XXX → FR-XXX → T-XXX → test_file` chain via `build_trace_graph()`; renders as ASCII tree via `format_tree()` |
| `schema.rs` | Workflow schema loading (YAML), 3-level resolution (project-local → built-in → default), 4 built-in schemas |
| `evidence.rs` | **[IDSD]** `EvidenceCriterionResult`, `EvidenceReport`, `collect_evidence()` (keyword-match evidence criteria against implemented test scaffolds), `update_intent_status()` (in-place rewrite of `intent.md` Status field), `format_evidence_report()` |
| `review.rs` | Preflight spec quality review, 8 dimension scoring (7 base + `IntentAlignment`), placeholder detection. **[IDSD]** `review_intent_alignment()` — scores FR-XXX traceability to evidence criteria; penalizes draft status |
| `change.rs` | Change-based workflow: delta spec parser (ADDED/MODIFIED/REMOVED), archive merge engine, change metadata |
| `constitution.rs` | Load constitution, parse gates, check plan compliance. **[IDSD]** `check_intent_constraints()` — validates plan against intent constraints |
| `task_generator.rs` | Generate `TaskList` from spec + plan, organize by phases |
| `test_generator.rs` | Extract Given/When/Then scenarios, detect framework, generate test scaffolds |
| `pipeline.rs` | Phase list constants (`PHASES` 8-phase SDD, `PHASES_IDSD` 10-phase), skip conditions, phase type (Auto/Handoff), filtering, log generation |
| `analyzer.rs` | Cross-artifact consistency validation, severity heuristics. **[IDSD]** `compute_drift()` — intent evidence vs implemented tests; `AnalysisReport` gains `trace_graph: Option<TraceGraph>` and `intent_coverage: Option<f64>`; orphaned-FR findings when tasks.md exists but doesn't reference a spec requirement |
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
| `mod.rs` | Tera rendering (autoescape disabled), embedded template constants. **[IDSD]** `INTENT_TEMPLATE`, `IDSD_SPEC_TEMPLATE`, `IDSD_PLAN_TEMPLATE` embedded via `include_str!` |
| `resolver.rs` | 4-layer resolution: overrides > presets > extensions > embedded |

**Resolution hierarchy:**
```
1. .solidspec/templates/overrides/    (project tweaks)
2. .solidspec/presets/<id>/templates/ (sorted by priority)
3. .solidspec/extensions/<id>/templates/
4. Embedded in binary (include_str!)
```

**IDSD templates** (in `templates/`):
- `intent-template.md` — ICE scaffold (Goal / Constraints / Evidence / Risks / Open Questions)
- `idsd/spec-template.md` — SDD spec template + `## Intent Reference` header
- `idsd/plan-template.md` — SDD plan template + `## Intent Reference` section with `{{ intent_goal }}`, `{{ intent_constraints }}`, `{{ intent_evidence }}`

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
| `mod.rs` | `RootConfig` (solidspec.toml), `PipelineConfig` (per-phase agent mapping, schema field), `ProjectInternalConfig`, `InitOptions`, project root finder |

---

## Data Flow

### SDD: Specify → Plan → Tasks Pipeline

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

### IDSD: Full 10-Phase Chain

```
solidspec intent "title"
      │
      ▼
┌─────────────┐
│   intent    │  Phase 0 (IDSD only)
│             │
│ - intent.md │  ICE model:
│   INT-XXX   │  Goal / Constraints / Evidence
└──────┬──────┘
       │
       ▼
┌─────────────┐   ┌─────────────┐   ┌─────────────┐
│   specify   │──>│    plan     │──>│    tasks    │
│  idsd/spec  │   │  idsd/plan  │   │   tasks.md  │
│  -template  │   │  +intent    │   │  [FR-001]   │
│  INT-001 ref│   │  reference  │   │  tags       │
└─────────────┘   └─────────────┘   └──────┬──────┘
                                           │
                                           ▼
┌─────────────┐                    ┌─────────────┐
│   evidence  │<───────────────────│  implement  │
│             │  Phase 8 (IDSD)    │  (handoff)  │
│ - evidence  │                    └─────────────┘
│   -report   │
│   .md       │
│ - per-      │
│   criterion │
│   status    │
│ - updates   │
│   intent    │
│   Status    │
└──────┬──────┘
       │
       ▼
┌─────────────┐   ┌─────────────┐
│   analyze   │──>│   review    │
│             │   │             │
│ - trace     │   │ - 8 dims    │
│   tree      │   │ - Intent    │
│ - drift     │   │   Alignment │
│ - orphaned  │   │ - score     │
│   FRs       │   └─────────────┘
│ - intent    │
│   coverage  │
└─────────────┘
```

### IDSD Traceability Graph Construction

```
build_trace_graph(feature_dir)
      │
      ├── Read intent.md  →  extract INT-XXX
      │
      ├── Read spec.md    →  extract FR-XXX → description
      │                      (FR_DEF_RE: **FR-001**: text)
      │
      ├── Read tasks.md   →  extract T-XXX lines
      │                      (TASK_LINE_RE: - [ ] T001 ... [FR-001])
      │                       dedup FR refs per task line
      │
      └── Scan tests/     →  for each .md/.ts/.py/.rs/.go file
                              find T\d+ patterns
                              (TASK_IN_TEST_RE: \bT(\d+)\b, normalised to T001)
      │
      ▼
Links:
  INT-XXX → FR-XXX  (IntentToRequirement)
  FR-XXX  → T-XXX   (RequirementToTask, one per [FR-XXX] tag in task line)
  T-XXX   → file    (TaskToTest, when file mentions T-number)

Orphaned FRs:
  FR-XXX present in spec but with zero RequirementToTask links
  → High finding in analyze (only when tasks.md exists)
```

### IDSD Intent Drift Computation

```
compute_drift(feature_dir)
      │
      ├── Parse intent.md  →  evidence criteria list
      │
      ├── Scan tests/       →  collect scaffold files
      │                         (.md, .ts, .py, .rs, .go)
      │
      ├── Check baseline:   →  if no file has STATUS: IMPLEMENTED
      │                         return IntentDrift { score: 0.0, unsatisfied: [] }
      │                         (baseline — not yet measurable)
      │
      └── For each criterion:
            keywords = words ≥ 5 chars from criterion text
            if keywords empty → short criterion, exact phrase match
            check: any keyword in implemented_words (word set from IMPLEMENTED tests)
            if not found → unsatisfied

score = unsatisfied_count / total_criteria * 100
  → 0%:  all covered (or baseline)
  → 30%+: High finding
  → 70%+: Critical finding

intent_coverage (in AnalysisReport):
  → None at baseline (has_implemented_tests == false)
  → Some(satisfaction_rate) once any test is IMPLEMENTED
  → Distinguishes "0% covered" from "baseline: not measured yet"
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

IDSD template selection (in `specify.rs` and `plan.rs`):
```rust
let (template_name, fallback) = if effective_schema == "intent-driven" {
    ("idsd/spec-template.md", IDSD_SPEC_TEMPLATE)
} else {
    ("spec-template.md", SPEC_TEMPLATE)
};
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
solidspec pipeline [--new "desc"] [--from X] [--to Y]
                  [--auto] [--no-agent] [--schema S]
        │
        ▼
┌─ Select phase list ─────────────────────────────────┐
│  schema == "intent-driven"                           │
│    → PHASES_IDSD: intent→specify→clarify→plan→tasks  │
│                   →tests→implement→evidence→analyze  │
│                   →review  (10 phases)               │
│  else                                                │
│    → PHASES: specify→clarify→plan→tasks→tests        │
│              →implement→analyze→review  (8 phases)   │
└─────────────────────┬───────────────────────────────┘
                      │
        ▼─────────────┘
┌─ Resolve feature ──────────────────────────────┐
│  --new → next_feature_number() + branch name    │
│  else  → 4-level feature resolution             │
└─────────────────────┬──────────────────────────┘
                      │
        ▼─────────────┘
┌─ Check agent availability ─────────────────────┐
│  For each phase, check if agent CLI is in PATH  │
│  → AllCli / Mixed / Disabled (--no-agent)       │
└─────────────────────┬──────────────────────────┘
                      │
        ▼─────────────┘
┌─ For each phase ───────────────────────────────────────────────┐
│                                                                 │
│  1. Resolve agent (per-phase config > default_agent)            │
│  2. Check skip condition (artifact exists? force?)              │
│  3. Execute phase:                                              │
│     ├── "intent"    → intent::run()    [IDSD phase 0]          │
│     ├── "specify"   → specify::run() or run_for_existing()     │
│     ├── "clarify"   → clarify::run()                           │
│     ├── "plan"      → plan::run(schema)  [injects intent vars] │
│     ├── "tasks"     → tasks::run()                              │
│     ├── "tests"     → tests_cmd::run()                          │
│     ├── "implement" → HANDOFF (always; user confirms)           │
│     ├── "evidence"  → evidence::run(false)  [IDSD phase 8]     │
│     ├── "analyze"   → analyze::run()                            │
│     └── "review"    → review::run()                             │
│  4. Invoke AI agent (unless --no-agent or Handoff):             │
│     ├── Auto phases → invoker::invoke_agent()                   │
│     └── NotAvailable/Failed → fall back to handoff              │
│  5. After intent/specify with --new → re-detect feature dir     │
│  6. Record PhaseResult (status, duration, output)               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
        │
        ▼
  Write specs/<feature>/pipeline-log.md
```

### IntentAlignment Review Dimension

```
preflight_review(feature_dir, project_root)
        │
        ├── [checks 1-8: placeholders, sections, ambiguity,
        │    requirement quality, scenario coverage,
        │    cross-references, task links, test coverage,
        │    security hints]
        │
        ├── Cap findings at MAX_FINDINGS = 100
        │
        └── review_intent_alignment(feature_dir, spec)
              │
              ├── intent.md absent → ([], 0.0)  score=0/10
              │
              ├── parse_intent fails → ([High: "could not be parsed"], 0.0)
              │
              └── intent.md present:
                    penalty = 0
                    if status == Draft → High finding, penalty += 3
                    for each FR-XXX:
                      keywords = words ≥ 5 chars; fallback ≥ 3 chars
                      if no evidence criterion contains keyword:
                        Medium finding, penalty += 1.5
                    score = max(0, 10 - penalty)
              │
              ▼
        DimensionScore { dimension: IntentAlignment, score, max: 10.0 }
        (appended after cap — never truncated)

format_review_report():
  → "## Intent Alignment" section (dedicated, not in general Findings)
  → "## Findings (N)" excludes IntentAlignment findings (shown above)
  → "No issues found" only if report.findings.is_empty() (all dims)
```

### Evidence Baseline vs Satisfied Distinction

```
At baseline (no tests marked STATUS: IMPLEMENTED):
  compute_drift()   → IntentDrift { score: 0.0, unsatisfied: [] }
  collect_evidence() → EvidenceReport { has_implemented_tests: false, satisfaction_rate: 0.0 }

In AnalysisReport:
  intent_drift    = Some(IntentDrift { score: 0.0 })
  intent_coverage = None  ← signals "not measurable yet"

In format_report():
  if drift.unsatisfied.is_empty():
    if intent_coverage.is_some() → "✓ all evidence criteria satisfied"
    else                         → "(baseline — no tests implemented yet)"

After first implementation (≥1 test is STATUS: IMPLEMENTED):
  intent_coverage = Some(satisfaction_rate)  ← measured value
```

### Agent CLI Invocation Flow

```
Pipeline Auto phase
        │
        ▼
┌─ invoker::build_phase_prompt() ────────────────┐
│  Generate phase-specific instructions           │
└─────────────────────┬──────────────────────────┘
                      │
        ▼─────────────┘
┌─ invoker::invoke_agent() ──────────────────────┐
│  1. Look up AgentConfig for agent ID            │
│  2. Check cli_binary is non-empty               │
│  3. Check binary exists in PATH (which::which)  │
│  4. Build Command:                              │
│     ├── claude: claude -p "prompt" --allowedTools│
│     ├── vibe:   vibe -p "prompt" --max-turns 25 │
│     ├── codex:  codex exec "prompt"             │
│     ├── kimi:   kimi --yolo "prompt"            │
│     └── others: {binary} {flag} "prompt"        │
│  5. Execute with current_dir = project_root     │
│  6. Return Success/NotAvailable/Failed          │
└─────────────────────────────────────────────────┘
```

### Change-Based Workflow (Delta Specs)

```
solidspec change propose "Add social login"
        │
        ▼
┌─ create_change() ───────────────────────────────────┐
│  1. Generate slug, create specs/<id>/changes/<slug>/ │
│  2. Write proposal.md + delta-spec.md + .change.yaml │
└──────────────────────┬──────────────────────────────┘
                       │ (user edits)
                       ▼
┌─ archive_change() ────────────────────────────────────┐
│  1. Parse delta-spec.md → DeltaSpec                   │
│  2. merge_deltas(): remove → modify → append          │
│  3. Write merged spec.md, move to archive/             │
└───────────────────────────────────────────────────────┘
```

### DAG Status Flow

```
solidspec status [feature-id] [--schema X]
        │
        ▼
┌─ schema::load_graph(name, root) ──────────────────┐
│  3-level: project-local → built-in → default       │
└────────────────────┬───────────────────────────────┘
                     ▼
┌─ graph.detect_completion(feature_dir) ────────────┐
│  Scan filesystem for generated artifacts           │
└────────────────────┬───────────────────────────────┘
                     ▼
┌─ graph.compute_states(completed) ─────────────────┐
│  Done / Ready / Blocked (missing deps)             │
└────────────────────┬───────────────────────────────┘
                     ▼
          Print status table (topological order)
          [IDSD] If intent.md present: show intent drift score
```

---

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

SolidSpec generates directory-based skills for OpenCode (`.opencode/skills/solidspec-specify/SKILL.md`) with the required `name:` + `description:` YAML frontmatter per OpenCode's skill discovery protocol.

### 17. Schema-driven workflows (4 built-in)

Workflows are defined in YAML schema files rather than hardcoded in Rust. Four built-in schemas ship with the binary (`spec-driven`, `minimal`, `security-first`, `intent-driven`). Users can create project-local overrides in `.solidspec/workflows/<name>/schema.yaml`. Resolution follows a 3-level cascade (project-local → built-in → default).

### 18. IDSD additive design

All IDSD features are strictly additive — they activate only when `schema == "intent-driven"` is explicitly set. The `spec-driven` schema YAML is never modified. SDD commands (`specify`, `plan`, `analyze`, `review`, `pipeline`) behave identically in SDD mode. IDSD-specific output (trace tree, drift score, intent coverage, IntentAlignment dimension) only appears when `intent.md` is present or the schema is `intent-driven`.

### 19. IntentAlignment appended after finding cap

The `IntentAlignment` `DimensionScore` is computed by `review_intent_alignment()` and pushed to `dimension_scores` after the 100-finding cap runs. This ensures the IntentAlignment score is always present in the dimension table regardless of how many base-dimension findings exist. IntentAlignment findings are displayed in their own `## Intent Alignment` section, not in the general `## Findings` section.

### 20. Orphaned-FR findings before finding cap, guarded by `has_tasks`

Orphaned-FR findings (FR in spec with no task referencing it) are added to `all_findings` before the `MAX_FINDINGS` cap so they participate in overflow counting. They are only added when `tasks.md` exists (`has_tasks == true`) — when `tasks.md` is absent, the existing "tasks.md missing" finding is sufficient and avoids generating N redundant "FR-XXX has no task" findings.

### 21. Drift vs coverage distinction

`compute_drift()` returns 0% drift at baseline (all tests are `STATUS: NOT IMPLEMENTED`) to avoid false alarms. `intent_coverage` returns `None` at baseline (not `Some(0.0)`) so the report can distinguish "baseline — not measured yet" from "0% coverage measured". `format_report()` uses `intent_coverage.is_some()` as the signal to print "✓ all satisfied" vs "(baseline — no tests implemented yet)" when drift is 0%.

### 22. Traceability regex asymmetry resolved

`TASK_LINE_RE` in `build_trace_graph` matches `T\d+` (any digit count) in `tasks.md`. `TASK_IN_TEST_RE` also matches `T\d+` with left-pad normalisation to T001 format (e.g., `T5` → `T005`) so test-file references to short task IDs can be matched to the zero-padded IDs stored in `task_texts`.

---

## File Counts

| Category | Files | Unit Tests |
|----------|-------|-----------|
| CLI commands | 21 | 21 |
| Core domain | 18 | 175 |
| Agents | 8 | 58 |
| Templates | 2 | 22 |
| Presets | 4 | 28 |
| Extensions | 5 | 40 |
| Config | 1 | 9 |
| main.rs | 1 | — |
| **Total src** | **60** | **353** |
| Integration tests | 7 files | 402 total |

Integration test files: `pipeline.rs`, `evidence.rs`, `traceability.rs`, `status.rs`, `change.rs`, `check.rs`, `completions.rs`

---

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` + `clap_complete` | CLI parsing + shell completions |
| `serde` + `toml` + `serde_yaml` + `serde_json` | Config serialization |
| `tera` | Template rendering |
| `git2` | Git operations (libgit2 bindings) |
| `regex` | Spec parsing, feature numbering, trace graph extraction |
| `semver` | Version validation |
| `thiserror` + `anyhow` | Error handling |
| `console` | Colored output |
| `which` | CLI tool detection |
| `chrono` | Timestamps |
| `log` + `env_logger` | Logging |
