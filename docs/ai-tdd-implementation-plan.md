# AI-Augmented TDD (AI-TDD) — SolidSpec Integration Plan

**Version:** 2.0
**Status:** Production Ready
**Date:** 2026-06-09
**Scope:** Add `tdd-driven` as a new SolidSpec workflow variant that implements the RED-GREEN-REFACTOR cycle with AI agents, reusing every existing phase except `tests` (replaced by `tdd-tests`) and adding `tdd-refactor` after `implement`.

---

## 1. Core Principles

### Tests Define Requirements
Requirements are expressed as executable tests. No implementation begins before failing tests exist.

### RED-GREEN-REFACTOR
```
RED    → Failing tests exist and prove the feature is not yet built
GREEN  → Minimum implementation makes all tests pass
REFACTOR → Code is improved without changing behavior
```

### Human Ownership
AI generates tests, code, and refactors. Humans own architecture, requirements, approvals, and production releases. Every handoff phase requires explicit human confirmation.

### Small Iterations
Maximum unit: one behavior, one endpoint, one user story. Avoid large AI-generated features in a single session.

---

## 2. Workflow Variants

```
Workflow          Phases
────────────────  ──────────────────────────────────────────────────────────
spec-driven       specify → clarify → plan → tasks → tests → implement
                    → analyze → review → ship
                  [UNCHANGED — default]

intent-driven     intent → specify → clarify → plan → tasks → tests
                    → implement → evidence → analyze → review → ship
                  [UNCHANGED]

apex-driven       specify → clarify → plan → tasks → tests → apex
                    → analyze → review → ship
                  [UNCHANGED]

intent-apex       intent → specify → clarify → plan → tasks → tests
                    → apex → evidence → analyze → review → ship
                  [UNCHANGED]

tdd-driven (NEW)  specify → clarify → plan → tasks → tdd-tests → implement
                    → tdd-refactor → analyze → review → ship
                  [= spec-driven with tdd-tests replacing tests
                   and tdd-refactor added after implement]
```

### What changes vs. what is reused

| Component | SDD | IDSD | APEX | TDD | Action |
|-----------|-----|------|------|-----|--------|
| `specify`, `clarify`, `plan`, `tasks` | ✓ | ✓ | ✓ | **reused** | No change |
| `analyze`, `review`, `ship` | ✓ | ✓ | ✓ | **reused** | No change |
| `tests` (scaffolds) | ✓ | ✓ | ✓ | — | Replaced by `tdd-tests` |
| `implement` | ✓ | ✓ | — | **reused** (GREEN) | No change |
| `tdd-tests` | — | — | — | NEW (RED) | New phase |
| `tdd-refactor` | — | — | — | NEW (REFACTOR) | New phase |
| `PHASES_TDD` constant | — | — | — | NEW | New constant |
| `should_skip()` | ✓ | ✓ | ✓ | Add 2 arms | Extend |
| `filter_phases()` | ✓ | ✓ | ✓ | Add `"tdd-driven"` branch | Extend |
| `phase_type()` | ✓ | ✓ | ✓ | Add `"tdd-tests" \| "tdd-refactor"` | Extend |
| `COMMANDS` in registry | ✓ | ✓ | ✓ | Add 2 entries | Extend |

---

## 3. Difference Between `tests` and `tdd-tests`

| Aspect | `tests` (existing) | `tdd-tests` (new) |
|--------|-------------------|-------------------|
| Output format | Markdown scaffolds (`.md`) | Executable test files (`.ts`, `.rs`, `.py`, etc.) |
| Status markers | `STATUS: NOT IMPLEMENTED` | Actual assertions that fail at runtime |
| Agent instruction | Fill in GIVEN/WHEN/THEN | Write real tests, run them, confirm RED |
| Skip condition | `tests/` directory exists | `tdd-red-report.md` exists |
| Phase type | `Auto` | `Handoff` (human reviews RED state) |
| Coverage | Not tracked | Minimum thresholds in spec |

---

## 4. File Layout

```
schemas/
  tdd-driven/
    schema.yaml           NEW

src/
  cli/
    tdd_tests.rs          NEW  — `solidspec tdd-tests` command
    tdd_refactor.rs       NEW  — `solidspec tdd-refactor` command
    mod.rs                MOD  — add TddTests/TddRefactor variants
  core/
    tdd.rs                NEW  — RED verification, coverage parsing, refactor report
    mod.rs                MOD  — add `pub mod tdd;`
  agents/
    registry.rs           MOD  — add tdd-tests/tdd-refactor to COMMANDS

tests/
  tdd.rs                  NEW  — integration tests
```

---

## 5. Artifact Definitions

### `tdd-tests` artifact
- **generates**: `["tests/", "tdd-red-report.md"]`
- **requires**: `["spec", "tasks"]`
- **skip condition**: `tdd-red-report.md` exists in feature dir
- **phase type**: `Handoff`
- **agent instruction**: Generate executable failing tests for all acceptance criteria. Run them to confirm RED state. Write `tdd-red-report.md` with: test count, failing count, framework used, coverage baseline.

### `tdd-refactor` artifact
- **generates**: `["tdd-refactor-report.md"]`
- **requires**: `["implement"]`
- **skip condition**: `tdd-refactor-report.md` exists in feature dir
- **phase type**: `Handoff`
- **agent instruction**: Refactor the implementation for readability and maintainability. All tests must remain green throughout. Write `tdd-refactor-report.md` with: changes made, before/after metrics (complexity, duplication), test run confirmation.

---

## 6. Implementation Phases

### Phase 1 — Schema (`schemas/tdd-driven/schema.yaml`)

10 artifacts: `spec`, `clarify`, `plan`, `tasks`, `tdd-tests`, `implement`, `tdd-refactor`, `analyze`, `review`, `ship`.

Key differences from `spec-driven`:
- `tdd-tests` replaces `tests` (different instruction, generates `tdd-red-report.md` too)
- `tdd-refactor` inserted between `implement` and `analyze`
- `tdd-refactor` requires `["implement"]`

Register in `src/core/schema.rs` (same pattern as `apex-driven`).

**Acceptance criteria:**
- AC1: `solidspec status --schema tdd-driven` resolves without error
- AC2: `tdd-driven` has 10 artifacts
- AC3: `tdd-tests` requires `["spec", "tasks"]`; `tdd-refactor` requires `["implement"]`
- AC4: Schema produces valid `ArtifactGraph` (no cycles)

---

### Phase 2 — Pipeline Extension (`src/core/pipeline.rs`)

**2.1 New constant:**
```rust
pub const PHASES_TDD: &[&str] = &[
    "specify", "clarify", "plan", "tasks", "tdd-tests", "implement",
    "tdd-refactor", "analyze", "review",
];
```

**2.2 New arms in `should_skip()`:**
```rust
"tdd-tests" => !feature_dir.join("tdd-red-report.md").exists(),
// returns true (skip) when report already exists
"tdd-refactor" => feature_dir.join("tdd-refactor-report.md").exists(),
```

Wait — `should_skip` returns `true` when the phase SHOULD be skipped.
```rust
"tdd-tests"    => force == false && feature_dir.join("tdd-red-report.md").exists(),
"tdd-refactor" => force == false && feature_dir.join("tdd-refactor-report.md").exists(),
```

**2.3 `phase_type()`:**
```rust
"implement" | "apex" | "tdd-tests" | "tdd-refactor" => PhaseType::Handoff,
```

**2.4 `filter_phases()`:**
```rust
"tdd-driven" => PHASES_TDD,
```

**Acceptance criteria:**
- AC5: `filter_phases("tdd-driven", None, None)` returns 9 phases with `tdd-tests` at position 4
- AC6: `phase_type("tdd-tests")` and `phase_type("tdd-refactor")` both return `Handoff`
- AC7: `should_skip("tdd-tests", dir, false)` returns false when `tdd-red-report.md` absent
- AC8: `should_skip("tdd-tests", dir, false)` returns true when `tdd-red-report.md` present
- AC9: `should_skip("tdd-refactor", dir, false)` returns true when `tdd-refactor-report.md` present
- AC10: All existing `filter_phases` branches unchanged

---

### Phase 3 — Core TDD Module (`src/core/tdd.rs`)

```rust
/// Generate the tdd-red-report.md template with acceptance criteria from spec.
pub fn scaffold_red_report(feature_dir: &Path, feature_id: &str) -> Result<String>

/// Generate the tdd-refactor-report.md template.
pub fn scaffold_refactor_report(feature_dir: &Path, feature_id: &str) -> Result<String>

/// Parse tdd-red-report.md — return (tests_found, tests_failing, framework).
pub fn parse_red_report(report_path: &Path) -> Result<RedReport>

pub struct RedReport {
    pub tests_found: usize,
    pub tests_failing: usize,
    pub framework: String,
}
```

The `scaffold_red_report` function:
- Reads acceptance criteria from `spec.md` (lines under `## Acceptance Criteria` or stories)
- Lists each criterion as an unchecked test item
- Includes coverage threshold table from the spec (or defaults: Unit 90%, Integration 80%)
- Writes instructions for the agent

**Acceptance criteria:**
- AC11: `scaffold_red_report` produces a markdown file with AC items from spec.md
- AC12: `parse_red_report` returns correct counts from a sample report
- AC13: Missing spec.md produces graceful `[not yet generated]` sections

---

### Phase 4 — CLI Commands

**`solidspec tdd-tests [feature_id]`** (`src/cli/tdd_tests.rs`):
1. Resolve feature
2. Verify `tasks.md` exists (prerequisite)
3. Call `tdd::scaffold_red_report()` → write `tdd-red-report.md` template
4. Print RED-phase instructions + agent invocation hint

**`solidspec tdd-refactor [feature_id]`** (`src/cli/tdd_refactor.rs`):
1. Resolve feature
2. Verify all tasks done in `tasks.md` (warn if pending tasks exist)
3. Call `tdd::scaffold_refactor_report()` → write `tdd-refactor-report.md` template
4. Print REFACTOR-phase instructions

Add `TddTests` and `TddRefactor` variants to `Commands` enum in `cli/mod.rs`.

**Acceptance criteria:**
- AC14: `solidspec tdd-tests 001` writes `tdd-red-report.md` and prints instructions
- AC15: `solidspec tdd-tests 001` fails with message when `tasks.md` missing
- AC16: `solidspec tdd-refactor 001` writes `tdd-refactor-report.md`
- AC17: `solidspec tdd-refactor 001` warns when pending tasks remain

---

### Phase 5 — Agent Slash Commands (`src/agents/registry.rs`)

Add two entries to `COMMANDS`:
```rust
("tdd-tests",   "Generate failing tests for the RED phase (AI-TDD workflow)"),
("tdd-refactor", "Refactor implementation while keeping tests green (REFACTOR phase)"),
```

**`tdd-tests` instruction body:**
```
RED Phase — AI-TDD Workflow

Feature: {arg}
Spec: specs/{arg}/spec.md  (acceptance criteria)
Tasks: specs/{arg}/tasks.md (scope)

Steps:
1. Read acceptance criteria from spec.md
2. For each criterion, write an executable failing test in the project's test framework
3. Run the tests — confirm they ALL FAIL (RED state)
4. If any test passes already, that criterion is already implemented — mark it
5. Write test count + failing count to tdd-red-report.md
6. Do NOT write any implementation code in this phase
```

**`tdd-refactor` instruction body:**
```
REFACTOR Phase — AI-TDD Workflow

Feature: {arg}
All tests must be GREEN before this phase.

Steps:
1. Run the test suite — confirm GREEN state before refactoring
2. Refactor for: clarity, reduced complexity, no duplication, clean naming
3. After each refactoring change, run tests again — must stay GREEN
4. Write tdd-refactor-report.md: changes made, complexity delta, test run evidence
```

**Acceptance criteria:**
- AC18: After `solidspec init` with `.claude/`, `.claude/commands/solidspec-tdd-tests.md` exists
- AC19: After `solidspec init` with `.claude/`, `.claude/commands/solidspec-tdd-refactor.md` exists
- AC20: Both command files contain phase-specific instructions (RED/REFACTOR keywords)

---

### Phase 6 — Pipeline Dispatch (`src/cli/pipeline.rs`)

Add two arms in `execute_phase()`:

**`"tdd-tests"`** — scaffolds report and hands off:
```
1. Call cli::tdd_tests::run(Some(feature_dir_name), ...)
2. Print: "→ Open {agent} and run: /solidspec-tdd-tests {feature_dir_name}"
3. Wait for user confirmation (Handoff)
```

**`"tdd-refactor"`** — scaffolds report and hands off:
```
1. Call cli::tdd_refactor::run(Some(feature_dir_name), ...)
2. Print: "→ Open {agent} and run: /solidspec-tdd-refactor {feature_dir_name}"
3. Wait for user confirmation (Handoff)
```

Add `"tdd-tests"` and `"tdd-refactor"` to `skip_reason()`.

Update `check_agent_availability()` to treat both as Handoffs.

**Acceptance criteria:**
- AC21: `pipeline --schema tdd-driven --dry-run` shows tdd-tests and tdd-refactor phases with `[HANDOFF]`
- AC22: `pipeline --schema tdd-driven --only tdd-tests --dry-run` succeeds
- AC23: `pipeline --schema tdd-driven --from tdd-tests --to tdd-refactor --dry-run` includes implement

---

### Phase 7 — Integration Tests (`tests/tdd.rs`)

```rust
// Schema
fn tdd_driven_schema_resolves()
fn tdd_driven_has_ten_artifacts()
fn tdd_tests_requires_spec_and_tasks()
fn tdd_refactor_requires_implement()

// CLI commands
fn tdd_tests_command_writes_red_report()
fn tdd_tests_fails_without_tasks_md()
fn tdd_refactor_command_writes_refactor_report()
fn tdd_refactor_warns_when_tasks_pending()

// Pipeline
fn pipeline_tdd_driven_dry_run_shows_tdd_phases()
fn pipeline_tdd_driven_dry_run_shows_handoff_for_tdd_tests()
fn pipeline_tdd_driven_dry_run_shows_handoff_for_tdd_refactor()
fn pipeline_tdd_driven_excludes_regular_tests_phase()
fn pipeline_tdd_driven_skips_tdd_tests_when_report_exists()
fn pipeline_tdd_driven_skips_tdd_refactor_when_report_exists()
fn pipeline_spec_driven_unchanged_by_tdd_addition()

// Slash commands
fn init_registers_tdd_tests_slash_command()
fn init_registers_tdd_refactor_slash_command()

// Status
fn status_shows_tdd_artifacts_in_tdd_schema()
```

---

## 7. AI Agent Roles (mapped to SolidSpec phases)

| AI-TDD Role | SolidSpec Phase | Slash Command |
|-------------|-----------------|---------------|
| AI Test Agent | `tdd-tests` | `/solidspec-tdd-tests` |
| AI Implementation Agent | `implement` | `/solidspec-implement` |
| AI Refactor Agent | `tdd-refactor` | `/solidspec-tdd-refactor` |
| AI Review Agent | `review` + `ship` | `/solidspec-review`, `/solidspec-ship` |

---

## 8. Coverage Requirements (in `tdd-red-report.md`)

Default thresholds written into the report template:

| Type | Threshold |
|------|-----------|
| Unit | 90% |
| Integration | 80% |
| Critical Services | 95% |
| Security Logic | 100% |

These are recorded in the report for agent guidance. SolidSpec does not enforce them programmatically — the CI/CD pipeline enforces them via coverage gates.

---

## 9. Definition of Done (per story)

A feature is complete in `tdd-driven` when:

```yaml
done:
  tdd_red_report_exists: true       # tdd-red-report.md present
  all_tasks_complete: true          # no - [ ] T### in tasks.md
  tdd_refactor_report_exists: true  # tdd-refactor-report.md present
  analysis_report_exists: true      # analysis-report.md present
  review_report_exists: true        # review-report.md present
  ship_decision: "SHIP"             # ship-report.md contains <!-- ship: true -->
```

---

## 10. Anti-Patterns (enforced by phase ordering)

| Anti-Pattern | SolidSpec Enforcement |
|---|---|
| Generate implementation without tests | `tdd-tests` must exist before `implement` (DAG dependency) |
| Skip RED phase | `should_skip("tdd-tests")` only skips when report already exists |
| Allow AI to modify requirements | `specify` and `clarify` are upstream of all implementation phases |
| Merge failing tests | `ship` requires `analyze` + `review`, which check consistency |
| Accept unreviewed AI code | `tdd-tests` and `tdd-refactor` are Handoff phases |
| Large feature generation | Tasks phase enforces small, independently-completable units |

---

## 11. Usage Examples

### Basic TDD flow
```bash
solidspec specify "User login endpoint"
solidspec plan 001
solidspec tasks 001

# RED phase: AI generates failing tests, human reviews
solidspec tdd-tests 001
# Open agent: /solidspec-tdd-tests 001
# Agent writes tests, runs them, confirms RED, writes tdd-red-report.md

# GREEN phase: AI implements to pass tests
solidspec implement 001  # existing command, reused unchanged
# /solidspec-implement 001

# REFACTOR phase: AI cleans up, human approves
solidspec tdd-refactor 001
# /solidspec-tdd-refactor 001

# Quality gates
solidspec analyze 001
solidspec review 001
solidspec ship 001
```

### Full TDD pipeline
```bash
solidspec pipeline --new "User login endpoint" --schema tdd-driven
# specify → clarify → plan → tasks → tdd-tests [HANDOFF]
#   → implement [HANDOFF] → tdd-refactor [HANDOFF]
#   → analyze → review → ship
```

### TDD starting from existing tasks
```bash
solidspec pipeline 001 --schema tdd-driven --from tdd-tests --to tdd-refactor
```

---

## 12. Implementation Order

| Phase | Effort | Depends on |
|-------|--------|------------|
| 1 — Schema | 0.5d | Nothing |
| 2 — Pipeline extension | 0.5d | Phase 1 |
| 3 — Core tdd.rs | 0.5d | Nothing |
| 4 — CLI commands | 1d | Phases 2–3 |
| 5 — Agent commands | 0.5d | Nothing |
| 6 — Pipeline dispatch | 0.5d | Phases 2, 4 |
| 7 — Integration tests | 1d | Phases 1–6 |

**Total: ~4.5 developer-days**
