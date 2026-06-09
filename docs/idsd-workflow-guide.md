# IDSD Workflow Guide

Intent-Driven Specification Development (IDSD) is an opt-in workflow layered on top of SolidSpec's standard SDD (Spec-Driven Development) pipeline. Instead of starting with a specification, you start with an **intent** — a concise statement of *why* a capability must exist, what boundaries it must respect, and how you will know it succeeded.

The rest of the pipeline (spec → plan → tasks → tests → implement → analyze → review) runs unchanged; IDSD adds **phase 0** (intent capture) and **phase 8** (evidence collection), and enriches the analysis and review outputs with traceability and drift metrics.

---

## When to choose IDSD over SDD

| Situation | Recommendation |
|-----------|---------------|
| Greenfield feature with uncertain scope | **IDSD** — intent anchors the "why" before the "what" |
| Brownfield change to an existing system | **SDD** with `solidspec change` for delta specs |
| Regulated feature requiring evidence of test coverage | **IDSD** — evidence-report satisfies audit requirements |
| Rapid prototype where requirements are known | **SDD** — fewer phases, lower overhead |
| Team wants to detect requirement drift over time | **IDSD** — drift score flags divergence automatically |

---

## Complete walkthrough: Task Manager App

This walkthrough implements a *Task Manager* feature from intent to review using the IDSD workflow.

### Prerequisites

```bash
# Verify SolidSpec is installed and a project is initialised
solidspec check
solidspec init my-app --here   # skip if already initialised
```

---

### Step 0 — Capture the intent

The intent describes **why** the capability must exist, not **how** it will work.

```bash
solidspec intent "Allow users to manage tasks so they never lose track of what needs to be done"
```

This creates `specs/001-allow-users-to/intent.md` with the ICE scaffold:

```markdown
# Intent: Allow users to manage tasks so they never lose track of what needs to be done

**Intent ID**: INT-001
**Feature**: 001-allow-users-to
**Created**: 2026-06-03
**Status**: draft

## Goal

Allow users to manage tasks so they never lose track of what needs to be done.

## Constraints

- [Add boundaries here — e.g. "Must work offline", "Must not require an account"]

## Evidence

- [Add measurable success criteria — e.g. "User can create a task in under 3 taps"]

## Risks

- [Add risks here]

## Open Questions

- [Add open questions here]
```

**Edit `intent.md`** to fill in the ICE sections before running the next step:

```markdown
## Goal

Allow users to manage tasks so they never lose track of what needs to be done.

## Constraints

- Must work offline without network access
- Must persist tasks across app restarts
- Must support at least 1 000 tasks without performance degradation

## Evidence

- Users can create a task with a title and optional due date in under 3 taps
- Users can mark a task as complete; it moves to the "Done" list immediately
- Tasks are still present after the app is closed and reopened
- The task list loads in under 200 ms with 1 000 tasks

## Risks

- Local storage quota limits on mobile browsers

## Open Questions

- Should completed tasks be archived after 30 days or kept indefinitely?
```

Then change the status to `active`:

```markdown
**Status**: active
```

> **Writing good evidence criteria**
>
> Each criterion should be:
> - **Measurable** — "loads in under 200 ms" not "loads quickly"
> - **User-observable** — what the user sees, not how the system implements it
> - **Directly testable** — maps to a concrete Given/When/Then scenario
> - **Independent** — each criterion can be satisfied or failed on its own

---

### Step 1 — Generate the specification

```bash
solidspec specify "Task manager with CRUD and local persistence"
```

This scaffolds `spec.md` with placeholder user stories and requirements. Because the schema is `spec-driven` by default, pass `--schema intent-driven` to use the IDSD spec template which includes an **Intent Reference** header linking `spec.md` back to `INT-001`.

Or run the full pipeline (recommended):

```bash
solidspec pipeline --new "Task manager with CRUD and local persistence" \
  --schema intent-driven \
  --from specify --to clarify
```

**Edit `spec.md`** to turn the placeholders into real requirements. Align the `FR-XXX` requirements with the Evidence criteria in `intent.md` — every FR should be traceable to at least one evidence criterion.

```markdown
## Requirements

### Functional Requirements

- **FR-001**: System MUST allow users to create a task with a title and optional due date
- **FR-002**: System MUST allow users to mark tasks as complete or incomplete
- **FR-003**: System MUST persist tasks in local storage across app sessions
- **FR-004**: System MUST render the task list in under 200 ms with up to 1 000 tasks
```

---

### Step 2 — Clarify ambiguities

```bash
solidspec clarify 001
```

SolidSpec scans for `[NEEDS CLARIFICATION]` markers in `spec.md` and prompts your AI agent to resolve them. Clean up any remaining ambiguous language before proceeding.

---

### Step 3 — Generate the architecture plan

```bash
solidspec plan 001
```

This creates `plan.md`, `data-model.md`, `research.md`, `quickstart.md`, and `contracts/api.md`. Because the schema is `intent-driven`, `plan.md` includes an `## Intent Reference` section that quotes the Goal and confirms each Constraint is respected.

Open `plan.md` and verify the **Constitution Check** section — any constraint from `intent.md` that is unaddressed will appear as a violation.

---

### Step 4 — Generate the task breakdown

```bash
solidspec tasks 001
```

This creates `tasks.md`. The generated tasks reference user stories (`[US1]`, `[US2]`) but **do not** automatically reference FR numbers. Add FR references manually for the traceability chain to work:

```markdown
## Phase 1: Setup

- [ ] T001 Initialize project and local storage adapter [FR-001] [FR-003]
- [ ] T002 Configure CI and linting

## Phase 2: Foundational

- [ ] T003 [P] Implement Task model with validation [FR-001] [FR-002]
- [ ] T004 [P] Implement LocalStorageRepository [FR-003] [FR-004]

## Phase 3: User Story 1 - Create and manage tasks (Priority: P1)

- [ ] T005 [US1] Implement task creation form [FR-001]
- [ ] T006 [US1] Add due date picker component [FR-001]

## Phase 4: User Story 2 - Mark tasks complete (Priority: P1)

- [ ] T007 [US2] Add completion checkbox [FR-002]
- [ ] T008 [US2] Implement done list view [FR-002]

## Phase 5: Performance

- [ ] T009 Implement virtual list rendering [FR-004]
- [ ] T010 Add benchmark suite [FR-004]
```

> **Tip**: Mark tasks with `[FR-XXX]` tags so `solidspec analyze` can trace requirements to tasks and detect orphaned requirements (FRs with no implementing task).

---

### Step 5 — Generate test scaffolds

```bash
solidspec tests 001
```

This creates `tests/` with one scaffold file per user story. The scaffolds contain `GIVEN`/`WHEN`/`THEN` scenarios and are marked `STATUS: NOT IMPLEMENTED`.

To link test files back to tasks (enabling the Task→Test trace link), add a comment at the top of each test file referencing the relevant task IDs:

```markdown
# Test: Create Task
<!-- Tasks: T001 T003 T005 -->

GIVEN: user opens the task manager
WHEN: they type "Buy milk" and tap "Add"
THEN: the task "Buy milk" appears at the top of the list
STATUS: NOT IMPLEMENTED
```

---

### Step 6 — Implement

```bash
solidspec implement 001
```

This is a **handoff** phase — the command pauses and asks you to open your AI agent (Claude, Cursor, etc.) and run `/solidspec-implement 001`. The agent reads `tasks.md`, `spec.md`, and `plan.md` then executes each task, updating checkboxes from `- [ ]` to `- [x]`.

As tests are implemented, update the scaffold files from `STATUS: NOT IMPLEMENTED` to `STATUS: IMPLEMENTED`.

---

### Step 7 — Collect evidence

```bash
solidspec evidence 001
```

This reads `intent.md` Evidence criteria and cross-references them against implemented test scaffolds (`STATUS: IMPLEMENTED`). It prints a per-criterion satisfaction table:

```
Collecting evidence: 001-task-manager

Satisfaction: 3/4 criteria (75%)

  ✓ 1. Users can create a task with a title and optional due date in under 3 taps
  ✓ 2. Users can mark a task as complete; it moves to the "Done" list immediately
  ✓ 3. Tasks are still present after the app is closed and reopened
  ✗ 4. The task list loads in under 200 ms with 1 000 tasks

Report written to specs/001-task-manager/evidence-report.md
```

When satisfaction reaches 100%, update the intent status automatically:

```bash
solidspec evidence 001 --update
# → Updated intent.md Status → satisfied
```

---

### Step 8 — Analyze cross-artifact consistency

```bash
solidspec analyze 001
```

The IDSD-enriched analysis report includes:

```
# Analysis Report

**Traceability Score**: 100%
**Intent Drift**: 25%
  ✗ "The task list loads in under 200 ms with 1 000 tasks" — not covered by implemented tests
**Intent Coverage**: 75%
**Findings**: 0

## Traceability Chain

INT-001
├── FR-001  allow users to create a task with a title and optional due date
│   ├── T001  Initialize project and local storage adapter
│   │   └── story1_create_task.md
│   ├── T003  Implement Task model with validation
│   │   └── story1_create_task.md
│   └── T005  Implement task creation form
├── FR-002  allow users to mark tasks as complete or incomplete
│   ├── T003  Implement Task model with validation
│   └── T007  Add completion checkbox
│       └── story2_complete_task.md
├── FR-003  persist tasks in local storage across app sessions
│   └── T004  Implement LocalStorageRepository
└── FR-004  render the task list in under 200 ms  ← no task
```

**Reading the output:**
- `← no task` — an orphaned requirement (FR-004 above): add a task that references `[FR-004]`
- Intent Drift 25% — one evidence criterion uncovered; above 30% → `High` finding, above 70% → `Critical`
- Intent Coverage 75% — 3 of 4 evidence criteria are covered by at least one implemented test

---

### Step 9 — Review spec quality

```bash
solidspec review 001
```

The review report includes an `IntentAlignment` dimension (score 0–10) alongside the standard 7 dimensions. This dimension checks:

- Every `FR-XXX` requirement traces to at least one evidence criterion in `intent.md`
- The intent `Status` is `active` or `satisfied` (not `draft`) before implementation
- Score: 10/10 when all requirements are traced and status is valid; penalised by 3 points for draft status, 1.5 points per untraced requirement

---

## Running the full pipeline in one command

```bash
solidspec pipeline --new "Task manager with CRUD and local persistence" \
  --schema intent-driven \
  --auto
```

The `--auto` flag skips the `implement` handoff confirmation, useful in CI. Omit it to pause and let your AI agent implement the tasks interactively.

The `--no-agent` flag scaffolds all artifacts without invoking any AI agent — useful for reviewing templates or running tests:

```bash
solidspec pipeline --new "Task manager" --schema intent-driven --no-agent
```

---

## Checking progress

```bash
solidspec status 001 --schema intent-driven
```

```
Feature: 001-task-manager  |  Schema: intent-driven (built-in)
10 artifacts, 7 complete, 2 ready

#     Artifact        Status          Depends On
-----------------------------------------------------------------
1     intent          ✓ done          —
2     spec            ✓ done          intent
3     clarify         ✓ done          spec
4     plan            ✓ done          spec
5     tasks           ✓ done          spec, plan
6     tests           ✓ done          spec
7     implement       ✓ done          tasks
8     evidence        ▶ ready         tests, implement
9     analyze         ▶ ready         spec
10    review          ⏸ blocked       spec

Intent Drift: 25%  (1 evidence criterion unsatisfied)
  ✗ The task list loads in under 200 ms with 1 000 tasks
```

---

## Glossary

| Term | Meaning |
|------|---------|
| **ICE model** | Intent / Constraints / Evidence — the three sections of `intent.md` |
| **INT-XXX** | Intent identifier (e.g. `INT-001`) — the root of the traceability chain |
| **FR-XXX** | Functional requirement in `spec.md` |
| **T-XXX** | Task in `tasks.md` that implements one or more FRs |
| **Intent Drift** | % of evidence criteria NOT yet covered by implemented tests |
| **Intent Coverage** | % of evidence criteria covered by at least one implemented test |
| **Orphaned requirement** | FR-XXX present in spec with no task referencing it in tasks.md |
| **Traceability Chain** | INT-XXX → FR-XXX → T-XXX → test file |
| **Evidence phase** | `solidspec evidence` — reads implemented test scaffolds and reports satisfaction per criterion |
