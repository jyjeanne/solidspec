# SDD + IDSD Dual-Mode Improvement Plan — SolidSpec

## Purpose

Add **Intent-Driven Development (IDSD)** as an opt-in workflow alongside the existing **Specification-Driven Development (SDD)** workflow. Users keep their current SDD experience unchanged and can choose IDSD when they want intent-first development with drift detection and evidence-based validation.

**SDD is not replaced.** The `spec-driven` schema and all existing templates remain untouched. IDSD is a new mode, not a migration.

> **Reference articles:**
> - [The Method That Replaces Spec-Driven Development — IDSD](https://medium.com/activated-thinker/the-method-that-replaces-spec-driven-development-idsd-66e921f6cdf7)
> - [The Anatomy of Intent (ICE in IDSD)](https://medium.com/activated-thinker/the-anatomy-of-intent-ice-in-idsd-built-from-where-spec-driven-breaks-1597e5a16659)

---

## Workflow Mode Selection

Users choose their workflow via the `--schema` flag or `solidspec.toml`:

```toml
# solidspec.toml — default (unchanged behaviour)
[pipeline]
schema = "spec-driven"

# or opt into IDSD:
[pipeline]
schema = "intent-driven"
```

Or per-invocation:

```bash
# SDD (existing, unchanged)
solidspec pipeline --new "Feature X"
solidspec pipeline --new "Feature X" --schema spec-driven

# IDSD (new opt-in)
solidspec pipeline --new "Feature X" --schema intent-driven
```

### Comparison

| Capability | `spec-driven` | `intent-driven` |
|---|---|---|
| Spec → Plan → Tasks → Tests → Implement | ✅ Full | ✅ Full |
| `specify`, `plan`, `tasks`, `tests`, `implement`, `analyze`, `review` commands | ✅ Unchanged | ✅ Unchanged |
| Traceability score (FR → Task) | ✅ Existing | ✅ Existing |
| `intent.md` (ICE model) | ❌ Not used | ✅ Phase 0 |
| Intent drift score | ❌ | ✅ Phase 3 |
| Evidence-based validation | ❌ | ✅ Phase 5 |
| Full traceability chain (Intent → Code → Test) | ❌ | ✅ Phase 6 |
| IntentAlignment review dimension | ❌ | ✅ Phase 4 |

### Backward Compatibility Guarantee

- The `spec-driven` schema YAML is **never modified** by this plan
- All existing templates (`spec-template.md`, `plan-template.md`, `tasks-template.md`, …) are **never modified**
- All existing CLI commands behave identically when `--schema spec-driven` (default)
- New IDSD commands (`intent`, `evidence`) are **additive** — they do nothing harmful when run against a spec-driven project

---

## Codebase Baseline (verified June 2026)

### What Already Exists

| Area | Location | Status |
|------|----------|--------|
| CLI subcommands | `src/cli/` | 17 commands: `init`, `specify`, `clarify`, `plan`, `tasks`, `implement`, `tests`, `analyze`, `review`, `checklist`, `pipeline`, `preset`, `change`, `extension`, `upgrade`, `completions`, `check`, `status` |
| Spec parsing | `src/core/spec_parser.rs` | Parses `spec.md` into `ParsedSpec { user_stories, requirements, clarification_markers, entities, raw }` |
| Task generation | `src/core/task_generator.rs` | Generates `TaskList { phases }` with 4 phase types (Setup, Foundational, per-story, Polish) |
| Test scaffolds | `src/core/test_generator.rs` | Extracts Given/When/Then scenarios, detects framework (Jest, Pytest, Cargo, Go), renders scaffold files |
| Artifact DAG | `src/core/artifact_graph.rs` | DAG engine with Kahn's topological sort; states: `Blocked`, `Ready`, `Done` |
| Workflow schemas | `src/core/schema.rs` + `schemas/` | Three built-in schemas: `spec-driven` (8 artifacts), `minimal` (4 artifacts), `security-first` (adds `security-review`) |
| Traceability analysis | `src/core/analyzer.rs` | `traceability_score` = traced requirements / total * 100; checks plan/task cross-references |
| Quality review | `src/core/review.rs` | 7 dimensions: Completeness, Clarity, Testability, Consistency, Security, Performance, Maintainability |
| Brownfield changes | `src/core/change.rs` | Delta spec parser, archive merge engine, `ChangeMetadata` |
| Constitution | `src/core/constitution.rs` | Gates parsed from articles (Simplicity VII, Anti-Abstraction VIII, Integration-First IX); `check_plan_compliance()` |
| Pipeline orchestration | `src/core/pipeline.rs` | 8 phases: specify → clarify → plan → tasks → tests → implement → analyze → review |
| Feature resolution | `src/core/feature.rs` | 4-level cascade: CLI arg → env var → git branch (`^\d{3}-.+$`) → latest `specs/` dir |
| Templates | `templates/` | `spec-template.md`, `plan-template.md`, `tasks-template.md`, `checklist-template.md`, `constitution-template.md`, `agent-file-template.md` |
| Config | `src/config/` | `RootConfig` (project, ai, git, templates, pipeline, context) + `ProjectInternalConfig` |

### What Does NOT Yet Exist (IDSD gaps)

| IDSD Feature | Status |
|---|---|
| `intent.md` artifact | ❌ Missing |
| `/intents` directory per feature | ❌ Missing |
| ICE model (Intent / Constraints / Evidence) | ❌ Missing |
| `intent-template.md` | ❌ Missing |
| `IntentSpec` core struct + parser | ❌ Missing |
| `intent` CLI subcommand (capture) | ❌ Missing |
| Intent drift detection / score | ❌ Missing |
| Evidence-based validation | ❌ Missing |
| Full traceability graph (Intent → Spec → Task → Code → Test) | ❌ Missing (partial: Spec → Task exists) |
| Intent compliance in `pipeline` phases | ❌ Missing |
| Intent compliance in `review` dimensions | ❌ Missing |
| `intent-driven` workflow schema | ❌ Missing |

---

## Target Architecture

### SDD Workflow (existing — unchanged)

```
Specification (spec.md)
   │
   ↓  [solidspec plan]
Architecture Plan (plan.md)
   │
   ↓  [solidspec tasks]
Task Breakdown (tasks.md)
   │
   ↓  [solidspec tests]
Test Scaffolds
   │
   ↓  [solidspec implement / analyze / review]
Code + Reports
```

### IDSD Workflow (new — opt-in via `--schema intent-driven`)

```
Intent (intent.md — ICE model)          ← NEW phase 0
   │
   ↓  [solidspec specify]
Specification (spec.md)                 ← unchanged format
   │
   ↓  [solidspec plan]
Architecture Plan (plan.md)             ← unchanged format
   │
   ↓  [solidspec tasks / tests / implement]
Code + Test Scaffolds                   ← unchanged
   │
   ↓  [solidspec analyze]
Analysis Report + Intent Drift Score    ← EXTENDED (drift added)
   │
   ↓  [solidspec review]
Review Report — 8 dimensions            ← EXTENDED (IntentAlignment added)
   │
   ↓  [solidspec evidence]              ← NEW command
Evidence Satisfaction Report
```

Each artifact in the IDSD chain references the parent intent ID (`INT-001`). The SDD chain has no such requirement.

---

## ICE Model (Intent / Constraints / Evidence)

Every `intent.md` file contains three sections:

### I — Goal
Why the capability exists. One sentence, no implementation detail.

```markdown
## Goal
Allow users to validate specifications automatically against their stated intent.
```

### C — Constraints
Boundaries that must remain true regardless of implementation changes.

```markdown
## Constraints
- Must support Markdown output
- Must be CI-compatible (no interactive prompts)
- Must complete in under 10 seconds
- Must not require network access
```

### E — Evidence
How success is measured. Directly maps to test scenarios and metrics.

```markdown
## Evidence
- Validation command exits 0 when intent is satisfied
- Drift score < 10% after implementation
- All Given/When/Then scenarios pass
- Review report overall score ≥ 70
```

### Full `intent.md` structure

```markdown
# Intent: <title>

**Intent ID**: INT-001
**Feature**: 001-<slug>
**Created**: YYYY-MM-DD
**Status**: draft | active | satisfied | drifted

## Goal
<one-sentence statement of why this capability exists>

## Constraints
- <boundary 1>
- <boundary 2>

## Evidence
- <measurable success criterion 1>
- <measurable success criterion 2>

## Risks
- <risk 1>

## Open Questions
- <question requiring clarification>
```

---

## Intent Drift Score

Measures how far the current implementation has drifted from the original intent.

Computed by `solidspec analyze` by cross-referencing:
- Evidence criteria from `intent.md`
- Passing test scenarios from `tests/`
- Requirement coverage in `plan.md` and `tasks.md`

```
drift_score = (unsatisfied_evidence_count / total_evidence_count) * 100
```

Example output:
```
Intent Drift:  20%  (1 of 5 evidence criteria unsatisfied)
  ✗ "Drift score < 10% after implementation"  — no analyze output found
```

---

## Full Traceability Chain

```
INT-001
 └─ spec.md        (generated from intent)
     └─ T001       (tasks.md references FR-001)
         └─ PR-001 (PR description references INT-001)
             └─ TEST-001 (test file references story from spec)
```

`solidspec analyze` will report coverage at each level.

---

## Phased Development Plan

---

### Phase 1 — Intent Foundation *(estimated: 3–4 days)*

Introduce `intent.md` as a first-class artifact **in the new `intent-driven` schema only**. Zero changes to `spec-driven` schema or existing templates.

#### Tasks

| ID | Task | Files |
|----|------|-------|
| P1-T1 | Create `intent-template.md` in `templates/` with ICE structure | `templates/intent-template.md` |
| P1-T2 | Add `IntentSpec` struct and `parse_intent()` to a new `src/core/intent_parser.rs` | `src/core/intent_parser.rs`, `src/core/mod.rs` |
| P1-T3 | Add `intent` subcommand (`src/cli/intent.rs`) that creates `intent.md` in the feature dir | `src/cli/intent.rs`, `src/cli/mod.rs` |
| P1-T4 | Create new `schemas/intent-driven/schema.yaml` — 9-artifact chain with `intent` as first node (spec requires intent); **do not modify `spec-driven` schema** | `schemas/intent-driven/schema.yaml` |
| P1-T5 | Embed `intent-template.md` via `include_str!()` in `src/templates/mod.rs` | `src/templates/mod.rs` |
| P1-T6 | Update `specify` command to print a soft reminder if `intent.md` is absent **and** active schema is `intent-driven` | `src/cli/specify.rs` |
| P1-T7 | Add unit tests for `IntentSpec` parsing (Goal, Constraints, Evidence extraction) | `src/core/intent_parser.rs` |
| P1-T8 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

**Acceptance criteria:**
- `solidspec intent "Allow users to export PDF"` creates `specs/001-*/intent.md` with ICE scaffold
- `solidspec status 001 --schema intent-driven` shows `intent` artifact as `done` when `intent.md` exists
- `solidspec status 001` (no schema flag / default `spec-driven`) is **identical to today** — no new columns, no warnings
- Existing tests still pass; `spec-driven` schema YAML is byte-for-byte identical to before

---

### Phase 2 — Intent-Aware Pipeline *(estimated: 2–3 days)*

Wire intent into the `pipeline` command **only when `--schema intent-driven` is active**. Existing SDD pipeline flow is untouched. Create new IDSD-specific templates; do not modify `plan-template.md` or `spec-template.md`.

#### Tasks

| ID | Task | Files |
|----|------|-------|
| P2-T1 | Add `intent` as phase 0 in `src/core/pipeline.rs` IDSD branch (guarded by `schema == "intent-driven"`) | `src/core/pipeline.rs` |
| P2-T2 | Update `pipeline` CLI handler: when `--schema intent-driven` and `--new`, invoke `intent` subcommand first | `src/cli/pipeline.rs` |
| P2-T3 | Pass `Option<IntentSpec>` into `generate_plan()` context; only non-None when schema is `intent-driven` | `src/core/task_generator.rs`, `src/cli/plan.rs` |
| P2-T4 | Create `templates/idsd/plan-template.md` — extends SDD plan with `## Intent Reference` section (`{{ intent_goal }}`); **do not modify** `templates/plan-template.md` | `templates/idsd/plan-template.md` |
| P2-T5 | Create `templates/idsd/spec-template.md` — extends SDD spec with `**Intent**: INT-{{ feature_id }}` header; **do not modify** `templates/spec-template.md` | `templates/idsd/spec-template.md` |
| P2-T6 | Update `intent-driven` schema YAML to reference `idsd/plan-template.md` and `idsd/spec-template.md` | `schemas/intent-driven/schema.yaml` |
| P2-T7 | Update `constitution.rs` gate checks: when `IntentSpec` is provided, also validate against intent constraints (additive, no-op when intent is absent) | `src/core/constitution.rs` |
| P2-T8 | Add integration test: `pipeline --new "X" --schema intent-driven` produces `intent.md` before `spec.md` | `tests/pipeline.rs` |
| P2-T9 | Add integration test: `pipeline --new "X"` (default schema) produces NO `intent.md` and output is identical to today | `tests/pipeline.rs` |
| P2-T10 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

**Acceptance criteria:**
- `solidspec pipeline --new "Feature X" --schema intent-driven` generates `intent.md` → `spec.md` → …
- `solidspec pipeline --new "Feature X"` (SDD default) does **not** generate `intent.md`; output identical to current behaviour
- `plan.md` from SDD workflow contains no `## Intent Reference` section
- `solidspec check` reports missing `intent.md` as a warning **only** when schema is `intent-driven`

---

### Phase 3 — Intent Drift Detection *(estimated: 3–4 days)*

Add drift score to the `analyze` command by comparing evidence criteria against actual artifacts.

#### Tasks

| ID | Task | Files |
|----|------|-------|
| P3-T1 | Add `EvidenceCriterion { text, satisfied: bool }` and `IntentDrift { score, unsatisfied }` to `src/core/intent_parser.rs` | `src/core/intent_parser.rs` |
| P3-T2 | Implement `compute_drift()` in `src/core/analyzer.rs`: parse `intent.md` evidence, cross-reference with test scaffold files + `review-report.md` score | `src/core/analyzer.rs` |
| P3-T3 | Add `intent_drift: Option<IntentDrift>` field to `AnalysisReport` | `src/core/analyzer.rs` |
| P3-T4 | Update `analyze` CLI output to print drift score and unsatisfied criteria | `src/cli/analyze.rs` |
| P3-T5 | Add `drift` severity level to `Finding`: mark `High` if drift > 30%, `Critical` if drift > 70% | `src/core/analyzer.rs` |
| P3-T6 | Update `status` command to show intent drift score in artifact table **when schema is `intent-driven`** | `src/cli/status.rs` |
| P3-T7 | Add unit tests for `compute_drift()` with mock intent + test files | `src/core/analyzer.rs` |
| P3-T8 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

**Acceptance criteria:**
- `solidspec analyze 001` prints `Intent Drift: X%` section
- Unsatisfied evidence items listed individually with severity
- `solidspec status 001` shows drift score column
- Drift is 0% when all test scaffold files contain `STATUS: NOT IMPLEMENTED` (baseline — criteria exist but not yet measured)

---

### Phase 4 — Intent Alignment Review Dimension *(estimated: 2 days)*

Add `IntentAlignment` as an 8th review dimension in `src/core/review.rs`.

#### Tasks

| ID | Task | Files |
|----|------|-------|
| P4-T1 | Add `IntentAlignment` variant to `Dimension` enum | `src/core/review.rs` |
| P4-T2 | Implement `review_intent_alignment()`: checks that every `FR-XXX` requirement can be traced to an intent evidence criterion; checks intent `Status` field is not `draft` before implementation | `src/core/review.rs` |
| P4-T3 | Add `IntentAlignment` to `DimensionScore` computation and `overall_score` weighting | `src/core/review.rs` |
| P4-T4 | Update `review-report.md` output to include intent alignment section | `src/cli/review.rs` |
| P4-T5 | Add unit tests for intent alignment checks | `src/core/review.rs` |
| P4-T6 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

**Acceptance criteria:**
- `solidspec review 001` shows `IntentAlignment` as a dimension with a score
- Missing `intent.md` results in 0/10 for IntentAlignment dimension
- Review report markdown contains `## Intent Alignment` section

---

### Phase 5 — Evidence-Based Validation *(estimated: 3–4 days)*

Replace pass/fail spec checks with evidence satisfaction status.

#### Tasks

| ID | Task | Files |
|----|------|-------|
| P5-T1 | Create `EvidenceReport { criteria, satisfied_count, total_count, status }` struct in new `src/core/evidence.rs` | `src/core/evidence.rs`, `src/core/mod.rs` |
| P5-T2 | Implement `collect_evidence()`: scan test scaffold files for `STATUS: IMPLEMENTED` vs `STATUS: NOT IMPLEMENTED`, cross-reference evidence criteria | `src/core/evidence.rs` |
| P5-T3 | Add `evidence` subcommand (`src/cli/evidence.rs`) that prints evidence satisfaction table | `src/cli/evidence.rs`, `src/cli/mod.rs` |
| P5-T4 | Add `evidence` artifact to `intent-driven` schema YAML (requires `tests`, `implement`) | `schemas/intent-driven/schema.yaml` |
| P5-T5 | Update `pipeline` to run evidence collection after `implement` phase **when schema is `intent-driven`** | `src/cli/pipeline.rs`, `src/core/pipeline.rs` |
| P5-T6 | Add intent `Status` auto-update: set `satisfied` when evidence score = 100%, `drifted` when drift > 30% | `src/core/evidence.rs` |
| P5-T7 | Add integration tests for evidence collection | `tests/` |
| P5-T8 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

**Acceptance criteria:**
- `solidspec evidence 001` prints per-criterion satisfaction status
- `intent.md` Status field updated automatically by `solidspec evidence 001 --update`
- `solidspec status 001 --schema intent-driven` shows `evidence` artifact node

---

### Phase 6 — Full Traceability Graph *(estimated: 2–3 days)*

Extend `artifact_graph.rs` and `analyzer.rs` to produce a complete Intent → Spec → Task → Code → Test traceability chain.

#### Tasks

| ID | Task | Files |
|----|------|-------|
| P6-T1 | Add `TraceLink { from_id, to_id, link_type }` and `TraceGraph` to `src/core/artifact_graph.rs` | `src/core/artifact_graph.rs` |
| P6-T2 | Implement `build_trace_graph()`: parse `intent.md` → `spec.md` (FR-XXX) → `tasks.md` (T-XXX) → test files | `src/core/artifact_graph.rs` |
| P6-T3 | Add `trace_graph: Option<TraceGraph>` to `AnalysisReport` | `src/core/analyzer.rs` |
| P6-T4 | Update `analyze` output to print traceability chain as ASCII tree | `src/cli/analyze.rs` |
| P6-T5 | Add `intent_coverage: f32` metric to `AnalysisReport` (% of intent evidence criteria with at least one trace to a test) | `src/core/analyzer.rs` |
| P6-T6 | Add unit tests for `build_trace_graph()` with fixture files | `src/core/artifact_graph.rs` |
| P6-T7 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

**Acceptance criteria:**
- `solidspec analyze 001` prints traceability chain: `INT-001 → FR-001 → T001 → TEST-001`
- `intent_coverage` reported as a percentage
- Orphaned requirements (FR-XXX with no task) shown as `High` severity findings

---

## Success Metrics (per phase)

| Metric | Phase | Target |
|--------|-------|--------|
| `intent.md` present on all features | P1 | 100% |
| Pipeline generates intent before spec | P2 | ✓ |
| Intent drift score computed | P3 | `analyze` output |
| IntentAlignment dimension in review | P4 | score ≥ 0 |
| Evidence satisfaction rate | P5 | ≥ 80% at ship |
| Full trace chain coverage | P6 | ≥ 90% |

---

## Agent Rules (for AI agents operating in IDSD mode)

> These rules apply **only** when `pipeline.schema = "intent-driven"` (or `--schema intent-driven` is passed). Agents using the default `spec-driven` schema are unaffected.

Agents running `solidspec pipeline --schema intent-driven` must:

1. Read `intent.md` **before** reading `spec.md`
2. Confirm intent goal aloud before generating a plan
3. Produce evidence artifacts after implementation
4. Refuse to implement when `intent.md` is in `draft` status
5. Report drift score after each implementation phase

---

## References

- [IDSD: The Method That Replaces Spec-Driven Development](https://medium.com/activated-thinker/the-method-that-replaces-spec-driven-development-idsd-66e921f6cdf7)
- [The Anatomy of Intent: ICE in IDSD](https://medium.com/activated-thinker/the-anatomy-of-intent-ice-in-idsd-built-from-where-spec-driven-breaks-1597e5a16659)
- Architecture reference: `docs/ARCHITECTURE.md`
- Pipeline reference: `docs/multi-agent-pipeline.md`
