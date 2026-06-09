# SolidSpec Roadmap

## Current: v0.3.0

### Implemented — SDD (Specification-Driven Development)

| Feature | Status | Description |
|---------|--------|-------------|
| Spec-to-Test Generation | ✅ | Given/When/Then → runnable test scaffolds (Jest, pytest, Cargo, Go, Generic) |
| Multi-Agent Pipeline | ✅ | 8-phase pipeline with 20+ agent support, CLI invocation with timeout |
| Constitution Gates | ✅ | Simplicity, Anti-Abstraction, Integration-First — auto-checked in plan phase |
| Extension Hooks | ✅ | Cross-platform hooks (sh/ps1/cmd) — after_init, before_tasks, etc. |
| Preset System | ✅ | Import/export workflow presets with priority-based template resolution |
| Anti-Rationalization Guards | ✅ | Excuse→rebuttal table + compliance checklist injected into every agent prompt |
| Agent Personas | ✅ | 8 role-based personas (Spec Writer, Architect, Code Reviewer, etc.) with verification checklists |
| Project Context Injection | ✅ | `[context]` in solidspec.toml → auto-injected into every prompt |
| DAG Artifact Graph | ✅ | Kahn's algorithm topological sort, completion detection, `solidspec status` |
| Schema-Driven Workflows | ✅ | 4 built-in schemas (spec-driven, minimal, security-first, intent-driven), YAML-customizable |
| Change-Based Workflow | ✅ | Delta specs (ADDED/MODIFIED/REMOVED), propose → list → archive lifecycle |
| OpenCode Skills | ✅ | Directory-based `.opencode/skills/` with `name:` + `description:` SKILL.md format |
| Agent Timeout | ✅ | 300s `try_wait()` polling loop, process killed on timeout |

### Implemented — IDSD (Intent-Driven Specification Development)

All IDSD features are additive and fully backward-compatible. The `spec-driven` schema and all existing SDD templates are unchanged.

| Phase | Feature | Status | Description |
|-------|---------|--------|-------------|
| P1 | Intent Foundation | ✅ | `solidspec intent` captures the ICE model (Goal / Constraints / Evidence) into `intent.md`. `intent-driven` schema YAML with `intent` as root artifact. |
| P2 | Intent-Aware Pipeline | ✅ | `intent` as phase 0 in the IDSD pipeline (`--schema intent-driven`). IDSD-specific templates (`idsd/spec-template.md`, `idsd/plan-template.md`) include an `## Intent Reference` section. `plan.rs` injects `intent_goal`, `intent_constraints`, `intent_evidence` into template vars. Constitution check validates intent constraints against the plan. |
| P3 | Intent Drift Detection | ✅ | `compute_drift()` in `analyzer.rs` cross-references evidence criteria from `intent.md` against `STATUS: IMPLEMENTED` test scaffolds using keyword overlap (≥5-char terms). Drift ≥ 30% → High finding; ≥ 70% → Critical. Shown in `solidspec analyze` and `solidspec status --schema intent-driven`. |
| P4 | Intent Alignment Review Dimension | ✅ | `Dimension::IntentAlignment` (8th review dimension). Scores 0–10: −3 for `draft` status, −1.5 per FR-XXX not traceable to any evidence criterion. Score 0/10 when `intent.md` absent; added after cap so it is never hidden by overflow. `## Intent Alignment` section in `review-report.md`. |
| P5 | Evidence-Based Validation | ✅ | `solidspec evidence [id] [--update]` reads `intent.md` Evidence criteria, scans `tests/` for `STATUS: IMPLEMENTED` scaffolds, and produces `evidence-report.md` with a per-criterion satisfaction table. `--update` rewrites `intent.md` Status (`active` / `satisfied` / `drifted`) automatically. Integrated as phase 8 in the IDSD pipeline. |
| P6 | Full Traceability Chain | ✅ | `build_trace_graph()` in `artifact_graph.rs` constructs `INT-XXX → FR-XXX → T-XXX → test_file` links by parsing `intent.md` (INT ID), `spec.md` (FR definitions), `tasks.md` ([FR-XXX] tags), and test files (T-number comments). Renders as ASCII tree in `solidspec analyze`. Orphaned FRs (spec requirement with no task referencing it) → High finding. `intent_coverage` metric (% of evidence criteria covered by implemented tests) added to `AnalysisReport`. |
| P7 | Integration Tests & Workflow Guide | ✅ | 8 end-to-end integration tests in `tests/traceability.rs` covering the full IDSD chain (pipeline scaffold, trace tree, Task→Test links, orphaned FRs, intent coverage, evidence update, SDD purity, no IDSD bleed). `docs/idsd-workflow-guide.md`: complete Task Manager walkthrough with all 10 IDSD phases, ICE writing tips, trace link instructions, output reading guide, glossary. |

---

## Next: v0.4.0

### Planned

| Priority | Feature | Est. Effort | Why |
|----------|---------|-------------|-----|
| **HIGH** | Parallel Fan-Out Orchestration | 12h | Concurrent review (code + security + test + perf), ship decision |
| **HIGH** | Doubt-Driven Development | 16h | In-flight adversarial review (3-cycle bounded) catches problems during implementation |
| **MEDIUM** | Spec Import from Issues | 8h | `solidspec import --github 42` — pre-fill spec from GitHub Issues / Jira |
| **MEDIUM** | IDSD: `evidence` → code source link | 6h | Parse implemented test files to extract which source files they exercise; add `test → src` layer to the traceability chain |
| **LOW** | Shell Completions Enhancement | 4h | Install completions via `solidspec completions install <shell>` |
| **LOW** | MSRV Declaration | 1h | Declare Minimum Supported Rust Version in Cargo.toml |

---

## Future: v1.0.0

### Backlog

| Feature | Difficulty | Impact | Description |
|---------|-----------|--------|-------------|
| Interactive TUI Builder | Medium | High | `ratatui`-based guided spec + intent creation with real-time quality scoring |
| Live Traceability Matrix (AST) | Very High | Very High | Scan source code via tree-sitter AST → map requirement IDs to actual code locations, closing the last gap in the full chain |
| AI-Powered Spec Review | Very High | High | Send spec to AI agent for structured quality review with scoring (IDSD mode: include evidence alignment in the prompt) |
| Workspace Coordination | Medium | Medium | Multi-repo coordination with linked workspaces |
| IDSD: Drift Alerts in CI | Medium | High | `solidspec analyze --fail-on-drift 30` exit-codes non-zero when drift exceeds threshold; designed for CI gates |
| IDSD: Intent Versioning | Medium | Medium | Track intent evolution over time (`intent-v1.md`, `intent-v2.md`), diff constraints and evidence across versions |

---

## Full Feature Comparison

See [docs/KILLER_FEATURE_IDEAS.md](KILLER_FEATURE_IDEAS.md) for original feature brainstorming with implementation details.

See [docs/idsd-workflow-guide.md](idsd-workflow-guide.md) for the complete IDSD walkthrough with a Task Manager use case.
