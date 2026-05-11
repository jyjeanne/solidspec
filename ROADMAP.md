# SolidSpec Roadmap

## Current: v0.3.0

### Implemented

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
| Schema-Driven Workflows | ✅ | 3 built-in schemas (spec-driven, minimal, security-first), YAML-customizable |
| Change-Based Workflow | ✅ | Delta specs (ADDED/MODIFIED/REMOVED), propose → list → archive lifecycle |
| OpenCode Skills | ✅ | Directory-based `.opencode/skills/` with `name:` + `description:` SKILL.md format |
| Agent Timeout | ✅ | 300s `try_wait()` polling loop, process killed on timeout |

---

## Next: v0.4.0

### Planned

| Priority | Feature | Est. Effort | Why |
|----------|---------|-------------|-----|
| **HIGH** | Parallel Fan-Out Orchestration | 12h | Concurrent review (code + security + test + perf), ship decision |
| **HIGH** | Doubt-Driven Development | 16h | In-flight adversarial review (3-cycle bounded) catches problems during implementation |
| **MEDIUM** | Spec Import from Issues | 8h | `solidspec import --github 42` — pre-fill spec from GitHub Issues / Jira |
| **LOW** | Shell Completions Enhancement | 4h | Install completions via `solidspec completions install <shell>` |
| **LOW** | MSRV Declaration | 1h | Declare Minimum Supported Rust Version in Cargo.toml |

---

## Future: v1.0.0

### Backlog

| Feature | Difficulty | Impact | Description |
|---------|-----------|--------|-------------|
| Interactive TUI Builder | Medium | High | `ratatui`-based guided spec creation with real-time quality scoring |
| Live Traceability Matrix | High | Very High | Scan source code → requirement-to-code traceability map (tree-sitter AST) |
| Spec Drift Detection | High | High | Detect when implementation diverges from spec over time |
| AI-Powered Spec Review | Very High | High | Send spec to AI agent for structured quality review with scoring |
| Workspace Coordination | Medium | Medium | Multi-repo coordination with linked workspaces |

---

## Full Feature Comparison

See [ADD_NEW_KILLER_FEATURES_PLAN.md](../ADD_NEW_KILLER_FEATURES_PLAN.md) for detailed competitive analysis vs agent-skills and OpenSpec.

See [docs/KILLER_FEATURE_IDEAS.md](KILLER_FEATURE_IDEAS.md) for original feature brainstorming with implementation details.
