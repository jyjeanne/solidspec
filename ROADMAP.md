# SolidSpec Roadmap

## Current: v0.3.1

SolidSpec has evolved from a single-methodology SDD tool into a **multi-methodology AI development platform**. Seven built-in schemas cover the full spectrum from lightweight spikes to fully-traced, intent-anchored, TDD-enforced production workflows — backed by a native, git-friendly knowledge graph of your own codebase that keeps `analyze` honest about what actually exists.

### Unreleased — CLI Simplification & Native Knowledge Graph

Two threads of work landed on top of v0.3.1, not yet cut as a tagged release: making the CLI's common case flagless (inspired by a comparison against [OpenSpec](https://github.com/Fission-AI/openspec) — see `docs/simplification-study-openspec.md`), and giving SolidSpec its own native, in-process knowledge graph of the codebase it's scaffolding (see `docs/okf-rs-integration-plan.md` and `docs/kg-workflow-vision-gap-analysis.md`).

**CLI simplification**

| Change | Description |
|--------|-------------|
| `solidspec go "desc"` / `solidspec continue [id]` | Flagless shortcuts for the common case — `go` wraps `pipeline --new "desc" --auto`, `continue` wraps `pipeline --auto` on whatever's next. Both run on the project's own default schema. |
| Per-phase commands hidden from `--help` | 14 subcommands (`specify`, `plan`, `implement`, `apex`, `tdd-tests`, ...) still work exactly as before but no longer clutter `solidspec --help` — the top-level surface is now `init`/`go`/`continue`/`status`/`schemas`/`pipeline`/`ship`/`okf`/... |
| `solidspec schemas` | One command listing all 7 workflow schemas with version, source, use case, artifact count, and each one's `/spcx:<short>:*` slash commands — replaces reading a README table before running anything. |
| `--schema` on `solidspec init`, persisted | Recorded in `solidspec.toml`'s `[pipeline].schema`. `go`/`continue`/`status`/`tasks`/`pipeline` all resolve an unset `--schema` flag to this stored default instead of each independently hardcoding `spec-driven`. |
| `minimal` is the default schema | Omitting `--schema` on `init` now scaffolds the lean 4-artifact workflow rather than the full 9-artifact one — start light, opt into more ceremony explicitly. |
| Schema-aware `/spcx:*` commands | `/spcx:<short>:{new,apply,finalise}` — reduced-schema-name namespaced, one per built-in schema (`/spcx:tdd:apply`, `/spcx:sec:new`, ...), no flagless default-schema shortcut — generated straight from each schema's own DAG (`src/agents/spcx.rs`), so the body always matches what that schema actually does. A fully custom-named schema gets `/spcx:<name>:*` too, resolved by its actual identifier (the `.solidspec/workflows/<name>/` directory / `--schema` value) rather than a `schema.yaml`'s own possibly-stale internal `name:` field, which could otherwise disagree and silently drop its commands entirely. Its short name colliding with a built-in's (checked case-insensitively, since `.claude/commands/spcx/<short>/` is a real directory on case-insensitive filesystems like macOS/Windows) fails registration loudly instead of silently overwriting it; the not-yet-wired `unregister_commands` cleans up a custom schema's files symmetrically with whatever was registered for it. |
| `/spcx:<short>:apply` reminds the agent to refresh the knowledge graph | Unlike `go`/`continue`/`pipeline` (which refresh an existing bundle in-process right after a code-writing handoff), the interactive `/spcx:*` flow has the agent edit files directly with no CLI subprocess call in between — its generated `apply` body now tells the agent to run `solidspec okf generate` itself, so a `finalise` → `analyze` run right after doesn't flag the agent's own new code as "orphaned" against a now-stale graph. |
| "Next: solidspec X" hints | Every phase-producing command ends with what to run next, computed from the artifact graph's own topological order rather than hardcoded per command. |
| `analyze` is primary again | After a dedicated study of `check` vs `validate` vs `analyze`'s actual purposes, the earlier `validate` rename was reverted — `analyze` is the canonical name, `validate` remains a `clap` alias. |
| Mistral Vibe removed, OpenCode promoted | 19 supported agents (from 20) — Vibe had no meaningful adoption signal; OpenCode's directory-based skills format is now a first-class example throughout the docs. |
| README reorganized | Pitch → install → quick example → full reference, OpenSpec-style, instead of reference-first. |

**Native knowledge graph (OKF)**

| Change | Description |
|--------|-------------|
| `solidspec okf generate` / `validate` | [okf-rs](https://github.com/jyjeanne/okf-rs)'s generator/analyzer/validator crates vendored as pinned git dependencies and wrapped in-process (`src/core/okf.rs`) — **no external `okf-rs` binary**. Extraction is local tree-sitter AST parsing across 11 languages; nothing leaves the machine. Output is a plain Markdown+YAML bundle, one file per concept, committable and diffable like any other file. |
| Auto-generation on `init` | An existing codebase (anything already in the target directory) gets `.solidspec/knowledge/` generated automatically and registered as an `okf` MCP server in `.mcp.json` — an empty/fresh directory skips this; `solidspec okf generate` remains available on demand. |
| Structural cross-check in `analyze` | Cross-checks `tasks.md`'s backtick-quoted symbols and referenced source files against the bundle (`core::okf::BundleIndex`, reads the bundle back via `okf_parser::read_bundle` — no re-analysis, no external `search`/`explore`/`okf-mcp`), rendered as its own "Structural cross-check (okf-rs)" report section. Catches an orphaned/hallucinated reference a purely textual read of `tasks.md` can't. |
| Post-handoff refresh loop | `implement`, `apex`, `tdd-tests`, and `tdd-refactor` all regenerate an already-existing bundle in place right after their confirmation step — the point where the AI agent has just finished changing code — so the graph never goes stale for any schema that changes code, not only `implement`. Never creates a bundle where none existed. |
| `docs/kg-workflow-vision-gap-analysis.md` | Full architecture review comparing the codebase against a "knowledge graph answers what's true, DAG answers what's next, connected via MCP" vision — what's already in place, what's a real gap, and a prioritized recommendation list (2 of 3 near-term items already shipped above). |

### v0.3.1 — Security-First Fixes

| Fix | Description |
|-----|-------------|
| `security-review` executor | `solidspec pipeline --schema security-first --no-agent` previously failed with `Unknown phase: security-review` — no executor existed for the phase. Added `solidspec security-review`, backed by a no-agent OWASP Top 10 heuristic audit of `plan.md`/`spec.md` (`core::security_review`). |
| Heuristic false positives fixed | Several OWASP heuristic regexes had a truncated-stem word boundary bug (e.g. `tokeniz\b` never matched "tokenized") that produced false-positive Critical findings on plans that already documented the mitigation. Verified fixes against the `regex` crate directly; added regression tests. |
| Agent-mode prompt parity | Live-agent (non-`--no-agent`) `security-review` runs were falling through to a generic prompt instead of the detailed OWASP-audit instructions; added a dedicated `security-review` prompt arm and `Security Auditor` persona. |
| `tasks` DAG gate now enforced | `solidspec tasks` previously only checked `plan.md`'s existence and never consulted the schema graph, so calling it directly (bypassing `pipeline`) could silently skip the security-first schema's `security-review` → `tasks` dependency. `solidspec tasks` now accepts `--schema` and blocks with a clear error until the required artifact exists — same behavior `solidspec status` already displayed. |

---

### Implemented — Core Infrastructure

| Feature | Status | Description |
|---------|--------|-------------|
| DAG Artifact Graph | ✅ | Kahn's algorithm topological sort, completion detection, `solidspec status` |
| Schema-Driven Workflows | ✅ | 7 built-in schemas (YAML-customizable), 3-level resolution (project-local → built-in → default), `minimal` as the actual `init` default |
| Multi-Agent Support (19) | ✅ | Auto-detection, format translation, slash command registration per agent |
| Schema-Aware `/spcx:*` Commands | ✅ | `/spcx:<short>:new`/`apply`/`finalise`, reduced-schema-name namespaced per built-in schema, generated from each schema's own DAG (`src/agents/spcx.rs`) |
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

### Implemented — Native Knowledge Graph (OKF)

Zero external binary — [okf-rs](https://github.com/jyjeanne/okf-rs)'s generator/analyzer/validator/parser crates are vendored as pinned git dependencies and wrapped in-process (`src/core/okf.rs`). See `docs/okf-rs-integration-plan.md` for the full integration history and `docs/kg-workflow-vision-gap-analysis.md` for what's still a gap against the broader vision.

| Feature | Status | Description |
|---------|--------|-------------|
| `solidspec okf generate` / `validate` | ✅ | Native tree-sitter extraction (11 languages) → a plain Markdown+YAML bundle, one file per concept, cross-linked and diffable in a PR. Incremental cache (`.okf-cache.json`) re-parses only changed files. |
| Auto-generation on `init` | ✅ | An existing codebase gets `.solidspec/knowledge/` and a registered `okf` MCP server (`.mcp.json`) automatically; a fresh empty directory skips it. |
| Structural cross-check | ✅ | `solidspec analyze` flags a `tasks.md` symbol/file reference absent from the bundle, as its own report section — never blocks, never merges into the textual heuristics. |
| Post-handoff refresh loop | ✅ | Bundle regenerates automatically after `implement`/`apex`/`tdd-tests`/`tdd-refactor`, only when one already exists — closes the "graph goes stale the moment code changes" gap. |
| `search`/`explore`/`graph`/`impact`/`diff` | External CLI only | Not vendored (would pull in tantivy, an active LSP client, a PDF renderer) — `cargo install --git https://github.com/jyjeanne/okf-rs okf-cli` for these. |
| Native MCP server | 📋 Planned | `.mcp.json`'s `okf` entry still points at an external, unvendored `okf-mcp` binary and only covers Claude Code — see Next below. |
| Fact/decision/inference provenance | 📋 Planned (upstream) | Every OKF fact today is `confidence: exact` tree-sitter output; distinguishing fact from decision from hypothesis needs new `okf-rs` concept kinds — an upstream discussion, not a SolidSpec-only change. |

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
| `security-review` Command | ✅ | `solidspec security-review [id] [--dry-run]` runs a no-agent OWASP Top 10 heuristic audit of `plan.md`/`spec.md` and writes `security-review.md`; wired into `solidspec pipeline`'s `security-review` phase and registered as a slash command per agent. |
| OWASP Audit Gate | ✅ | Security findings by severity (Critical/High/Medium/Low); every finding becomes a mitigation task |
| DAG Gate Enforcement | ✅ | `solidspec tasks --schema security-first` consults the schema graph and blocks until `security-review.md` exists (previously only `solidspec status` displayed the gate; `tasks` itself didn't enforce it) |

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
| **MEDIUM** | Native MCP registration per agent | 6h | Extend `AGENTS` (`src/agents/config.rs`) with a `supports_mcp` field and write a real, per-agent MCP config — today `.mcp.json`'s `okf` entry only covers Claude Code and points at an external, unvendored `okf-mcp` binary. See `docs/kg-workflow-vision-gap-analysis.md` §1 / `docs/okf-rs-integration-plan.md` step 3. |
| **MEDIUM** | `evidence` → code source link | 6h | Parse implemented test files to extract which source files they exercise; add `test → src` layer to the IDSD traceability chain, closing the last gap in `INT → FR → T → test → src`. The OKF bundle's file/symbol index (`core::okf::BundleIndex`) is already in place as the lookup this would build on. |
| **MEDIUM** | TDD: Cycle Progress Tracking | 4h | `solidspec tdd-status` shows RED/GREEN/REFACTOR progress: tests written vs. passing, refactor candidates resolved vs. pending. |
| **LOW** | Structural impact report in `review`/`ship` | 6h | Optional lane surfacing blast-radius of changed functions from the OKF bundle, attached to `review-report.md`/`ship-report.md` — particularly useful for `security-first`'s audit. See `docs/okf-rs-integration-plan.md` step 6. |
| **LOW** | Shell Completions Enhancement | 4h | `solidspec completions install <shell>` — one-command install that writes to the correct profile file. |
| **LOW** | MSRV Declaration | 1h | Declare Minimum Supported Rust Version in `Cargo.toml`. |

Also tracked, but deliberately not SolidSpec's alone to schedule: **fact/decision/inference OKF provenance** (`docs/kg-workflow-vision-gap-analysis.md` §3) needs new concept kinds in `okf-rs` itself — an upstream discussion, not a local task with an effort estimate.

---

## Future: v1.0.0

### Backlog

| Feature | Difficulty | Impact | Description |
|---------|-----------|--------|-------------|
| Interactive TUI Builder | Medium | High | `ratatui`-based guided workflow: spec + intent creation with real-time quality scoring. Workflow selector shows the comparison matrix and recommends a schema based on answers. |
| Live Traceability Matrix (AST) | Medium (was Very High) | Very High | Map requirement IDs to actual code locations, closing the last gap in the full `INT → FR → T → test → src_function` chain. The tree-sitter AST scan this needs already exists as the native OKF knowledge graph (`core::okf`) — this is now "link FR-### to a `BundleIndex` symbol" rather than building an extractor from scratch. |
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
See [docs/okf-rs-integration-plan.md](docs/okf-rs-integration-plan.md) for the native knowledge-graph integration history and what's still external-CLI-only.
See [docs/kg-workflow-vision-gap-analysis.md](docs/kg-workflow-vision-gap-analysis.md) for the full architecture review behind the "Next" items above.
See [docs/simplification-study-openspec.md](docs/simplification-study-openspec.md) for the UX study behind the CLI-simplification changes.
