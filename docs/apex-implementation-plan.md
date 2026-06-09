# APEX Workflow — SolidSpec Integration Plan

**Version:** 2.0  
**Date:** 2026-06-09  
**Scope:** Integrate the APEX (Analyze-Plan-Execute-eXamine) methodology into SolidSpec as two new workflow variants that reuse every existing phase and replace only the `implement` handoff.

---

## 1. Problem Statement

SolidSpec today stops at the specification boundary. After `solidspec tasks` produces `tasks.md`, the `implement` phase is a manual handoff — the user opens their AI agent and works without structure. APEX provides exactly what's missing: a repeatable 10-stage (steps 00–09, with 00b sub-steps for branch/interactive handling) implementation workflow with parallel exploration agents, adversarial code review, test creation, and optional PR automation.

**Key insight:** APEX is a smarter `implement`. Every other phase (specify, clarify, plan, tasks, tests, analyze, review, ship, evidence) is **reused unchanged**. Integration means:

1. Two new built-in schemas — `apex-driven` and `intent-apex` — that are identical to `spec-driven` and `intent-driven` respectively, except `implement` is replaced by `apex`.
2. A `solidspec apex` CLI command that prepares the SolidSpec-enriched context and invokes the agent.
3. APEX skill files registered in agent directories via `solidspec init` / `solidspec upgrade`.
4. `pipeline.rs` extended with two new phase-list constants and two new `filter_phases` branches — all existing branches unchanged.

---

## 2. Architecture

### Workflow variants

```
Workflow              Phases
─────────────────     ──────────────────────────────────────────────────────────
spec-driven (SDD)     specify → clarify → plan → tasks → tests → implement
                        → analyze → review → ship
                      [UNCHANGED — default, stays as-is]

intent-driven (IDSD)  intent → specify → clarify → plan → tasks → tests
                        → implement → evidence → analyze → review → ship
                      [UNCHANGED — stays as-is]

apex-driven (NEW)     specify → clarify → plan → tasks → tests → apex
                        → analyze → review → ship
                      [= spec-driven with apex replacing implement]

intent-apex (NEW)     intent → specify → clarify → plan → tasks → tests
                        → apex → evidence → analyze → review → ship
                      [= intent-driven with apex replacing implement]
```

### What changes vs. what is reused

| Component | SDD | IDSD | APEX-driven | Intent-apex | Action |
|-----------|-----|------|-------------|-------------|--------|
| `specify`, `clarify`, `plan`, `tasks`, `tests` | ✓ | ✓ | **reused** | **reused** | No change |
| `analyze`, `review`, `ship` | ✓ | ✓ | **reused** | **reused** | No change |
| `evidence` | — | ✓ | — | **reused** | No change |
| `implement` | ✓ | ✓ | — | — | No change (kept in SDD/IDSD) |
| `apex` | — | — | NEW | NEW | New phase |
| `PHASES` constant | ✓ | — | — | — | No change |
| `PHASES_IDSD` constant | — | ✓ | — | — | No change |
| `PHASES_APEX` constant | — | — | NEW | — | New constant |
| `PHASES_APEX_IDSD` constant | — | — | — | NEW | New constant |
| `should_skip()` | ✓ | ✓ | **reused** + `"apex"` arm | **reused** + `"apex"` arm | Add one arm |
| `filter_phases()` | ✓ | ✓ | Add `"apex-driven"` branch | Add `"intent-apex"` branch | Extend |
| `schema::builtin` | ✓ | ✓ | Add `APEX_DRIVEN` | Add `INTENT_APEX` | Extend |
| `agents::registry::COMMANDS` | ✓ | ✓ | Add `"apex"` entry | Add `"apex"` entry | Add one row |

### Integration diagram

```
┌─────────────────── SolidSpec workflow (apex-driven) ─────────────────────┐
│                                                                            │
│  specify → clarify → plan → tasks → tests                                 │
│                                  ↓           (all phases identical to SDD)│
│                              ┌───┴────────────────────┐                   │
│                              │  solidspec apex 001 -a  │  ← new CLI cmd   │
│                              └───┬────────────────────┘                   │
│                                  │ builds context from spec.md/plan.md/   │
│                                  │ tasks.md; inlines into agent prompt    │
│                                  ↓                                        │
│                         ┌────────────────┐                                │
│                         │  APEX Skill    │  ← registered at solidspec init│
│                         │  /apex -a -s   │                                │
│                         │  00-init       │                                │
│                         │  01-analyze ◄── spec + plan + tasks injected   │
│                         │  02-plan       │                                │
│                         │  03-execute ◄── tasks.md updated live          │
│                         │  04-validate   │                                │
│                         │  [05-examine]  │                                │
│                         │  [07-tests]    │                                │
│                         │  [09-finish]   │                                │
│                         └────────────────┘                                │
│                                  ↓                                        │
│                    (--sync) tasks.md checkboxes updated                   │
│                                  ↓                                        │
│         analyze → review → ship  (identical to SDD)                      │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. File Layout (New & Changed)

```
src/
  cli/
    apex.rs               NEW  — `solidspec apex` command
    mod.rs                MOD  — add Apex variant to Commands enum + dispatch
  core/
    apex.rs               NEW  — skill extraction, context builder, task sync
    mod.rs                MOD  — add `pub mod apex;`
  agents/
    registry.rs           MOD  — add "apex" to COMMANDS; add register_apex_skill()

schemas/
  apex-driven/
    schema.yaml           NEW  — spec-driven with apex instead of implement
  intent-apex/
    schema.yaml           NEW  — intent-driven with apex instead of implement

templates/
  apex/                   NEW  — embedded APEX assets (via include_str!/include_bytes!)
    SKILL.md
    steps/
      step-00-init.md
      step-00b-branch.md
      step-00b-economy.md
      step-00b-interactive.md
      step-01-analyze.md
      step-02-plan.md
      step-03-execute.md
      step-04-validate.md
      step-05-examine.md
      step-06-resolve.md
      step-07-tests.md
      step-08-run-tests.md
      step-09-finish.md
    templates/
      00-context.md
      01-analyze.md … 09-finish.md
      step-complete.md
      solidspec-context.md  NEW — SolidSpec preamble template
    scripts/
      setup-templates.sh
      update-progress.sh

tests/
  apex.rs                 NEW  — integration tests
```

**`pipeline.rs`** — changes limited to:
- Two new constants (`PHASES_APEX`, `PHASES_APEX_IDSD`)
- One new arm in `should_skip()` (`"apex"`)
- One new arm in `phase_type()` (`"apex"`)
- Two new branches in `filter_phases()` (handles `"apex-driven"` and `"intent-apex"`)

**`schema.rs`** — changes limited to:
- Two new `include_str!` constants in `builtin` module
- Two new match arms in `builtin::by_name()`
- Two new names in `builtin::names()`

All other existing files are **unchanged**.

---

## 4. Implementation Phases

### Phase 1 — New Schemas (schemas/)

**Goal:** Add `apex-driven` and `intent-apex` schemas — the only structural difference from existing schemas is `apex` in place of `implement`.

**1.1 Create `schemas/apex-driven/schema.yaml`:**

```yaml
# Schema: apex-driven
# SDD workflow with APEX replacing the manual implement handoff.
# All other artifacts are identical to spec-driven.
name: apex-driven
version: "1.0"
description: "APEX-enhanced SDD workflow — APEX replaces the implement handoff"

artifacts:
  - id: spec
    generates: ["spec.md"]
    requires: []
    instruction: "Create a feature specification with user stories, functional requirements (FR-### format), key entities, success criteria, and edge cases."
    template: "spec-template.md"

  - id: clarify
    generates: ["spec.md"]
    requires: ["spec"]
    instruction: "Find and resolve all [NEEDS CLARIFICATION] markers in spec.md. Propose concrete resolutions. Update spec.md with the resolved content."
    template: null

  - id: plan
    generates: ["plan.md"]
    requires: ["spec"]
    instruction: "Create an architecture plan: decisions with rationale, project structure, data model, technology stack, contracts, and constitution compliance check."
    template: "plan-template.md"

  - id: tasks
    generates: ["tasks.md"]
    requires: ["spec", "plan"]
    instruction: "Generate a phased task breakdown (Setup → Foundational → User Stories → Polish). Mark parallel-safe tasks with [P]. Link to user stories with [US#]. Each task independently completable."
    template: "tasks-template.md"

  - id: tests
    generates: ["tests/"]
    requires: ["spec"]
    instruction: "Generate test scaffolds from acceptance scenarios. One test file per user story. Auto-detect framework. Cover edge cases from spec."
    template: null

  - id: apex
    generates: ["apex/"]
    requires: ["tasks"]
    instruction: "Run 'solidspec apex' to launch the APEX implementation workflow. APEX provides structured Analyze-Plan-Execute-eXamine methodology with parallel exploration agents, adversarial code review, and optional PR automation. SolidSpec injects spec.md + plan.md + tasks.md as pre-loaded context."
    template: null

  - id: analyze
    generates: ["analysis-report.md"]
    requires: ["spec"]
    instruction: "Validate cross-artifact consistency. Check requirement traceability (FR-### → plan → tasks). Entity coverage. Constitution compliance. Output findings by severity."
    template: null

  - id: review
    generates: ["review-report.md"]
    requires: ["spec"]
    instruction: "Perform comprehensive spec quality review. Check placeholders, ambiguous language, requirement quality, acceptance scenario coverage, and security heuristics. Score dimensions 0-10."
    template: null

  - id: ship
    generates: ["ship-report.md"]
    requires: ["analyze", "review"]
    instruction: "Run 'solidspec ship' to execute parallel fan-out review (code, security, tests, performance) and produce a SHIP/HOLD decision."
    template: null
```

**1.2 Create `schemas/intent-apex/schema.yaml`:**

Same as `intent-driven` but with the `implement` artifact replaced by the same `apex` artifact above. The `evidence` artifact keeps its `requires: ["tests", "apex"]` dependency (was `["tests", "implement"]`).

**1.3 Register both schemas in `src/core/schema.rs`:**

```rust
pub mod builtin {
    pub const SPEC_DRIVEN: &str = include_str!("../../schemas/spec-driven/schema.yaml");
    pub const MINIMAL: &str = include_str!("../../schemas/minimal/schema.yaml");
    pub const SECURITY_FIRST: &str = include_str!("../../schemas/security-first/schema.yaml");
    pub const INTENT_DRIVEN: &str = include_str!("../../schemas/intent-driven/schema.yaml");
    // NEW:
    pub const APEX_DRIVEN: &str = include_str!("../../schemas/apex-driven/schema.yaml");
    pub const INTENT_APEX: &str = include_str!("../../schemas/intent-apex/schema.yaml");

    pub fn names() -> Vec<&'static str> {
        vec!["spec-driven", "minimal", "security-first", "intent-driven",
             "apex-driven", "intent-apex"]   // add two
    }

    pub fn by_name(name: &str) -> Option<&'static str> {
        match name {
            "spec-driven"    => Some(SPEC_DRIVEN),
            "minimal"        => Some(MINIMAL),
            "security-first" => Some(SECURITY_FIRST),
            "intent-driven"  => Some(INTENT_DRIVEN),
            "apex-driven"    => Some(APEX_DRIVEN),   // add
            "intent-apex"    => Some(INTENT_APEX),   // add
            _                => None,
        }
    }
}
```

**Acceptance criteria:**
- [ ] AC1: `solidspec status --schema apex-driven` resolves the schema without error
- [ ] AC2: `solidspec status --schema intent-apex` resolves the schema without error
- [ ] AC3: `apex-driven` schema has 9 artifacts; `intent-apex` has 11
- [ ] AC4: Both schemas parse to valid `ArtifactGraph` (no cycles, all deps exist)

---

### Phase 2 — Pipeline Extension (core/pipeline.rs)

**Goal:** Add APEX phase lists and extend skip/type/filter logic. All existing `PHASES`, `PHASES_IDSD`, and their branches are untouched.

**2.1 Add new phase-list constants:**

```rust
/// Pipeline phase names for the APEX-driven workflow.
/// Identical to PHASES except `apex` replaces `implement`.
pub const PHASES_APEX: &[&str] = &[
    "specify", "clarify", "plan", "tasks", "tests", "apex",
    "analyze", "review",
];

/// Pipeline phase names for the Intent-APEX workflow.
/// Identical to PHASES_IDSD except `apex` replaces `implement`.
pub const PHASES_APEX_IDSD: &[&str] = &[
    "intent", "specify", "clarify", "plan", "tasks", "tests", "apex",
    "evidence", "analyze", "review",
];
```

**2.2 Add `"apex"` arm to `should_skip()`:**

```rust
"apex" => {
    // Skip only when APEX fully completed: 09-finish.md exists inside
    // a run subdirectory of feature_dir/apex/.
    // Requires APEX to have been run with --save and --output-dir pointing here.
    // Without --save, apex never auto-skips (safe default: always offer to run).
    let apex_dir = feature_dir.join("apex");
    if !apex_dir.exists() {
        return false;
    }
    std::fs::read_dir(&apex_dir)
        .ok()
        .map(|d| {
            d.filter_map(|e| e.ok())
             .filter(|e| e.path().is_dir())
             .any(|run_dir| run_dir.path().join("09-finish.md").exists())
        })
        .unwrap_or(false)
}
```

**2.3 Add `"apex"` arm to `phase_type()`:**

```rust
"implement" | "apex" => PhaseType::Handoff,
```

**2.4 Extend `filter_phases()`** to pick the right constant per schema:

```rust
pub fn filter_phases(
    schema: &str,
    from: Option<&str>,
    to: Option<&str>,
) -> Result<Vec<&'static str>> {
    let all: &[&str] = match schema {
        "intent-driven" => PHASES_IDSD,
        "apex-driven"   => PHASES_APEX,
        "intent-apex"   => PHASES_APEX_IDSD,
        _               => PHASES,   // default: spec-driven, minimal, security-first, custom
    };
    // ... rest of function unchanged
}
```

**Acceptance criteria:**
- [ ] AC5: `filter_phases("apex-driven", None, None)` returns 8 phases with `"apex"` at position 5
- [ ] AC6: `filter_phases("intent-apex", None, None)` returns 10 phases with `"apex"` at position 6
- [ ] AC7: `should_skip("apex", feature_dir, false)` returns `false` when no `09-finish.md` exists
- [ ] AC8: `should_skip("apex", feature_dir, false)` returns `true` when any run subdir has `09-finish.md`
- [ ] AC9: `phase_type("apex")` returns `PhaseType::Handoff`
- [ ] AC10: Existing `filter_phases("spec-driven", ...)` and `filter_phases("intent-driven", ...)` unchanged

---

### Phase 3 — Embed APEX Assets (core/apex.rs)

**Goal:** Make APEX skill files part of the SolidSpec binary. Provide context-building and task-sync utilities.

**3.1 Create `src/core/apex.rs`:**

```rust
use std::path::{Path, PathBuf};
use anyhow::Result;

// ── Embedded APEX skill files ────────────────────────────────────────────────
pub mod skill_files {
    pub const SKILL_MD: &str =
        include_str!("../../templates/apex/SKILL.md");
    pub const STEP_00_INIT: &str =
        include_str!("../../templates/apex/steps/step-00-init.md");
    pub const STEP_00B_BRANCH: &str =
        include_str!("../../templates/apex/steps/step-00b-branch.md");
    pub const STEP_00B_INTERACTIVE: &str =
        include_str!("../../templates/apex/steps/step-00b-interactive.md");
    pub const STEP_01_ANALYZE: &str =
        include_str!("../../templates/apex/steps/step-01-analyze.md");
    pub const STEP_02_PLAN: &str =
        include_str!("../../templates/apex/steps/step-02-plan.md");
    pub const STEP_03_EXECUTE: &str =
        include_str!("../../templates/apex/steps/step-03-execute.md");
    pub const STEP_04_VALIDATE: &str =
        include_str!("../../templates/apex/steps/step-04-validate.md");
    pub const STEP_05_EXAMINE: &str =
        include_str!("../../templates/apex/steps/step-05-examine.md");
    pub const STEP_06_RESOLVE: &str =
        include_str!("../../templates/apex/steps/step-06-resolve.md");
    pub const STEP_07_TESTS: &str =
        include_str!("../../templates/apex/steps/step-07-tests.md");
    pub const STEP_08_RUN_TESTS: &str =
        include_str!("../../templates/apex/steps/step-08-run-tests.md");
    pub const STEP_09_FINISH: &str =
        include_str!("../../templates/apex/steps/step-09-finish.md");
    pub const SOLIDSPEC_CONTEXT_MD: &str =
        include_str!("../../templates/apex/templates/solidspec-context.md");
    pub const SETUP_TEMPLATES_SH: &[u8] =
        include_bytes!("../../templates/apex/scripts/setup-templates.sh");
    pub const UPDATE_PROGRESS_SH: &[u8] =
        include_bytes!("../../templates/apex/scripts/update-progress.sh");
}

/// Copy all APEX skill files to target_dir, preserving steps/templates/scripts.
/// For Claude Code: target_dir = .claude/commands/apex/
/// For Vibe/Kimi/OpenCode: target_dir = .vibe/skills/apex/ etc.
/// NOTE: setup-templates.sh takes feature_name (slug without number prefix,
///       e.g. "auth-system" not "001-auth-system") as its first argument;
///       the script generates task_id internally.
pub fn extract_skill(target_dir: &Path) -> Result<()>

/// Build the SolidSpec-enriched context preamble from feature artifacts.
/// Extracts:
///   - Lines under `## Functional Requirements` (up to next `##`) from spec.md
///   - Lines under `## User Scenarios` (up to next `##`) from spec.md
///   - First 60 lines of plan.md
///   - All `- [ ] T###` lines from tasks.md (pending tasks)
/// Missing files produce `[not yet generated]` sections.
/// Total output is hard-capped at 16 KB.
pub fn build_solidspec_context(feature_dir: &Path, feature_id: &str) -> Result<String>

/// Parse the APEX execute log and mark completed tasks in tasks.md.
/// Detection patterns:
///   ✓ T### …          (completion marker with task ID)
///   - [x] T### …      (checkbox completion)
///   ### ✓ T###: …     (section header completion)
/// Returns how many tasks were found vs. marked done.
pub fn sync_tasks_from_apex_log(apex_log: &Path, tasks_md: &Path) -> Result<SyncReport>

pub struct SyncReport {
    pub tasks_found: usize,
    pub tasks_marked_done: usize,
}
```

**3.2 Copy APEX source files from `docs/apex/` into `templates/apex/`** and add `solidspec-context.md`:

```markdown
<!-- templates/apex/templates/solidspec-context.md -->
# SolidSpec Feature Context

**Feature:** {{feature_id}}
**Spec:** specs/{{feature_id}}/spec.md
**Plan:** specs/{{feature_id}}/plan.md
**Tasks:** specs/{{feature_id}}/tasks.md

## Functional Requirements (from spec.md)

{{spec_requirements_section}}

## User Scenarios (from spec.md)

{{spec_user_stories}}

## Architecture Plan (from plan.md — first 60 lines)

{{plan_summary_section}}

## Pending Tasks (from tasks.md)

{{pending_tasks}}
({{pending_tasks_count}} pending / {{completed_tasks_count}} done)

---
_Injected by `solidspec apex`. APEX analyze phase should treat this as
pre-loaded discovery — focus analysis on the implementation side
(existing code, patterns, dependencies) rather than re-analyzing the spec._
```

**3.3 Register `pub mod apex;` in `src/core/mod.rs`.**

**Acceptance criteria:**
- [ ] AC11: `extract_skill()` writes all APEX files to `target_dir` preserving `steps/`, `templates/`, `scripts/` structure
- [ ] AC12: `build_solidspec_context()` extracts FR-### lines from spec.md, first 60 lines of plan.md, and all `- [ ] T###` lines from tasks.md
- [ ] AC13: `sync_tasks_from_apex_log()` identifies `✓ T###` and `- [x] T###` patterns in the execute log and marks matching tasks in tasks.md
- [ ] AC14: Missing files produce `[not yet generated]` sections, not errors
- [ ] AC15: Context output stays under 16 KB for a typical 20-task feature

---

### Phase 4 — Agent Skill Registration (agents/registry.rs)

**Goal:** Register the APEX slash command alongside existing SolidSpec commands, and install the full APEX skill directory for agents that support it.

**4.1 Add `"apex"` to the `COMMANDS` constant** (existing list in `registry.rs`):

```rust
const COMMANDS: &[(&str, &str)] = &[
    ("specify",   "Create a new feature specification"),
    ("clarify",   "Resolve ambiguities in a specification"),
    ("plan",      "Generate an architecture plan from a specification"),
    ("tasks",     "Generate a story-driven task breakdown from the plan"),
    ("implement", "Execute tasks from the task breakdown"),
    ("tests",     "Generate test scaffolds from acceptance scenarios"),
    ("analyze",   "Validate cross-artifact consistency"),
    ("review",    "Review spec quality with preflight heuristics"),
    ("checklist", "Generate a quality validation checklist"),
    // NEW:
    ("apex",      "Launch the APEX implementation workflow (Analyze-Plan-Execute-eXamine)"),
];
```

This gives every agent a `/solidspec-apex [feature-id]` slash command via the existing `register_commands()` loop — no loop changes needed.

**4.2 Add `register_apex_skill()` for directory-based agents** that support full skill directories:

```rust
/// Agent skill directory mapping for APEX full-skill installation.
/// Claude Code reads slash-commands from .claude/commands/; a subdirectory
/// named "apex" with SKILL.md registers the /apex command.
fn apex_skill_dir(agent_id: &str, project_root: &Path) -> Option<PathBuf> {
    match agent_id {
        "claude"    => Some(project_root.join(".claude/commands/apex")),
        "kimi"      => Some(project_root.join(".kimi/skills/apex")),
        "vibe"      => Some(project_root.join(".vibe/skills/apex")),
        "opencode"  => Some(project_root.join(".opencode/skills/apex")),
        _           => None,   // agent does not support skill directories
    }
}

/// Install the full APEX skill (SKILL.md + steps/ + templates/ + scripts/)
/// for a single agent. Returns Ok(false) if the agent does not support skills.
pub fn register_apex_skill(agent_id: &str, project_root: &Path) -> Result<bool> {
    match apex_skill_dir(agent_id, project_root) {
        Some(dir) => {
            core::apex::extract_skill(&dir)?;
            Ok(true)
        }
        None => Ok(false),   // silently skip unsupported agents
    }
}
```

**4.3 Call `register_apex_skill` in `cli/init.rs`'s agent registration loop** (after existing `register_commands` call for each detected agent). Skip agents returning `Ok(false)` silently.

**4.4 Call `register_apex_skill` in `solidspec upgrade`** so APEX files are refreshed.

**Acceptance criteria:**
- [ ] AC16: After `solidspec init --here` with `.claude/` present, `.claude/commands/apex/SKILL.md` exists
- [ ] AC17: `.claude/commands/apex/steps/`, `.claude/commands/apex/templates/`, `.claude/commands/apex/scripts/` created
- [ ] AC18: After `solidspec init --here` with `.kimi/` present, `.kimi/skills/apex/SKILL.md` exists
- [ ] AC19: All agents (including those without skill-dir support) receive a `/solidspec-apex` slash command via the existing `COMMANDS` path
- [ ] AC20: `solidspec upgrade` overwrites APEX files with the embedded version
- [ ] AC21: Agents without skill-dir support are silently skipped (no error)

---

### Phase 5 — CLI Command `solidspec apex` (cli/apex.rs)

**Goal:** Prepare the SolidSpec context bridge and optionally invoke the agent.

**Command signature:**

```
solidspec apex [feature-id] [flags]
   [feature-id]          Feature to implement (auto-detected if omitted)
   --auto / -a           Pass -a to APEX (autonomous mode)
   --examine / -x        Pass -x to APEX (adversarial review)
   --save / -s           Pass -s to APEX (save step outputs)
   --test / -t           Pass -t to APEX (include test steps 07–08)
   --economy / -e        Pass -e to APEX (no subagents, fewer tokens)
   --branch / -b         Pass -b to APEX (create/verify git branch first)
   --pr                  Pass -pr to APEX (create PR at step 09)
   --resume / -r         Pass -r to APEX (resume from last saved step)
   --interactive / -i    Pass -i to APEX (interactive flag selection at start)
   --no-agent            Prepare context only; print invocation without calling agent
   --dry-run             Print invocation and context size without writing any file
   --sync                After APEX, sync task completion back to tasks.md
```

**Execution flow in `cli/apex.rs::run()`:**

```
1.  Resolve feature_id and feature_dir via feature::resolve_feature() (reused)
2.  Verify spec.md exists; bail with descriptive error if not
3.  Warn if tasks.md missing (APEX works best with tasks)
4.  Load RootConfig for default_agent (reused)
5.  Build context string via core::apex::build_solidspec_context()
6.  If dry_run: print context size and invocation, exit 0
7.  Write context to project_root/.solidspec/apex-context.md
8.  Construct agent prompt:
      "/apex [flags] implement feature: {feature_slug}\n\n{context_content}"
    (Context is inlined into the prompt — APEX has no --context flag)
9.  If !no_agent: invoke agent via agents::invoker (reused)
10. If sync: run core::apex::sync_tasks_from_apex_log()
    - Searches .claude/output/apex/ for the most recent run matching feature_slug
11. Print summary
```

**Note on `setup-templates.sh` first argument:** `setup-templates.sh` takes `feature_name` as its first argument — this is the slug *without* the leading number (e.g. `auth-system`, not `001-auth-system`). The script generates `task_id` internally from the task description. SolidSpec strips the `NNN-` prefix when constructing this argument.

**Dry-run output example:**

```
APEX Setup: 001-auth-system

  Workflow:   apex-driven (or auto-detected)
  Feature:    001-auth-system
  Spec:       specs/001-auth-system/spec.md      ✓
  Plan:       specs/001-auth-system/plan.md      ✓
  Tasks:      specs/001-auth-system/tasks.md     ✓  (12 pending)
  Agent:      claude
  Output:     .claude/output/apex/auth-system/  (APEX native; override with --save + pipeline --output-dir)

  Context: 847 bytes (3 requirement lines, 12 pending tasks, 60 plan lines)

  Prompt to run in your agent:
  /apex -a -s implement feature: auth-system

  [SolidSpec context would be appended inline]
  Context also saved to: .solidspec/apex-context.md
```

**Tasks:**

5.1 Create `src/cli/apex.rs` implementing the flow above.

5.2 Add `Apex { ... }` variant to `Commands` enum in `src/cli/mod.rs`.

5.3 Add dispatch arm in `src/cli/mod.rs::run()`.

**Acceptance criteria:**
- [ ] AC22: `solidspec apex --no-agent` exits 0, writes `.solidspec/apex-context.md`, prints the invocation string
- [ ] AC23: `solidspec apex --dry-run` exits 0, prints stats, writes no files
- [ ] AC24: `solidspec apex` (no feature-id) auto-detects feature via `feature::resolve_feature()` (reused unchanged)
- [ ] AC25: Fails with descriptive error when no spec.md exists
- [ ] AC26: `--sync` reads `.claude/output/apex/*/03-execute.md` (most recent) and updates tasks.md checkboxes

---

### Phase 6 — Task Sync (core::apex::sync_tasks_from_apex_log)

**Goal:** After APEX's execute phase, mark tasks complete in `tasks.md` so the DAG status stays accurate.

**Detection patterns in APEX execute log (`03-execute.md`):**

```
✓ T001 Setup project        → direct completion marker
- [x] T002 Add auth handler  → checkbox completion
### ✓ T003: JWT validation   → section header completion
```

**Algorithm:**

```
1. Read 03-execute.md
2. Find all T### references adjacent to ✓ or [x] markers
3. For each T### found, flip `- [ ] T###` → `- [x] T###` in tasks.md
4. Return SyncReport { tasks_found, tasks_marked_done }
```

**Multiple APEX runs:** when multiple run directories exist under `.claude/output/apex/`, pick the one with the most recent `03-execute.md` by mtime.

**Acceptance criteria:**
- [ ] AC27: Correctly marks T001–T005 done when they appear with ✓ in the execute log
- [ ] AC28: Tasks absent from the log are unchanged
- [ ] AC29: Sync is idempotent — running twice produces identical tasks.md

---

### Phase 7 — Pipeline Integration (cli/pipeline.rs)

**Goal:** `solidspec pipeline --schema apex-driven` uses the APEX workflow with zero special-casing in the pipeline runner itself — only `filter_phases()` and `should_skip()` change (already done in Phase 2).

**7.1 The pipeline runner dispatch for the `apex` phase** calls `cli::apex::run()` with `--no-agent` (context prep only), then presents the generated invocation string to the user as a Handoff — exactly the same pattern as `implement`.

**7.2 Verify `cli/pipeline.rs`'s phase dispatch loop** handles the `"apex"` phase name by calling `apex::run(...)`. No other changes to the pipeline runner.

**Acceptance criteria:**
- [ ] AC30: `solidspec pipeline 001 --schema apex-driven --only apex --dry-run` prints the apex phase without error
- [ ] AC31: `solidspec pipeline 001 --schema apex-driven --from tasks --to analyze` includes apex as the intermediate phase
- [ ] AC32: `solidspec pipeline 001 --schema intent-apex --from intent --to ship --dry-run` includes all 10 phases

---

### Phase 8 — Integration Tests (tests/apex.rs)

```rust
// T1: init registers apex skill for claude agent
fn apex_skill_registered_after_init()
// T2: init registers apex slash-command for all agents (via COMMANDS)
fn apex_slash_command_registered_for_all_agents()
// T3: apex --no-agent writes context file and prints invocation
fn apex_no_agent_writes_context_file()
// T4: apex --dry-run writes no files
fn apex_dry_run_writes_no_files()
// T5: apex fails without spec.md
fn apex_fails_without_spec_md()
// T6: apex auto-detects feature from latest specs/ dir
fn apex_auto_detects_feature()
// T7: apex --sync updates tasks.md from execute log
fn apex_sync_marks_completed_tasks()
// T8: context includes FR-### lines from spec.md
fn apex_context_includes_requirements()
// T9: context includes pending tasks from tasks.md
fn apex_context_includes_pending_tasks()
// T10: pipeline --schema apex-driven --dry-run shows apex phase
fn pipeline_apex_driven_dry_run()
// T11: pipeline --schema intent-apex --dry-run shows all 10 phases
fn pipeline_intent_apex_dry_run()
// T12: filter_phases apex-driven has apex not implement
fn filter_phases_apex_driven_correct()
// T13: should_skip apex returns false without 09-finish.md
fn should_skip_apex_false_without_finish()
// T14: should_skip apex returns true with 09-finish.md
fn should_skip_apex_true_with_finish()
// T15: status shows apex artifact in apex-driven schema
fn status_shows_apex_artifact()
// T16: sync is idempotent
fn apex_sync_idempotent()
```

---

## 5. Data Flow Summary

```
solidspec apex 001 -a -s -t
        │
        ├─ feature::resolve_feature("001")       [reused unchanged]
        │    → "001-auth-system"
        │
        ├─ core::apex::build_solidspec_context(feature_dir)
        │     reads spec.md  → FR-### lines + user scenarios
        │     reads plan.md  → first 60 lines
        │     reads tasks.md → all - [ ] T### lines
        │     → writes project_root/.solidspec/apex-context.md
        │
        ├─ agents::invoker::invoke_agent_with_prompt(  [reused unchanged]
        │     agent_id: "claude",
        │     prompt: "/apex -a -s -t implement feature: auth-system\n\n[context]",
        │     project_root, timeout
        │  )
        │
        └─ (if --sync) core::apex::sync_tasks_from_apex_log(
               apex_log: .claude/output/apex/auth-system/03-execute.md,
               tasks_md: specs/001-auth-system/tasks.md
           )
```

---

## 6. Non-Goals

- **SDD and IDSD workflows are not modified.** `spec-driven` and `intent-driven` schemas, their `PHASES` constants, and all associated CLI commands remain exactly as they are. Teams that prefer the manual implement handoff keep using those schemas.
- **APEX does not replace the SDD spec/plan/tasks phases.** Those are upstream of APEX and are reused identically.
- **No new agent format.** APEX SKILL.md is copied into existing agent directories; no new entry is added to `agents/config.rs`.
- **No forced integration.** All APEX CLI flags pass through to APEX without interpretation. SolidSpec only prepares context; it does not validate or constrain APEX's internal behavior.
- **No schema-level enforcement.** Teams may use `solidspec apex` standalone (without `--schema apex-driven`) or with any schema. The new schemas are opt-in conveniences.

---

## 7. Dependencies & Risks

| Risk | Mitigation |
|------|-----------|
| APEX SKILL.md format changes upstream | Pin embedded version in `templates/apex/`; `solidspec upgrade` refreshes it |
| Agent doesn't support SKILL.md format | `register_apex_skill()` returns `Ok(false)` and skips silently; slash command still registered via COMMANDS |
| Task sync false-positive matches | Require T-prefix + digits + space in the regex; unit-test against realistic APEX logs |
| Context too large for agent context window | Hard-cap at 16 KB; truncate plan.md section with `[truncated — full plan at specs/NNN/plan.md]` |
| `03-execute.md` not yet written when `--sync` runs | Check file existence first; emit clear `--sync requires APEX to have completed the execute phase` message |
| `filter_phases` hardcodes schema names | Pattern is consistent with existing `"intent-driven"` branch; acceptable for a small enum of named schemas |
| `evidence` dependency in `intent-apex` | Must update `requires: ["tests", "apex"]` (was `["tests", "implement"]`); schema test catches regressions |

---

## 8. Implementation Order & Effort

| Phase | Priority | Effort | Depends on |
|-------|----------|--------|------------|
| 1 — New schemas | P0 | 0.5 day | Nothing |
| 2 — Pipeline extension | P0 | 0.5 day | Phase 1 |
| 3 — Embed APEX assets (core/apex.rs) | P0 | 1 day | Nothing |
| 4 — Agent skill registration | P0 | 0.5 day | Phase 3 |
| 5 — CLI command | P0 | 1 day | Phases 2–4 |
| 6 — Task sync | P1 | 0.5 day | Phase 3 |
| 7 — Pipeline integration (dispatch) | P1 | 0.5 day | Phases 2, 5 |
| 8 — Integration tests | P1 | 1 day | Phases 1–7 |

**Total estimated effort:** 5.5 developer-days

**Recommended delivery order:** Phase 1 → 3 → 2 → 4 → 5 → 6 → 7 → 8

---

## 9. Acceptance Criteria Summary

| ID | Criterion |
|----|-----------|
| AC1 | `solidspec status --schema apex-driven` resolves schema |
| AC2 | `solidspec status --schema intent-apex` resolves schema |
| AC3 | `apex-driven` has 9 artifacts; `intent-apex` has 11 |
| AC4 | Both schemas produce valid `ArtifactGraph` |
| AC5 | `filter_phases("apex-driven")` returns 8 phases with `apex` at position 5 |
| AC6 | `filter_phases("intent-apex")` returns 10 phases with `apex` at position 6 |
| AC7 | `should_skip("apex")` returns `false` without `09-finish.md` |
| AC8 | `should_skip("apex")` returns `true` with `09-finish.md` in a run subdir |
| AC9 | `phase_type("apex")` returns `Handoff` |
| AC10 | Existing `filter_phases("spec-driven")` and `filter_phases("intent-driven")` unchanged |
| AC11 | `extract_skill()` writes all APEX files preserving `steps/`, `templates/`, `scripts/` |
| AC12 | `build_solidspec_context()` extracts FR-### + first 60 lines of plan.md + `- [ ] T###` tasks |
| AC13 | `sync_tasks_from_apex_log()` matches `✓ T###` / `- [x] T###` in execute log |
| AC14 | Missing files produce `[not yet generated]` sections gracefully |
| AC15 | Context output stays under 16 KB for a typical 20-task feature |
| AC16 | `solidspec init` registers `.claude/commands/apex/SKILL.md` for Claude Code |
| AC17 | `.claude/commands/apex/steps/`, `templates/`, `scripts/` created |
| AC18 | `solidspec init` registers `.kimi/skills/apex/SKILL.md` for Kimi |
| AC19 | All agents receive `/solidspec-apex` slash command via `COMMANDS` |
| AC20 | `solidspec upgrade` overwrites APEX files |
| AC21 | Unsupported agents silently skipped |
| AC22 | `solidspec apex --no-agent` writes context file and prints invocation |
| AC23 | `solidspec apex --dry-run` prints stats and writes no files |
| AC24 | Feature auto-detected when feature-id omitted |
| AC25 | Fails with descriptive error when spec.md missing |
| AC26 | `--sync` reads `.claude/output/apex/*/03-execute.md` and updates tasks.md |
| AC27 | Sync marks T001–T005 done when they appear with ✓ in the execute log |
| AC28 | Tasks absent from log are unchanged |
| AC29 | Sync is idempotent |
| AC30 | `pipeline --schema apex-driven --only apex --dry-run` succeeds |
| AC31 | `pipeline --schema apex-driven --from tasks --to analyze` includes apex |
| AC32 | `pipeline --schema intent-apex --dry-run` shows all 10 phases |

---

## 10. Usage Examples (Post-Implementation)

### 10.1 Basic: prepare context and run APEX interactively (SDD + APEX)

```bash
# Step 1: run SDD phases as normal
solidspec specify "User auth system"
solidspec plan 001
solidspec tasks 001

# Step 2: instead of solidspec implement, launch APEX
solidspec apex 001 --no-agent    # prepares context, prints invocation
# Then in your agent:
# /apex -a -s implement feature: auth-system
```

### 10.2 Full APEX pipeline with new schema

```bash
# Start a new feature using the apex-driven schema
solidspec pipeline --new "User auth system" --schema apex-driven
# Runs: specify → clarify → plan → tasks → tests → apex (handoff) → analyze → review → ship
# At the apex phase, you see the invocation and run it in your agent
```

### 10.3 IDSD + APEX combined

```bash
solidspec pipeline 001 --schema intent-apex
# Runs: intent → specify → clarify → plan → tasks → tests
#         → apex (handoff) → evidence → analyze → review → ship
```

### 10.4 Autonomous mode (all automated)

```bash
solidspec apex 001 -a -s -t -x   # autonomous + save + tests + examine
# Agent handles entire APEX workflow; outputs go to .claude/output/apex/auth-system/
```

### 10.5 Sync task completion after APEX

```bash
solidspec apex 001 --sync        # reads APEX execute log → updates tasks.md
solidspec status 001             # shows updated completion state in DAG
```

### 10.6 Economy mode (no subagents, fewer tokens)

```bash
solidspec apex 001 -e -a        # economy + autonomous
# /apex -e -a implement feature: auth-system
```

### 10.7 Resume interrupted APEX run

```bash
solidspec apex 001 -r -s        # resume from last saved step
# /apex -r -s implement feature: auth-system
```

### 10.8 CI/CD gate: full pipeline with APEX and ship gate

```bash
solidspec pipeline --new "User auth system" --schema apex-driven --auto
# specify → clarify → plan → tasks → tests → apex (auto) → analyze → review → ship
# ship produces SHIP/HOLD decision from parallel fan-out review
```
