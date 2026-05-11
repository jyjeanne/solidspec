# Feature Specification: Multi-Agent Pipeline

**Feature ID:** 003
**Feature Name:** Multi-Agent Pipeline
**Project Name:** SolidSpec
**Version:** v0.3.0
**Status:** Draft
**Author:** jjeanne
**Date:** 2026-03-17

---

## 1. Overview

`solidspec pipeline <feature-id>` runs the full SDD workflow (specify → clarify → plan → tasks → tests → implement → analyze) with a **different AI agent per phase**. The pipeline configuration lives in `solidspec.toml` and maps each phase to an agent ID.

This is the **first multi-agent SDD tool** — no other product orchestrates multiple AI coding agents in a structured, phased development pipeline.

---

## 2. Problem Statement

Each AI coding agent has different strengths:
- **Claude Code** excels at structured text, specifications, architectural reasoning, and complex multi-file refactoring
- **Codex CLI** (OpenAI) is strong at code generation and following precise instructions
- **Copilot CLI** integrates tightly with GitHub workflows and PR-based development
- **Kimi Code** offers competitive performance for code generation tasks
- **Mistral Vibe** provides fast iteration with lightweight prompting

Today, developers use one agent for everything. They can't leverage agent-specific strengths per phase. Switching agents manually requires re-explaining context each time.

Multi-Agent Pipeline solves this by orchestrating agent switching per phase while preserving context through the shared `specs/` artifact directory.

---

## 3. User Stories

### User Story 1 — Configure pipeline agents (Priority: P1)

As a developer, I want to assign different AI agents to different SDD phases so each phase uses the best agent for the job.

**Acceptance Scenarios:**

1. **Given** `solidspec.toml` has a `[pipeline]` section mapping phases to agents, **When** running `solidspec pipeline 001`, **Then** each phase uses the configured agent
2. **Given** no `[pipeline]` section exists, **When** running `solidspec pipeline 001`, **Then** all phases use the default agent from `[ai].default_agent`
3. **Given** a pipeline maps a phase to an unknown agent ID, **When** running `solidspec pipeline 001`, **Then** an error lists available agents

### User Story 2 — Run full pipeline end-to-end (Priority: P1)

As a developer, I want to run the entire SDD pipeline with one command so I don't have to invoke each step manually.

**Acceptance Scenarios:**

1. **Given** a feature description, **When** running `solidspec pipeline --new "TODO app with CRUD"`, **Then** the pipeline runs specify → clarify → plan → tasks → tests → implement → analyze in sequence
2. **Given** an existing feature 001 with spec.md already created, **When** running `solidspec pipeline 001`, **Then** the pipeline skips specify, starts from clarify, and continues to analyze
3. **Given** a pipeline step fails (e.g., plan fails because spec has unresolved markers), **When** the failure occurs, **Then** the pipeline stops, reports which step failed and why, and suggests the next action

### User Story 3 — Preview pipeline without executing (Priority: P2)

As a developer, I want to preview the pipeline before running it so I can verify the agent assignments.

**Acceptance Scenarios:**

1. **Given** a pipeline config, **When** running `solidspec pipeline 001 --dry-run`, **Then** the output shows each phase, the assigned agent, and whether the phase would be skipped (artifact already exists)
2. **Given** a `--dry-run`, **When** displayed, **Then** no files are created or modified

### User Story 4 — Run partial pipeline (Priority: P2)

As a developer, I want to run only specific phases so I can re-run a single step with a different agent.

**Acceptance Scenarios:**

1. **Given** `--from plan` flag, **When** running `solidspec pipeline 001 --from plan`, **Then** the pipeline starts from the plan phase and continues to the end
2. **Given** `--to tasks` flag, **When** running `solidspec pipeline 001 --to tasks`, **Then** the pipeline runs from the beginning up to and including tasks, then stops
3. **Given** both `--from plan --to tasks`, **When** running, **Then** only plan and tasks phases run

### User Story 5 — Pipeline execution log (Priority: P3)

As a developer, I want a log of which agent handled which phase so I can track and audit the pipeline execution.

**Acceptance Scenarios:**

1. **Given** a completed pipeline run, **When** it finishes, **Then** a `pipeline-log.md` is created in the feature directory with timestamps, agent IDs, and pass/fail per phase
2. **Given** a second pipeline run on the same feature, **When** it finishes, **Then** the log is appended (not overwritten)

---

## 4. Requirements

### Functional Requirements

- **FR-001**: System MUST read pipeline configuration from `[pipeline]` section in `solidspec.toml`
- **FR-002**: System MUST support mapping any of these phases to an agent: `specify`, `clarify`, `plan`, `tasks`, `tests`, `implement`, `analyze`
- **FR-003**: System MUST fall back to `[ai].default_agent` for phases not explicitly mapped
- **FR-004**: System MUST validate all mapped agent IDs exist in the agent config table
- **FR-005**: System MUST execute phases in order: specify → clarify → plan → tasks → tests → implement → analyze
- **FR-006**: System MUST skip phases whose primary output artifact already exists (unless `--force` is passed)
- **FR-007**: System MUST stop on the first phase failure and report which phase failed with the error
- **FR-008**: System MUST support `--new <description>` to create a new feature and run the full pipeline
- **FR-009**: System MUST support `--from <phase>` and `--to <phase>` to run partial pipelines
- **FR-010**: System MUST support `--dry-run` to preview without executing
- **FR-011**: System MUST write a `pipeline-log.md` with timestamps, agent, and status per phase
- **FR-012**: System MUST fire extension hooks at appropriate points (`after_tasks`, `before_implement`, `after_implement`)

### Key Entities

- **[PipelineConfig]**: Mapping of phase name → agent ID, read from `solidspec.toml`
- **[PipelinePhase]**: A single step in the pipeline with: name, agent, status (pending/running/done/failed/skipped), output artifact path
- **[PipelineRun]**: A complete pipeline execution with ordered phases, feature ID, start/end timestamps, overall status

---

## 5. Configuration

### `solidspec.toml` pipeline section

```toml
[project]
name = "my_project"

[ai]
default_agent = "claude"

[pipeline]
specify = "claude"        # Claude writes structured specs
clarify = "claude"        # Claude resolves ambiguities
plan = "claude"           # Claude for architecture
tasks = "claude"          # Claude for task breakdown
tests = "claude"          # Claude for test generation
implement = "codex"       # Codex for code generation
analyze = "vibe"          # Mistral Vibe for cross-checking
```

**Rules:**
- All fields are optional — missing fields use `[ai].default_agent`
- Agent IDs must match the agent config table or aliases (`kiro` → `kiro-cli`)
- The `[pipeline]` section itself is optional — without it, `pipeline` command uses default agent for all phases

---

## 6. CLI Command

### `solidspec pipeline [feature-id]`

The `feature-id` is optional — if omitted, auto-detected from git branch or latest spec (same as other commands). Mutually exclusive with `--new`.

**Flags:**

| Flag | Description |
|------|-------------|
| `--new <description>` | Create a new feature and run full pipeline (cannot be used with `feature-id`) |
| `--from <phase>` | Start from this phase (skip earlier ones) |
| `--to <phase>` | Stop after this phase (skip later ones) |
| `--only <phase>` | Run a single phase only (shorthand for `--from X --to X`) |
| `--force` | Re-run phases even if output artifacts exist |
| `--dry-run` | Preview pipeline without executing |
| `--auto` | Skip user confirmation at handoff phases (implement) |

**Examples:**

```bash
# Full pipeline on new feature
solidspec pipeline --new "User authentication with OAuth"

# Full pipeline on existing feature
solidspec pipeline 001

# Partial: only plan and tasks
solidspec pipeline 001 --from plan --to tasks

# Preview
solidspec pipeline 001 --dry-run

# Single phase only
solidspec pipeline 001 --only plan

# Re-run everything from scratch
solidspec pipeline 001 --force

# Unattended (skip handoff confirmation)
solidspec pipeline --new "auth system" --auto
```

**Output:**

```
Pipeline: 001-user-auth

  Phase 1/7: specify (claude)
    ✓ spec.md created (1.2s)
  Phase 2/7: clarify (claude)
    ○ skipped — no [NEEDS CLARIFICATION] markers
  Phase 3/7: plan (claude)
    ✓ plan.md + research.md + data-model.md + contracts/ (2.1s)
  Phase 4/7: tasks (claude)
    ✓ tasks.md — 17 tasks across 7 phases (0.8s)
  Phase 5/7: tests (claude)
    ✓ 4 test files — 8 scenarios (0.5s)
  Phase 6/7: implement (codex) [HANDOFF]
    → Open codex and run: /solidspec-implement
    ⏳ Waiting for confirmation... (press Enter when done, or Ctrl+C to abort)
    ✓ User confirmed implementation complete
  Phase 7/7: analyze (vibe)
    ✓ Traceability: 100% — 0 issues (0.5s)

Pipeline complete: 7 phases, 3 agents (claude, codex, vibe)
Log: specs/001-user-auth/pipeline-log.md
```

---

## 7. Phase Definitions

Each phase maps to an existing SolidSpec command:

| Phase | Command | Type | Primary output | Skip condition |
|-------|---------|------|----------------|----------------|
| `specify` | `solidspec specify` | Auto | `spec.md` + `checklists/requirements.md` | spec.md exists (unless `--force`); only runs with `--new` |
| `clarify` | `solidspec clarify` | Auto | `clarifications.md` | No `[NEEDS CLARIFICATION]` markers in spec |
| `plan` | `solidspec plan` | Auto | `plan.md` + research + data-model + contracts | plan.md exists (unless `--force`) |
| `tasks` | `solidspec tasks` | Auto | `tasks.md` | tasks.md exists (unless `--force`) |
| `tests` | `solidspec tests` | Auto | `tests/` directory | tests/ dir exists with files (unless `--force`) |
| `implement` | `solidspec implement` | Handoff | source code files | Zero pending tasks in tasks.md (all `- [x]`) |
| `analyze` | `solidspec analyze` | Auto | analysis report | Never skipped — always runs |

**Notes:**
- `specify` also generates the mandatory quality checklist (`checklists/requirements.md`) as part of its execution
- `implement` skip check: parses tasks.md, counts `- [ ]` entries — if zero pending, implementation is complete
- `clarify` skip check: parses spec.md for `[NEEDS CLARIFICATION]` markers — if zero, spec is clear

---

## 8. Execution Model

The pipeline has two types of phases:

### Automated phases (SolidSpec generates artifacts directly)

These phases run without any AI agent — SolidSpec produces the artifacts itself:

| Phase | What SolidSpec does | AI agent needed? |
|-------|--------------------|-----------------|
| `specify` | Generates spec.md from template + checklist | No |
| `clarify` | Generates clarification questions | No |
| `plan` | Generates plan.md + research + data-model + contracts | No |
| `tasks` | Generates tasks.md from spec + plan | No |
| `tests` | Generates test scaffolds from acceptance scenarios | No |
| `analyze` | Runs consistency analysis, produces report | No |

### Handoff phases (require AI agent action)

These phases pause the pipeline and tell the user what to do:

| Phase | What SolidSpec does | What the user does |
|-------|--------------------|--------------------|
| `implement` | Prints pending tasks and target agent | Opens the configured agent and runs `/solidspec-implement` |

When the pipeline reaches a handoff phase:
1. It prints `→ Handoff to <agent>: run /solidspec-implement in your <agent> session`
2. The pipeline pauses and waits for user confirmation (or `--auto` to skip confirmation)
3. After the user confirms completion, the pipeline continues to the next phase

### Agent assignment meaning

The agent configured per phase determines:
- **For automated phases:** Which agent's command directory receives the updated AGENT.md context (so the agent can reference it later)
- **For handoff phases:** Which agent the user is instructed to switch to

### Context sharing

All agents share context through the `specs/<feature>/` artifact directory. Each phase reads from and writes to this directory. The `.solidspec/AGENT.md` file provides project-wide context to every agent.

### Future: Direct agent spawning (v0.4.0)

For CLI-based agents (Claude Code, Gemini CLI), a future version could spawn the agent process directly using `std::process::Command` to fully automate handoff phases. This is out of scope for v0.3.0.

---

## 9. Pipeline Log Format

`specs/<feature>/pipeline-log.md`:

```markdown
# Pipeline Log: 001-user-auth

## Run 2026-03-17T14:30:00Z

| Phase | Agent | Status | Duration | Output |
|-------|-------|--------|----------|--------|
| specify | claude | done | 1.2s | spec.md |
| clarify | claude | skipped | — | no markers |
| plan | claude | done | 2.1s | plan.md, research.md, data-model.md |
| tasks | claude | done | 0.8s | tasks.md (17 tasks) |
| tests | claude | done | 0.5s | 4 test files |
| implement | codex | handoff | user-confirmed | user ran /solidspec-implement in codex |
| analyze | vibe | done | 0.5s | 100% traceability |

**Total:** 5.1s (automated) | **Agents:** claude, codex, vibe | **Status:** complete
```

---

## 10. Implementation Plan

### Phase 1: Pipeline Config

- Add `PipelineConfig` struct with optional agent mapping per phase
- Add `[pipeline]` section to `RootConfig` (serde deserialization)
- Validate agent IDs against config table on load
- Fall back to `default_agent` for unmapped phases

### Phase 2: Pipeline Executor

- Define the 7 phases in order with skip conditions
- Execute each phase by calling the existing CLI command logic
- Track timing, status (done/skipped/failed), and output artifacts
- Stop on first failure with clear error message

### Phase 3: CLI Command

- Add `pipeline` subcommand with `--new`, `--from`, `--to`, `--force`, `--dry-run`
- Wire `--new` to call `specify` with the description, then continue pipeline
- Implement `--from` / `--to` range filtering
- Implement `--dry-run` preview output

### Phase 4: Pipeline Log

- Generate `pipeline-log.md` in the feature directory
- Append to existing log (multiple runs)
- Include timestamps, agents, durations, outputs, status

---

## 11. Development Tasks

| Task | Description |
|------|-------------|
| T001 | Add `PipelineConfig` struct to `config/mod.rs` with per-phase agent mapping |
| T002 | **Test:** Parse `[pipeline]` from TOML, missing section → empty config, invalid agent → error |
| T003 | Implement pipeline executor: ordered phase execution with skip conditions |
| T004 | **Test:** All phases run in order; existing artifact → skipped; `--force` → re-run; failure → stop |
| T005 | Implement agent switching: resolve agent per phase from config, log transitions |
| T006 | **Test:** Mixed agents config → correct agent logged per phase; missing mapping → default used |
| T007 | Add `pipeline` CLI subcommand with all flags |
| T008 | **Test:** `--dry-run` shows preview; `--from plan --to tasks` runs only 2 phases; `--new` creates feature first |
| T009 | Implement pipeline log generation (`pipeline-log.md`) |
| T010 | **Test:** Log created after run; second run appends; log contains timestamps, agents, durations |
| T011 | Wire phase execution to existing command logic (specify, clarify, plan, tasks, tests, implement, analyze) |
| T012 | **Test:** Full pipeline E2E: `--new "test"` → all artifacts generated → log present |
| T013 | Implement handoff phase: pause for user confirmation, `--auto` to skip |
| T014 | **Test:** Handoff pauses without `--auto`; `--auto` skips confirmation; log records "handoff" status |
| T015 | Implement `--only` flag as shorthand for `--from X --to X` |
| T016 | **Test:** `--only plan` runs only plan phase; `--only` with invalid phase → error |

---

## 12. Edge Cases

- **No agents installed** — pipeline fails at first phase with "no agent detected" error
- **Agent directory removed mid-pipeline** — phase fails, pipeline stops, log records failure
- **Feature already fully implemented** — all phases skipped except analyze (which always runs)
- **`--new` with empty description** — error before pipeline starts
- **`--from` phase that doesn't exist** — error listing valid phase names
- **`--from` after `--to`** — error "from phase must come before to phase"
- **Pipeline interrupted (Ctrl+C)** — partial log written, incomplete phases marked as "interrupted"
- **`--auto` with handoff phase** — implement runs without user confirmation, pipeline assumes tasks will be done by the agent and continues to analyze
- **`--new` combined with `feature-id`** — error "cannot use --new with a feature ID"
- **No `[ai].default_agent` and no `[pipeline]` section** — falls back to "claude" as hardcoded default

---

## 13. Non-Goals

- NOT spawning AI agent processes directly (v0.3.0 is orchestration only — agents run in IDE/terminal)
- NOT sharing conversation context between agents (each agent reads from `specs/` artifacts)
- NOT supporting parallel phase execution (phases are sequential by design)
- NOT implementing agent-specific prompts per phase (all agents use the same command templates)
- NOT providing real-time streaming of agent output (pipeline waits for each phase to complete)

---

## 14. Acceptance Criteria

Feature is complete when:

- `solidspec pipeline --new "description"` runs the full 7-phase pipeline
- `solidspec pipeline 001` resumes from where artifacts left off
- `[pipeline]` config in `solidspec.toml` correctly maps phases to agents
- Unmapped phases fall back to `[ai].default_agent`
- `--from`, `--to`, `--force`, `--dry-run` flags work correctly
- `pipeline-log.md` is generated with timestamps, agents, and status
- At least 16 unit tests covering config, executor, CLI, log, handoff, and partial runs
- Extension hooks fire at appropriate points

---

## 15. Success Criteria

- Developers can leverage multiple AI agents' strengths in a single workflow
- Pipeline reduces manual context-switching between agents
- Full pipeline runs in under 2 minutes for a typical feature
- Pipeline log provides auditable record of which agent did what
- No other SDD tool offers multi-agent orchestration
