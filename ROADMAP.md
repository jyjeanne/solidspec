# SolidSpec Roadmap

## Current: v0.3.0

SolidSpec has evolved from a single-methodology SDD tool into a **multi-methodology AI development platform**. Seven built-in schemas cover the full spectrum from lightweight spikes to fully-traced, intent-anchored, TDD-enforced production workflows.

---

### Implemented — Core Infrastructure

| Feature | Status | Description |
|---------|--------|-------------|
| DAG Artifact Graph | ✅ | Kahn's algorithm topological sort, completion detection, `solidspec status` |
| Schema-Driven Workflows | ✅ | 7 built-in schemas (YAML-customizable), 3-level resolution (project-local → built-in → default) |
| Multi-Agent Support (20) | ✅ | Auto-detection, format translation, slash command registration per agent |
| Multi-Agent Pipeline | ✅ | Automated pipeline with CLI invocation, timeout, mixed-mode and scaffold-only modes |
| Agent Timeout | ✅ | 300s `try_wait()` polling loop, process killed on timeout |
| Constitution Gates | ✅ | Simplicity, Anti-Abstraction, Integration-First — auto-checked in plan phase |
| Extension Hooks | ✅ | Cross-platform hooks (sh/ps1/cmd) — after_init, before_tasks, etc. |
| Preset System | ✅ | Import/export workflow presets with priority-based template resolution |
| Anti-Rationalization Guards | ✅ | Excuse→rebuttal table + compliance checklist injected into every agent prompt |
| Agent Personas | ✅ | Role-based personas (Spec Writer, Architect, Code Reviewer, etc.) with verification checklists |
| Project Context Injection | ✅ | `[context]` in solidspec.toml → auto-injected into every prompt |
| Change-Based Workflow | ✅ | Delta specs (ADDED/MODIFIED/REMOVED), propose → list → archive lifecycle |
| OpenCode Skills | ✅ | Directory-based `.opencode/skills/` with `name:` + `description:` SKILL.md format |
| Template System | ✅ | Tera rendering, 4-layer resolver (project-local → preset → extension → embedded default) |

---

### Implemented — SDD (Specification-Driven Development)

Workflow: `spec-driven` (9 artifacts), `minimal` (4 artifacts)

| Feature | Status | Description |
|---------|--------|-------------|
| Spec Generation | ✅ | User stories, FR-### requirements, acceptance scenarios, quality checklist |
| Plan Generation | ✅ | Architecture plan + research + data model + contracts + constitution check |
| Task Generation | ✅ | Phased task breakdown with `[P]` parallel markers and `[US#]` user story links |
| Spec-to-Test Scaffolds | ✅ | Given/When/Then → runnable test stubs (Jest, Vitest, pytest, Cargo, Go, Generic) |
| Cross-Artifact Analysis | ✅ | Requirement traceability, entity coverage, constitution compliance by severity |
| Spec Quality Review | ✅ | 7-dimension preflight scoring (Completeness, Clarity, Testability, Consistency, Security, Performance, Maintainability) |
| `minimal` Schema | ✅ | 4-artifact lightweight path: spec → plan → tasks → implement |

---

### Implemented — Security-First Development

Workflow: `security-first` (5 artifacts)

| Feature | Status | Description |
|---------|--------|-------------|
| Security-First Schema | ✅ | Mandatory OWASP Top 10 security review as DAG dependency before tasks can be generated |
| OWASP Audit Gate | ✅ | Security findings by severity (Critical/High/Medium/Low); every finding becomes a mitigation task |

---

### Implemented — AI-TDD (Test-Driven Development)

Workflow: `tdd-driven` (10 artifacts)

| Phase | Feature | Status | Description |
|-------|---------|--------|-------------|
| RED | `tdd-tests` command | ✅ | Scaffolds `tdd-red-report.md` with interface design section, tracer bullet (first AC), cycle table (remaining ACs), quality checklist, and unexpectedly-passing field. Creates `tests/` directory. |
| RED | `tdd-red-report.md` structure | ✅ | 5 sections: Interface Design, Tracer Bullet, Remaining Cycles (table), Test Quality Checklist, Test Results. Extracted from spec acceptance criteria automatically. |
| RED | Agent command (`/solidspec-tdd-tests`) | ✅ | 5-step instruction body enforcing: interface design before any test, tracer-bullet first, vertical slices (never horizontal), mock boundary list (only external systems), framework detection gate. |
| GREEN | TDD implement instruction | ✅ | Schema instruction enforces one-failing-test-at-a-time; working from the cycle table in `tdd-red-report.md`; no bulk implementation allowed. |
| REFACTOR | `tdd-refactor` command | ✅ | Scaffolds `tdd-refactor-report.md` with 6 named refactor candidates (Duplication, Long methods, Shallow modules, Feature envy, Primitive obsession, Interface creep), changes audit table with Refactor Type column, and Definition of Done. |
| REFACTOR | Agent command (`/solidspec-tdd-refactor`) | ✅ | Enforces: pre-condition full test run, per-change test run, interface-must-not-grow rule, FORBIDDEN list (new behavior, test modification, interface expansion). |
| Pipeline | `tdd-driven` schema | ✅ | 10-artifact DAG; tdd-tests, implement, and tdd-refactor are `PhaseType::Handoff`; pipeline skip logic and phase numbering correct (tdd-tests: 5/9, implement: 6/9, tdd-refactor: 7/9). |
| Quality | `plan` and `tasks` instructions enriched | ✅ | Plan instruction adds deep-module design and interface testability guidance; tasks instruction adds per-task AC link and per-task mini RED-GREEN. |
| Tests | 56 integration tests | ✅ | Full coverage of RED/GREEN/REFACTOR phases, pipeline flags, status DAG, command bodies, multi-feature resolution, and end-to-end scaffold consistency. |

---

### Implemented — IDSD (Intent-Driven Specification Development)

Workflow: `intent-driven` (11 artifacts)

All IDSD features are additive and fully backward-compatible. The `spec-driven` schema and all existing SDD templates are unchanged.

| Phase | Feature | Status | Description |
|-------|---------|--------|-------------|
| P1 | Intent Foundation | ✅ | `solidspec intent` captures the ICE model (Goal / Constraints / Evidence) into `intent.md`. `intent-driven` schema YAML with `intent` as root artifact. |
| P2 | Intent-Aware Pipeline | ✅ | `intent` as phase 0 in the IDSD pipeline. IDSD-specific templates include `## Intent Reference` sections. Plan phase injects `intent_goal`, `intent_constraints`, `intent_evidence` into template vars. Constitution check validates intent constraints against the plan. |
| P3 | Intent Drift Detection | ✅ | `compute_drift()` cross-references evidence criteria from `intent.md` against `STATUS: IMPLEMENTED` test scaffolds using keyword overlap. Drift ≥ 30% → High finding; ≥ 70% → Critical. Shown in `solidspec analyze` and `solidspec status --schema intent-driven`. |
| P4 | Intent Alignment Review | ✅ | `Dimension::IntentAlignment` (8th review dimension). Scores 0–10: −3 for `draft` status, −1.5 per FR-XXX not traceable to any evidence criterion. |
| P5 | Evidence-Based Validation | ✅ | `solidspec evidence [id] [--update]` reads `intent.md` Evidence criteria, scans `tests/` for `STATUS: IMPLEMENTED` scaffolds, and produces `evidence-report.md` with per-criterion satisfaction table. |
| P6 | Full Traceability Chain | ✅ | `build_trace_graph()` constructs `INT-XXX → FR-XXX → T-XXX → test_file` links. Renders as ASCII tree in `solidspec analyze`. Orphaned FRs → High finding. `intent_coverage` metric in `AnalysisReport`. |
| P7 | Integration Tests & Workflow Guide | ✅ | 8 end-to-end integration tests in `tests/traceability.rs`. `docs/idsd-workflow-guide.md`: complete Task Manager walkthrough with all IDSD phases. |

---

### Implemented — APEX Workflows

Workflows: `apex-driven` (9 artifacts), `intent-apex` (11 artifacts)

| Feature | Status | Description |
|---------|--------|-------------|
| `apex-driven` schema | ✅ | SDD workflow with APEX replacing the manual implement handoff. APEX gets `spec.md + plan.md + tasks.md` as pre-loaded context. |
| `intent-apex` schema | ✅ | IDSD workflow with APEX replacing implement. Evidence phase requires `tests + apex`. Maximum rigor: intent-anchored + evidence-collected + APEX-implemented. |

---

### Implemented — Parallel Fan-Out Ship Gate

| Feature | Status | Description |
|---------|--------|-------------|
| 4-Lane Concurrent Review | ✅ | Code, Security, Tests, Performance lanes run in parallel via `solidspec ship` |
| AI Score Extraction | ✅ | `SCORE: N` suffix extraction from agent output; fallback severity-count formula |
| `--no-agent` Heuristic Mode | ✅ | Runs `solidspec review` heuristics filtered to each lane — no agent tokens required |
| HOLD Triggers | ✅ | TimedOut, Failed, CRITICAL security finding, `block_on_critical`, score below threshold |
| CI Integration | ✅ | `--fail-on-hold` exits 1 on HOLD; `--ignore-timeout` for flaky CI |
| Per-Lane Overrides | ✅ | `--lane`, `--code-agent`, `--security-agent`, per-lane thresholds in `solidspec.toml` |
| `ship-report.md` | ✅ | Machine-readable `<!-- ship: true|false -->` header + per-lane scores |

---

## Next: v0.4.0

### Planned

| Priority | Feature | Est. Effort | Why |
|----------|---------|-------------|-----|
| **HIGH** | Doubt-Driven Development | 16h | In-flight adversarial review (3-cycle bounded) that catches implementation problems mid-cycle. Complements TDD by challenging the agent's design assumptions during the GREEN phase. |
| **HIGH** | Spec Import from Issues | 8h | `solidspec import --github 42` — pre-fill spec from GitHub Issues / Jira tickets. Speeds up the specify phase for teams that already capture requirements in issue trackers. |
| **MEDIUM** | `evidence` → code source link | 6h | Parse implemented test files to extract which source files they exercise; add `test → src` layer to the IDSD traceability chain, closing the last gap in `INT → FR → T → test → src`. |
| **MEDIUM** | TDD: Cycle Progress Tracking | 4h | `solidspec tdd-status` shows RED/GREEN/REFACTOR progress: tests written vs. passing, refactor candidates resolved vs. pending. |
| **LOW** | Shell Completions Enhancement | 4h | `solidspec completions install <shell>` — one-command install that writes to the correct profile file. |
| **LOW** | MSRV Declaration | 1h | Declare Minimum Supported Rust Version in `Cargo.toml`. |

---

## Future: v1.0.0

### Backlog

| Feature | Difficulty | Impact | Description |
|---------|-----------|--------|-------------|
| Interactive TUI Builder | Medium | High | `ratatui`-based guided workflow: spec + intent creation with real-time quality scoring. Workflow selector shows the comparison matrix and recommends a schema based on answers. |
| Live Traceability Matrix (AST) | Very High | Very High | Scan source code via tree-sitter AST → map requirement IDs to actual code locations, closing the last gap in the full `INT → FR → T → test → src_function` chain |
| IDSD: Drift Alerts in CI | Medium | High | `solidspec analyze --fail-on-drift 30` exits non-zero when drift exceeds threshold; designed for CI gates on long-lived IDSD features |
| TDD: Mutation Testing Integration | Medium | High | `solidspec tdd-mutate` runs a mutation testing tool (mutants, cargo-mutants, pitest) and adds the mutation score to `tdd-refactor-report.md`; strengthens the REFACTOR quality gate |
| IDSD: Intent Versioning | Medium | Medium | Track intent evolution over time (`intent-v1.md`, `intent-v2.md`), diff constraints and evidence across versions, flag when intent changes would invalidate existing FRs |
| Workspace Coordination | Medium | Medium | Multi-repo coordination with linked workspaces; cross-feature traceability for platform teams |
| Schema Marketplace | Low | Medium | Community-contributed schemas published to a registry; `solidspec schema add marketplace/react-tdd` |

---

## Workflow Summary

| Schema | Artifacts | Methodology | Best For |
|--------|-----------|-------------|----------|
| `minimal` | 4 | Lean SDD | Scripts, spikes, fully-known requirements |
| `spec-driven` | 9 | Full SDD | Most team and solo features |
| `security-first` | 5 | SDD + OWASP | Payment, auth, PII, regulated domains |
| `tdd-driven` | 10 | AI-TDD | Libraries, APIs, complex business logic |
| `intent-driven` | 11 | IDSD | Uncertain scope, compliance, long-lived features |
| `apex-driven` | 9 | SDD + APEX | Complex implementation, structured execution |
| `intent-apex` | 11 | IDSD + APEX | Enterprise, regulated, maximum rigor |

---

See [docs/idsd-workflow-guide.md](docs/idsd-workflow-guide.md) for the complete IDSD walkthrough.
See [docs/tdd/](docs/tdd/) for the TDD skill documentation.
