# Feature Specification

**Feature ID:** 001
**Feature Name:** Rust Spec‑Driven Development CLI
**Project Name:** SolidSpec
**Version:** v1.0
**Status:** Implemented
**Author:** jjeanne
**Date:** 2026‑03‑16

---

## 1. Overview

**SolidSpec** is a **command‑line tool** written in **Rust** that enables **Specification‑Driven Development (SDD)** workflows. It transforms high-level feature descriptions into structured, executable development artifacts:

- Project constitutions (governance principles)
- Feature specifications
- Architecture plans with supporting documents
- Executable task breakdowns
- Quality validation checklists

The structured development pipeline:

```text
constitution → specify → clarify → plan → tasks → implement
```

SolidSpec embeds specification workflows directly into development repositories, treating **specifications as the source of truth** and code as their expression. This inverts the traditional relationship where specs serve code — instead, code serves specs.

### Why SDD Matters

Software development suffers from a persistent gap between intent and implementation. Specifications rot, plans diverge from code, and documentation lags behind reality. SDD eliminates this gap by making specifications executable — when specs generate implementation plans, and plans generate task lists, there is no gap, only transformation.

AI capabilities now make this practical: natural language specifications can reliably drive structured code generation. But raw AI generation without structure produces chaos. SolidSpec provides that structure through templates that constrain AI output toward consistency, completeness, and quality.

---

## 2. Problem Statement

Software development often begins with loosely defined requirements and unstructured planning, leading to:

- Unclear scope and incomplete feature definitions
- Poorly organized tasks with no traceability to requirements
- Inconsistent documentation that diverges from code
- Difficult onboarding and knowledge transfer
- AI-assisted development that produces plausible but unchecked output

Current project management tools separate planning from the codebase. SolidSpec bridges that gap by embedding specification workflows into development repositories and using templates to enforce quality at every stage.

---

## 3. Goals

SolidSpec aims to:

1. Provide a CLI for managing the full SDD lifecycle (constitution → specify → plan → tasks → implement)
2. Standardize specifications across teams via structured templates
3. Auto‑generate architecture plans with supporting documents (research, data models, API contracts)
4. Convert plans into prioritized, parallelizable, story‑driven task breakdowns
5. Enforce quality through constitution checks, clarification markers, and validation checklists
6. Support multiple AI agents (Claude, Gemini, Copilot, Cursor, etc.) via agent‑specific command registration
7. Enable customization through a preset and extension system
8. Integrate with Git workflows (branch creation, feature numbering)

---

## 4. Non‑Goals

SolidSpec will **not**:

- Replace full project management platforms (Jira, Linear, etc.)
- Auto‑generate entire production systems or execute code
- Manage deployments, CI/CD, or build systems
- Replace external build/test tools
- Implement an AI model or inference engine — it generates prompts and templates for external AI agents

Focus remains on **planning, specification management, and AI‑agent orchestration**.

---

## 5. Target Users

**Primary:**

- Software engineers using AI coding assistants (Claude Code, Gemini CLI, Cursor, etc.)
- Engineering teams wanting structured specification workflows
- Open‑source contributors and project maintainers
- Students learning structured software engineering

**Secondary:**

- Technical leads defining project governance
- Product engineers translating requirements to specifications
- Teams adopting SDD methodology

---

## 6. Development Workflow

SolidSpec supports this 6‑step workflow:

```text
solidspec init                          → Project bootstrap + constitution
solidspec specify <feature-name>        → Feature specification from description
solidspec clarify <feature-id>          → Resolve ambiguities in spec
solidspec plan <feature-id>             → Architecture plan + supporting docs
solidspec tasks <feature-id>            → Story-driven task breakdown
solidspec implement <feature-id>        → Execute tasks (via AI agent)
```

Supporting commands:

```text
solidspec analyze <feature-id>          → Cross-artifact consistency validation
solidspec checklist <feature-id>        → Quality validation checklist
solidspec check                         → Verify system prerequisites
solidspec upgrade                       → Refresh templates/scripts after binary update
solidspec preset <subcommand>           → Manage workflow presets
solidspec extension <subcommand>        → Manage extensions
```

Each step produces structured artifacts in the repository under `specs/<feature-id>-<feature-name>/`.

---

## 7. CLI Interface

SolidSpec follows established CLI design conventions:

- Subcommand‑based architecture (clap derive API)
- Clear help pages with examples
- Consistent error reporting with context
- Colored terminal output
- Predictable flags and input

### Global Flags

| Flag | Description |
|------|-------------|
| `--ai <agent>` | Target AI agent (claude, gemini, copilot, cursor, vibe, etc.) |
| `--ai-commands-dir <path>` | Custom agent command directory (for unsupported agents) |
| `--script <sh\|ps>` | Shell script type (POSIX shell or PowerShell) |
| `--debug` | Verbose output |
| `--config <path>` | Custom config file path |
| `--github-token <token>` | GitHub API token (overrides env vars) |

### Example Usage

```bash
solidspec init my-project
solidspec specify "Real-time chat with message history and presence"
solidspec clarify 003
solidspec plan 003
solidspec tasks 003
solidspec implement 003
```

---

## 8. CLI Commands

### 8.1 `solidspec init [project-name]`

Initializes project structure and governance.

**Flags:**

| Flag | Description |
|------|-------------|
| `--here` | Initialize in current directory (instead of creating a subdirectory) |
| `--no-git` | Skip Git repo initialization |
| `--ai <agent>` | Pre‑select AI agent |
| `--ai-skills` | Install AI skills in addition to commands (requires `--ai`) |
| `--script <sh\|ps>` | Pre‑select script type |
| `--force` | Skip confirmation prompts |

When no `project-name` is given and `--here` is not set, initializes in the current directory by default.

**Creates:**

```text
.solidspec/
├── constitution.md          # Project governance principles
├── templates/               # Core artifact templates
│   ├── spec-template.md
│   ├── plan-template.md
│   ├── tasks-template.md
│   ├── checklist-template.md
│   └── constitution-template.md
├── scripts/                  # Shell helper scripts (bash + powershell)
├── presets/                  # Installed presets
│   └── .registry             # Preset registry (JSON)
├── extensions/               # Installed extensions
│   ├── .registry             # Extension registry (JSON)
│   └── .cache/catalogs/      # Catalog cache (per-URL SHA256 hash)
├── init-options.json         # Persisted init choices
├── AGENT.md                  # Agent context file
└── config.toml               # Project configuration
specs/
solidspec.toml                # Root configuration
```

**Additional actions:**

- Initializes Git repo if missing (unless `--no-git`)
- Creates initial commit ("Initial commit from SolidSpec template")
- Generates default constitution from template (preserves existing on re‑init)
- Detects installed AI agents and registers commands in each agent's format
- Persists user choices to `.solidspec/init-options.json` for downstream operations:
  ```json
  { "ai_assistant": "claude", "script_type": "sh", "installed_at": "2026-03-14T12:34:56Z" }
  ```
- Sets executable permissions on `.sh` scripts (Unix only, checks shebang)
- Merges `.vscode/settings.json` if present (deep merge, preserves existing keys)

---

### 8.2 `solidspec specify <feature-name>`

Creates a new feature specification from a description.

**Behavior:**

1. Generates short branch name from description (2–4 words, action‑noun format)
2. Scans existing specs to determine next feature number using **global branch numbering** (not per‑spec‑directory — the next number is the highest existing number + 1 across all specs)
3. Creates a Git branch named `<number>-<short-name>`
4. Generates spec from template with structured sections
5. Runs **mandatory** quality validation checklist (auto‑generates `checklists/requirements.md`)
6. Iterates up to 3 times to fix validation failures
7. Validates table formatting renders correctly in Markdown preview

**Creates:**

```text
specs/
└── 003-chat-system/
    ├── spec.md
    └── checklists/
        └── requirements.md    # Quality validation result (mandatory)
```

**Spec template produces:**

- **User Scenarios & Testing** — prioritized stories (P1, P2, P3) with Given/When/Then acceptance scenarios, each independently testable
- **Functional Requirements** — numbered (FR‑001…) with `[NEEDS CLARIFICATION]` markers for ambiguities (**max 3 markers** — if more ambiguities exist, they must be inferred or the feature description is too vague)
- **Key Entities** — domain objects and relationships (no implementation details)
- **Edge Cases** — boundary conditions and error scenarios
- **Success Criteria** — measurable, **technology‑agnostic** outcomes (SC‑001…) — e.g., "users complete task in under 2 minutes", NOT "API response under 200ms"

**Quality validation checks:**

- Content quality — no implementation details, business‑focused, non‑technical writing
- Requirement completeness — testable, measurable, technology‑agnostic
- Feature readiness — acceptance criteria present, edge cases defined, bounded scope
- No speculative or "might need" features

**Key constraint:** The spec template enforces **WHAT users need and WHY**, explicitly forbidding **HOW to implement** (no tech stack, APIs, or code structure).

---

### 8.3 `solidspec clarify <feature-id>`

Resolves ambiguities in a specification.

**Behavior:**

1. Reads spec.md and identifies `[NEEDS CLARIFICATION]` markers
2. Generates structured clarification questions (max 5 per session), organized by coverage area
3. Presents each question with options in a structured format:
   | Option | Answer | Implications |
   |--------|--------|--------------|
4. **Atomically** updates spec.md after each resolved question (saves to disk per integration, not batched at end)
5. Normalizes terminology across the spec — retains original only with `(formerly referred to as "X")` annotation once
6. Validates no vague placeholders remain after write
7. Produces `clarifications.md` with session markers (`## Clarifications` / `### Session YYYY-MM-DD`)

**Output:**

```text
specs/003-chat-system/
├── spec.md               # Updated with resolved clarifications (saved atomically)
└── clarifications.md     # Decision log with dated sessions
```

---

### 8.4 `solidspec plan <feature-id>`

Generates an architecture plan from a specification.

**Behavior:**

1. Reads spec.md and constitution.md
2. **Prerequisite:** ALL `[NEEDS CLARIFICATION]` markers in spec.md must be resolved before planning begins (run `clarify` first if any remain)
3. Performs **first** constitution compliance check (quality gates)
4. Executes in phases:
   - **Phase 0:** Research — produces `research.md` (technology investigation)
   - **Phase 1:** Design — produces `plan.md`, `data-model.md`, `quickstart.md`, `contracts/`; updates `.solidspec/AGENT.md` with new context
   - **Phase 2:** Report — STOP and report completion (no implementation)
5. Performs **second** constitution compliance check post‑design (re‑evaluate gates after architecture decisions)

**Creates:**

```text
specs/003-chat-system/
├── plan.md               # Architecture plan
├── research.md           # Technology investigation (Phase 0)
├── data-model.md         # Entity definitions and relationships (Phase 1)
├── quickstart.md         # Key validation scenarios (Phase 1)
└── contracts/            # API specifications (Phase 1)
    ├── rest-api.md
    └── websocket-events.md
```

**Plan template produces:**

- **Summary** — primary requirement + technical approach from research
- **Technical Context** — language, dependencies, storage, testing, platform, performance goals, constraints
- **Constitution Check** — pre‑implementation quality gates (evaluated **twice**: before research and after design):
  - Simplicity Gate — ≤3 projects? No future‑proofing?
  - Anti‑Abstraction Gate — using framework directly? Single model representation?
  - Integration‑First Gate — contracts defined? Contract tests planned?
- **Project Structure** — concrete directory layout for this feature
- **Complexity Tracking** — justified violations of constitution principles

---

### 8.5 `solidspec tasks <feature-id>`

Generates a story‑driven task breakdown from the plan.

**Inputs:** plan.md (required), spec.md (required for user stories), data-model.md, contracts/, research.md

**Hook lifecycle:** Fires `after_tasks` extension hooks AFTER tasks.md is written but before returning to user.

**Creates:**

```text
specs/003-chat-system/
└── tasks.md
```

**Task format:** `- [ ] T### [P?] [Story?] Description with file path`

- Checkbox prefix (`- [ ]`) required — tasks are checkable and marked `[X]` when complete
- **T###** — zero‑padded 3‑digit sequential ID (T001, T002, T003…)
- **[P]** — marks tasks that can run in parallel; ONLY for tasks affecting **different files** with **no inter‑task dependencies**
- **[Story]** — links task to a user story (US1, US2…); required ONLY in User Story phases, NOT in Setup/Foundational/Polish phases
- Exact file paths included in descriptions

**Phase structure:**

1. **Setup** — project initialization, dependencies
2. **Foundational** — blocking prerequisites (MUST complete before any story)
3. **User Story phases** — one phase per story (P1, P2, P3…), each independently testable
4. **Polish** — cross‑cutting concerns, documentation, optimization

**Execution strategies:**

- **MVP First** — complete only P1 story, validate, ship
- **Incremental Delivery** — add stories one at a time
- **Parallel Team** — multiple developers on parallel stories after foundational phase

---

### 8.6 `solidspec implement <feature-id>`

Executes tasks via the configured AI agent.

**Flags:**

| Flag | Description |
|------|-------------|
| `--pass <N>` | Multi‑pass implementation (for complex features requiring iterative refinement) |

**Behavior:**

1. **Blocking checklist check** — if `checklists/requirements.md` has unchecked items, prompts user "Do you want to proceed anyway?" before continuing
2. Fires `before_implement` extension hooks
3. Parses tasks.md, respects task dependencies and `[P]` parallel markers
4. Sends tasks to the configured AI agent as structured prompts
5. Writes generated code directly to project files
6. Marks completed tasks as `[X]` in tasks.md
7. Detection‑driven ignore file creation — only creates `.dockerignore`, `.eslintignore`, etc. if corresponding tools are detected in the project
8. Fires `after_implement` extension hooks
9. Logs execution results
10. Multi‑pass mode (`--pass N`) re‑runs implementation with increasing detail/correction

---

### 8.7 `solidspec analyze <feature-id>`

Validates cross‑artifact consistency. **Read‑only** — does not modify files, but offers optional remediation suggestions.

**Checks:**

- All spec requirements traced to plan sections
- All plan components traced to tasks
- No orphaned tasks without plan backing
- Constitution compliance maintained

**Severity heuristic:**

| Severity | Examples |
|----------|----------|
| **CRITICAL** | Constitution MUST violations, missing core spec sections, zero‑coverage requirements. Constitution conflicts are **automatically CRITICAL** and non‑negotiable. |
| **HIGH** | Requirement duplication, conflicting definitions, ambiguous security requirements |
| **MEDIUM** | Terminology drift between artifacts, missing non‑functional tasks |
| **LOW** | Style/wording inconsistencies |

**Limits:** Maximum **50 findings** reported; overflow aggregated in summary. Offers optional remediation for each finding.

---

### 8.8 `solidspec checklist <feature-id>`

Generates a quality validation checklist. Checklists are **"unit tests for requirements"** — they validate specification quality, NOT implementation correctness.

**Flags:**

| Flag | Description |
|------|-------------|
| `--append` | Append to existing checklist (continues numbering from last CHK ID, e.g., CHK015 → CHK016) |

**Checklist areas:**

- Content quality (no impl details in spec, testable requirements)
- Requirement completeness (no remaining `[NEEDS CLARIFICATION]` markers)
- Feature readiness (all stories have acceptance criteria)
- Specification quality (bounded scope, no speculative features)

**Item format:** `- [ ] CHK### <question> [dimension] [reference]`

**Question patterns:**

- **Required:** "Are [requirement type] defined…", "Is [vague term] quantified…", "Can [requirement] be measured…"
- **Prohibited:** "Verify", "Test", "Confirm", "Check" + implementation behavior (checklists validate requirements, not code)

**Limits:** Soft cap of **40 items** per checklist; if >40 candidates, prioritize by risk/impact and merge near‑duplicates. Up to 5 initial contextual questions generated from user input + spec signals.

**Output:** `specs/<feature>/checklists/<checklist-name>.md` — creates new file or appends to existing (never deletes).

---

### 8.9 `solidspec check`

Verifies system prerequisites:

- Git availability and version
- AI agent CLI availability (for agents with `requires_cli: true`)
- Required tool versions for installed extensions (`requires.tools` in manifest)
- `.solidspec/` directory structure integrity
- Constitution file presence

---

### 8.10 `solidspec preset <subcommand>`

Manages workflow presets (template and command customizations).

**Subcommands:** `add [--priority N]`, `remove [--force]`, `list`, `search`, `info`, `catalog list`, `catalog add`, `catalog remove`

See section 12 for the preset system.

---

### 8.11 `solidspec extension <subcommand>`

Manages extensions (additional functionality modules).

**Subcommands:** `add [--from URL | --dev PATH]`, `remove [--force]`, `enable`, `disable`, `list`, `search`, `info`, `catalog list`, `catalog add`, `catalog remove`

See section 13 for the extension system.

---

### 8.12 `solidspec upgrade`

Refreshes project templates and scripts after a SolidSpec binary update.

**Flags:**

| Flag | Description |
|------|-------------|
| `--force` | Skip confirmation prompts |

**Behavior:**

- Updates `.solidspec/templates/` with new embedded defaults (does NOT touch `overrides/`)
- Updates `.solidspec/scripts/` with new helper scripts
- Refreshes agent command files for all detected agents
- **Preserves** `constitution.md` (known limitation: must back up before upgrade if customized)
- **Preserves** `specs/` directory entirely (never modified)
- **Preserves** installed presets and extensions
- Reports what was updated and what was preserved

---

## 9. Project Structure

### SolidSpec source code layout

```text
solidspec/
├── src/
│   ├── main.rs
│   ├── cli/                  # Command definitions (clap)
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── specify.rs
│   │   ├── clarify.rs
│   │   ├── plan.rs
│   │   ├── tasks.rs
│   │   ├── implement.rs
│   │   ├── analyze.rs
│   │   ├── checklist.rs
│   │   ├── check.rs
│   │   ├── upgrade.rs
│   │   ├── preset.rs
│   │   └── extension.rs
│   ├── core/                 # Business logic (no CLI dependency)
│   │   ├── mod.rs
│   │   ├── spec_parser.rs    # Parse & validate spec.md
│   │   ├── planner.rs        # Generate architecture plans
│   │   ├── task_generator.rs # Transform plans to tasks
│   │   ├── executor.rs       # Execute tasks via agents
│   │   ├── analyzer.rs       # Cross-artifact validation
│   │   ├── constitution.rs   # Governance enforcement
│   │   └── feature.rs        # Feature numbering & branch mgmt
│   ├── agents/               # AI agent integration
│   │   ├── mod.rs
│   │   ├── config.rs         # Agent config table (21 agents, data-driven)
│   │   ├── registry.rs       # Agent detection & command registration
│   │   ├── registrar.rs      # Write command files in agent-specific format
│   │   ├── formats.rs        # Markdown/TOML rendering, frontmatter, placeholder translation
│   │   ├── special.rs        # Copilot (.agent.md + .prompt.md), Kimi (dir-based), Cursor (.mdc)
│   │   └── skills.rs         # AI skills installation (separate from commands)
│   ├── templates/            # Template rendering engine
│   │   ├── mod.rs
│   │   └── resolver.rs       # Multi-layer template resolution
│   ├── presets/              # Preset management
│   │   ├── mod.rs
│   │   ├── manager.rs
│   │   └── catalog.rs
│   ├── extensions/           # Extension management
│   │   ├── mod.rs
│   │   ├── manager.rs
│   │   └── catalog.rs
│   └── config/               # Configuration handling
│       ├── mod.rs
│       └── spec_toml.rs
├── templates/                # Embedded default templates
│   ├── spec-template.md
│   ├── plan-template.md
│   ├── tasks-template.md
│   ├── checklist-template.md
│   ├── constitution-template.md
│   ├── agent-file-template.md
│   └── commands/             # AI agent command templates
│       ├── specify.md
│       ├── plan.md
│       ├── tasks.md
│       ├── clarify.md
│       ├── implement.md
│       ├── analyze.md
│       └── checklist.md
├── scripts/                  # Cross-platform helper scripts
│   ├── bash/
│   │   ├── common.sh         # Shared functions (get_repo_root, resolve_template, etc.)
│   │   ├── check-prerequisites.sh
│   │   ├── create-new-feature.sh
│   │   ├── setup-plan.sh
│   │   └── update-agent-context.sh
│   └── powershell/
│       ├── common.ps1
│       ├── check-prerequisites.ps1
│       ├── create-new-feature.ps1
│       ├── setup-plan.ps1
│       └── update-agent-context.ps1
├── tests/
│   ├── integration/
│   ├── snapshots/
│   └── agent_consistency/    # Agent config sync tests
├── Cargo.toml
└── Cargo.lock
```

### Target project layout (created by `solidspec init`)

```text
project/
├── .solidspec/
│   ├── constitution.md           # Project governance principles
│   ├── AGENT.md                  # Agent context file (auto-generated)
│   ├── init-options.json         # Persisted init choices
│   ├── config.toml               # Project configuration
│   ├── templates/
│   │   ├── spec-template.md      # Core templates (refreshed on upgrade)
│   │   ├── plan-template.md
│   │   ├── tasks-template.md
│   │   ├── checklist-template.md
│   │   ├── constitution-template.md
│   │   └── overrides/            # Project-specific template tweaks (user-managed)
│   ├── scripts/
│   │   ├── bash/                 # Shell helpers (refreshed on upgrade)
│   │   └── powershell/
│   ├── presets/
│   │   └── .registry             # Installed presets (JSON)
│   └── extensions/
│       ├── .registry             # Installed extensions (JSON)
│       └── .cache/catalogs/      # Catalog cache (SHA256-hashed)
├── specs/
│   └── 001-feature-name/
│       ├── spec.md
│       ├── clarifications.md
│       ├── plan.md
│       ├── research.md
│       ├── data-model.md
│       ├── quickstart.md
│       ├── contracts/
│       ├── tasks.md
│       └── checklists/
│           └── requirements.md
└── solidspec.toml
```

---

## 10. Configuration

### Root configuration (`solidspec.toml`)

```toml
[project]
name = "my_project"
version = "0.1.0"

[ai]
default_agent = "claude"       # Default AI agent for commands

[git]
auto_branch = true             # Auto-create branches on specify
auto_commit = true             # Auto-commit generated artifacts

[templates]
override_dir = ".solidspec/templates/overrides"
```

### Project configuration (`.solidspec/config.toml`)

```toml
[presets]
catalogs = []                  # Additional preset catalog URLs

[extensions]
catalogs = []                  # Additional extension catalog URLs
```

### User‑level configuration (`~/.solidspec/`)

User‑level configuration applies to all projects on the machine:

```text
~/.solidspec/
├── preset-catalogs.yml         # User-level preset catalog URLs
└── extension-catalogs.yml      # User-level extension catalog URLs
```

These catalogs merge with project‑level catalogs. Project‑level takes precedence over user‑level.

### Environment Variables

| Variable | Description |
|----------|-------------|
| `SOLIDSPEC_AI` | Override default AI agent |
| `SOLIDSPEC_FEATURE` | Override feature detection (branch/spec lookup) |
| `SOLIDSPEC_PRESET_CATALOG_URL` | Custom preset catalog (overrides entire stack) |
| `SOLIDSPEC_EXTENSION_CATALOG_URL` | Custom extension catalog (overrides entire stack) |
| `GH_TOKEN` / `GITHUB_TOKEN` | GitHub API authentication (CLI flag takes precedence) |

---

## 11. Template System

Templates are the core mechanism for enforcing quality in generated artifacts. They act as structured prompts that constrain AI output.

### Template Resolution Hierarchy

Resolution order (highest to lowest priority):

1. `.solidspec/templates/overrides/` — project‑specific one‑off tweaks
2. `.solidspec/presets/<id>/templates/` — preset overrides (sorted by priority)
3. `.solidspec/extensions/<id>/templates/` — extension‑provided templates
4. Embedded defaults (shipped with SolidSpec binary)

### Template Design Principles

Templates enforce quality through structural constraints:

- **Prevent premature implementation** — spec template forbids tech stack details, focusing on WHAT and WHY
- **Force explicit uncertainty** — `[NEEDS CLARIFICATION: specific question]` markers prevent assumptions
- **Structured checklists** — self‑review gates catch gaps systematically
- **Constitutional compliance** — plan template includes pre‑implementation quality gates
- **Hierarchical detail management** — main documents stay high‑level; details go to supporting files
- **Test‑first thinking** — task template orders test creation before implementation
- **No speculative features** — every feature must trace to a concrete user story

### Constitution Template

The default constitution template defines governance articles that the plan command evaluates as quality gates:

- **Article I: Library‑First** — features begin as standalone libraries with clear boundaries
- **Article II: CLI Interface** — all functionality accessible via text input/output (stdin/stdout/JSON)
- **Article III: Test‑First** — no implementation before tests are written, validated, and confirmed to fail
- **Article VII: Simplicity** — max 3 projects for initial implementation; additional requires justification
- **Article VIII: Anti‑Abstraction** — use frameworks directly, no wrapper layers; single model representation
- **Article IX: Integration‑First** — prefer real databases/services over mocks; contract tests mandatory before implementation

Users can customize the constitution after init. The plan command's Constitution Check gates map directly to these articles.

### Command Handoff Chain

Commands define handoffs — suggestions for the next command to run after completion:

```text
specify → clarify → plan → tasks → implement
                                ↘ analyze (optional)
specify → checklist (optional, can run anytime after specify)
```

Handoffs are encoded in command frontmatter (`handoffs` field) and displayed to the user as suggested next steps. They are suggestions, not enforced.

### Template Rendering

Templates use [Tera](https://keats.github.io/tera/) for variable substitution. Available context variables:

- `feature_name`, `feature_id`, `branch_name`
- `date`, `project_name`
- `arguments` (user input from command)

---

## 12. Preset System

Presets are stackable workflow customizations that override templates and commands.

### Preset Manifest (`preset.yml`)

```yaml
schema_version: "1.0"
preset:
  id: my-preset
  name: My Custom Preset
  version: 1.0.0
  description: "Custom workflow for my team"
  author: Author Name
requires:
  solidspec_version: ">=0.1.0"
provides:
  templates:
    - type: template
      name: spec-template
      file: templates/spec-template.md
      replaces: spec-template
    - type: command
      name: solidspec.specify
      file: commands/specify.md
```

### Preset Features

- **Stackable** — multiple presets active simultaneously
- **Priority‑based** — lower priority number = higher precedence
- **Template replacement** — presets fully replace templates, not merge (no partial overrides). Future composition strategies planned: `prepend`, `append`, `wrap` (using `{CORE_TEMPLATE}` placeholder)
- **Command overrides** — presets can provide `type: "command"` entries registered to AI agents
- **Catalog discovery** — multi‑catalog stack with priority ordering, same model as extensions
- **Template types** — valid values: `"template"`, `"command"`, `"script"` (enum enforced)

### Preset Registry

Installed presets tracked in `.solidspec/presets/.registry` (JSON), with `priority` field per preset.

### Preset Catalog Stack

Same architecture as extension catalogs:
- Default + community catalogs
- Custom catalogs in `.solidspec/config.toml` or `~/.solidspec/preset-catalogs.yml`
- `SOLIDSPEC_PRESET_CATALOG_URL` env var overrides entire stack
- Per‑URL SHA256 caching, 1‑hour TTL

### Preset Management

```bash
solidspec preset add my-preset
solidspec preset add my-preset --priority 10
solidspec preset remove my-preset
solidspec preset list
solidspec preset search "testing"
solidspec preset info my-preset
solidspec preset catalog list
solidspec preset catalog add <url> --priority 5
solidspec preset catalog remove <url>
```

---

## 13. Extension System

Extensions add functionality without modifying the core.

### Manifest Validation Rules

**Shared rules** (both extension and preset manifests):

- **ID**: must match `^[a-z0-9-]+$` (lowercase alphanumeric + hyphens only)
- **Version**: semantic versioning (semver), parsed and validated
- **Version specifiers** (in `requires`): `>=0.1.0`, `>=1.0,<2.0` etc., validated at install time
- **Description**: max 200 characters
- **Schema version**: must match supported version (`"1.0"`)

**Extension‑specific rules:**

- **Command name**: must follow three‑segment dot notation `^solidspec\.[a-z0-9-]+\.[a-z0-9-]+$` (e.g., `solidspec.my-ext.validate`)
- **Commands list**: must be non‑empty (at least one command required)
- **Hook triggers**: must be in the allowed set (`after_init`, `after_add`, `after_remove`, `after_tasks`, `before_implement`, `after_implement`)

**Preset‑specific rules:**

- **Template types**: must be `"template"`, `"command"`, or `"script"` (enum enforced)
- **Command names**: two‑segment dot notation for core overrides (`solidspec.specify`), three‑segment for extension commands

### Extension Manifest (`extension.yml`)

```yaml
schema_version: "1.0"
extension:
  id: my-extension
  name: My Extension
  version: 1.0.0
  description: "Adds custom validation"
  author: Author Name
requires:
  solidspec_version: ">=0.1.0"
  tools:
    - name: some-tool
      version: ">=1.0"
      required: false
provides:
  commands:
    - name: solidspec.my-ext.validate
      file: commands/validate.md
      description: "Custom validation command"
  config:
    - name: config.yml
      template: config-template.yml
      required: false
hooks:
  after_tasks:
    command: solidspec.my-ext.validate
    optional: true
```

### Extension Features

- **Command registration** — extensions add commands to all detected AI agents, with per‑agent format translation
- **Command aliases** — commands can have `aliases` list; each alias gets a full command file written for every agent
- **Enable/disable** — disable command registration without removing extension; re‑enable re‑registers commands
- **Hook system** — lifecycle hooks triggered at specific workflow points
- **Config templates** — extensions can ship their own configuration files
- **Catalog discovery** — multi‑catalog stack with priority ordering
- **Validation** — manifest validated against schema on install (schema version, required fields, semver)
- **Dependency resolution** — `requires.extensions` list checked before install; `requires.tools` validated with version specifiers
- **`.extensionignore`** — gitignore‑style file to exclude files during install from ZIP

### Hook System

Supported hook triggers:

| Hook | When |
|------|------|
| `after_init` | After project initialization |
| `after_add` | After extension is installed |
| `after_remove` | After extension is removed |
| `after_tasks` | After task generation |
| `before_implement` | Before implementation begins |
| `after_implement` | After implementation completes |

Hook execution:
- Loads hook file as shell script
- Passes environment variables: `EXTENSION_ID`, `PROJECT_ROOT`, etc.
- Captures output and returns status code
- Silent failure by default (logs warning, does not block workflow)
- Hook `condition` field is **not evaluated by templates** — deferred to runtime HookExecutor implementation
- Disabled extensions → hooks skipped entirely

### Extension Registry

Installed extensions tracked in `.solidspec/extensions/.registry` (JSON):

```json
{
  "my-extension": {
    "id": "my-extension",
    "name": "My Extension",
    "version": "1.0.0",
    "installed_timestamp": "2026-03-14T12:00:00Z",
    "enabled": true,
    "commands": { "claude": ["solidspec.my-ext.validate"] },
    "hooks": [{ "trigger": "after_tasks", "file": "hooks/validate.sh" }],
    "templates": []
  }
}
```

**Registry behavior:**

- `get()` and `list()` return **deep copies** to prevent accidental mutation of internal state
- `update()` **preserves** the original `installed_timestamp` even if caller provides a new one
- Corrupted or missing `.registry` file → starts fresh with empty registry (no crash)
- `update()` on non‑existent entry → error (must install first)

### Multi‑Catalog Stack

Catalogs are stackable with priority ordering (lower number = higher precedence):

- **Default catalog** (`catalog.json`) — official, `install_allowed: true`
- **Community catalog** (`catalog.community.json`) — discovery only, `install_allowed: false`
- **Custom catalogs** — configured in `.solidspec/config.toml` or `~/.solidspec/extension-catalogs.yml`:

```yaml
catalogs:
  - name: "Internal Org"
    url: "https://org.internal/catalog.json"
    priority: 5
    install_allowed: true
```

Catalog caching: per‑URL SHA256‑hashed cache files in `.solidspec/extensions/.cache/catalogs/`, 1‑hour TTL. HTTPS required (HTTP only for localhost).

On ID conflicts across catalogs, higher‑priority catalog wins.

### Extension Install Sources

1. **From catalog**: `solidspec extension add <id>` — enforces `install_allowed` policy
2. **From URL**: `solidspec extension add --from <url>` — downloads ZIP, validates HTTPS
3. **From local directory**: `solidspec extension add --dev <path>` — validates `extension.yml` exists

ZIP handling: auto‑unwraps single root directory; adjusts script paths (`../../scripts/` → `.solidspec/scripts/`).

### Extension Management

```bash
solidspec extension add my-extension
solidspec extension add --from https://github.com/user/repo
solidspec extension add --dev ./local-extension
solidspec extension remove my-extension
solidspec extension enable my-extension
solidspec extension disable my-extension
solidspec extension list
solidspec extension search "lint"
solidspec extension info my-extension
solidspec extension catalog list
solidspec extension catalog add <url> --priority 5
solidspec extension catalog remove <url>
```

Name resolution: both extension ID and display name accepted. Ambiguous matches (multiple names matching) require the user to specify the ID explicitly.

---

## 14. AI Agent Integration

SolidSpec generates commands for multiple AI coding agents.

### Supported Agents

| Agent | Command Dir | Format | Notes |
|-------|-------------|--------|-------|
| Claude Code | `.claude/commands/` | Markdown | Requires CLI |
| Gemini CLI | `.gemini/commands/` | TOML | Requires CLI; `{{args}}` placeholder |
| GitHub Copilot | `.github/agents/` | Markdown | `.agent.md` extension + companion `.prompt.md` in `.github/prompts/` |
| Cursor | `.cursor/commands/` | Markdown | IDE‑based (no CLI check) |
| Windsurf | `.windsurf/workflows/` | Markdown | IDE‑based |
| Codex CLI | `.codex/prompts/` | Markdown | Requires CLI |
| Qwen Code | `.qwen/commands/` | Markdown | Requires CLI |
| opencode | `.opencode/command/` | Markdown | Singular `command/` dir |
| Kilo Code | `.kilocode/workflows/` | Markdown | IDE‑based |
| Auggie CLI | `.augment/commands/` | Markdown | Requires CLI |
| Roo Code | `.roo/commands/` | Markdown | IDE‑based |
| CodeBuddy | `.codebuddy/commands/` | Markdown | Requires CLI |
| Qoder CLI | `.qoder/commands/` | Markdown | Requires CLI |
| Kiro CLI | `.kiro/prompts/` | Markdown | Accepts both `kiro-cli` and `kiro` executables |
| Amp | `.agents/commands/` | Markdown | Requires CLI |
| SHAI | `.shai/commands/` | Markdown | Requires CLI |
| Tabnine CLI | `.tabnine/agent/commands/` | TOML | `{{args}}` placeholder |
| Kimi Code | `.kimi/skills/<cmd>/SKILL.md` | Markdown | Directory‑based skill structure |
| Mistral Vibe | `.vibe/prompts/` | Markdown | |
| IBM Bob | `.bob/commands/` | Markdown | IDE‑based |
| Generic | `--ai-commands-dir <path>` | Markdown | For unsupported agents |

### Agent Aliases

Short aliases map to full agent IDs for convenience:

```text
claude → claude, kiro → kiro-cli, vibe → vibe
```

Aliases are auto‑generated in help text and accepted in `--ai` flag.

### Agent Categories

- **CLI agents** (`requires_cli: true`): Claude, Gemini, Codex, Qwen, opencode, Auggie, CodeBuddy, Qoder, Kiro, Amp, SHAI, Tabnine, Kimi
  - Verified during `solidspec check` — tool must be in PATH
- **IDE agents** (`requires_cli: false`): Cursor, Windsurf, Kilo Code, Roo, Copilot, Bob
  - No CLI verification — commands registered if directory exists or `--ai` explicitly set

### Command File Format

**Markdown format** (most agents):

```markdown
---
description: "Generate implementation plan from specification"
scripts:
  sh: scripts/bash/setup-plan.sh "$ARGUMENTS"
  ps: scripts/powershell/setup-plan.ps1 "$ARGUMENTS"
handoffs:
  - label: Generate Tasks
    agent: solidspec.tasks
    prompt: "Generate task breakdown for this feature"
    send: true
---

Read the specification at specs/$ARGUMENTS/spec.md and generate
an implementation plan following the plan template...
```

**TOML format** (Gemini, Tabnine):

```toml
description = "Generate implementation plan from specification"

prompt = """
Read the specification at specs/{{args}}/spec.md and generate
an implementation plan following the plan template...
"""
```

### Agent‑Specific Behaviors

- **Copilot**: Commands use `.agent.md` extension (not `.md`). Companion `.prompt.md` files auto‑generated in `.github/prompts/` and cleaned up on removal.
- **Cursor**: Agent context files (`.mdc`) require YAML frontmatter with `description`, `globs: ["**/*"]`, `alwaysApply: true`. Frontmatter added on creation, preserved on update (no duplication).
- **Kimi**: Commands stored as directory‑based skills (`.kimi/skills/<cmd-name>/SKILL.md`), not flat files. Uses **dot‑separator** skill names (`solidspec.specify`) matching `/skill:solidspec.*` invocation pattern. All other agents use **hyphen‑separator** (`solidspec-specify`).
- **Gemini/Tabnine**: Use TOML format with `{{args}}` instead of `$ARGUMENTS`.
- **opencode**: Uses singular `command/` (not `commands/`).
- **Kiro CLI**: Accepts both `kiro-cli` and `kiro` executables as compatibility fallback. Replaced legacy Amazon Q (`"q"`) agent key.
- **Qoder CLI**: Executable name is `qodercli` (not `qoder`).

### Command Frontmatter

Command templates include YAML frontmatter with:

- `description` — short summary shown in agent help
- `scripts` — per‑platform shell scripts (`sh` and `ps` keys)
- `handoffs` — list of downstream commands to suggest after completion (label, agent, prompt, send flag)

Script paths in frontmatter are adjusted during registration (`../../scripts/` → `.solidspec/scripts/`).

### Agent Detection

On `solidspec init`, the tool:

1. Scans the repository for known agent directories
2. Registers SolidSpec commands in each detected agent's format
3. Translates argument placeholders per agent (`$ARGUMENTS` vs `{{args}}`)
4. Creates alias files for commands that define aliases

### AI Skills Installation

In addition to command registration, SolidSpec can install **AI skills** — a separate mechanism from commands:

- Skills installed to `{agent_folder}/skills/` (not `commands/`)
- Kimi uses dot‑separator names (`solidspec.specify`); all other agents use hyphen (`solidspec-specify`)
- Skills are **additive** — never overwrite pre‑existing skill files
- `--ai-skills` flag on init; requires `--ai` to be specified
- Returns success only if at least one skill file was written

### Input Validation

- **Empty description validation** — `solidspec specify ""` returns error (description must not be empty)
- **Agent flag validation** — `--ai` with invalid value suggests available agents list
- **Token sanitization** — `--github-token` value stripped of whitespace; empty string treated as None

### Agent Context

SolidSpec generates a `.solidspec/AGENT.md` file containing:

- Project constitution principles
- Current specifications and plans
- Available SolidSpec commands

This file is referenced by AI agents for project context. Updated automatically by helper scripts when the constitution or specs change.

### Feature Branch Detection

When determining the current feature (for commands that accept `<feature-id>`), the resolution order is:

1. Explicit `<feature-id>` argument
2. `SOLIDSPEC_FEATURE` environment variable
3. Current Git branch name (if it matches `\d{3}-.*` pattern)
4. Latest feature directory in `specs/` (by numeric prefix)

Multiple branches can map to the same spec directory (e.g., `004-fix-bug` and `004-add-feature` both resolve to `specs/004-*/`).

---

## 15. CLI User Experience

### Step Tracker

Long‑running commands display hierarchical progress using a step tracker:

- **Status states**: `pending`, `running`, `done`, `error`, `skipped`
- **Visual indicators**: `●` (filled) for done/error, `○` (open) for running/pending
- **Color coding**: green for done, red for error, yellow for running, gray for pending/skipped
- **Detail text**: optional light‑gray parenthetical for each step
- **Live refresh**: callback support for real‑time updates during operations

### Interactive Selection

For choices (e.g., AI agent selection, preset picker):

- Arrow key navigation (UP/DOWN)
- ENTER to confirm, ESC to cancel
- Keyboard shortcut support (CTRL+P/CTRL+N as alternatives)
- Single‑item lists auto‑select without prompting

### GitHub API Rate Limiting

When fetching catalogs from GitHub:

- Parse `X-RateLimit-Remaining`, `X-RateLimit-Reset` headers
- Display user‑friendly reset time (converted to local timezone)
- Show authenticated (5,000/hour) vs unauthenticated (60/hour) limits
- Suggest `GH_TOKEN` setup for unauthenticated rate limit hits

### Non‑Git Repository Support

SolidSpec works in non‑Git repositories with degraded functionality:

- Feature branch creation skipped (with warning)
- Feature detection falls back to `SOLIDSPEC_FEATURE` env var or latest `specs/` directory
- All artifact generation still works
- `solidspec check` reports Git as unavailable

---

## 16. Technical Stack

All Rust libraries are **mature and widely used**:

### Versioning

- **semver** — semantic version parsing and comparison (for extension/preset requires)

### CLI

- **clap** (derive API) — command parsing, help system, completions

### Configuration

- **serde** + **toml** — config serialization/deserialization
- **serde_yaml** — YAML parsing for preset/extension manifests

### Templates

- **tera** — template rendering with inheritance

### Markdown

- **pulldown-cmark** — Markdown parsing for spec validation

### HTTP Client

- **ureq** — synchronous, minimal HTTP client (for catalog fetching)

### Git Integration

- **git2** — libgit2 bindings (branch creation, repo init)

### Shell Execution

- **duct** — cross-platform process handling

### Filesystem

- **walkdir** — directory traversal
- **directories** — platform‑specific paths (cache, config)

### Logging

- **log** + **env_logger** — environment-controlled logging

### Error Handling

- **thiserror** — typed errors for library code
- **anyhow** — ergonomic errors at application boundary

### Archive

- **zip** — ZIP extraction for extension/preset install from URL

### Hashing

- **sha2** — SHA256 hashing for catalog cache filenames

### Terminal UI

- **console** — colored output, terminal detection
- **indicatif** — progress bars for long operations
- **dialoguer** — interactive selection menus, confirmations

---

## 17. Core Modules

### CLI Layer (`src/cli/`)

Handles argument parsing, validation, user interaction. Thin layer that delegates to core logic.

### Spec Parser (`src/core/spec_parser.rs`)

Parses spec.md into structured data:
- Extracts user stories with priorities
- Identifies `[NEEDS CLARIFICATION]` markers
- Validates against checklist criteria
- Extracts functional requirements and entities

### Planner (`src/core/planner.rs`)

Generates architecture plans:
- Reads spec + constitution
- Runs constitution compliance gates
- Produces plan.md + supporting documents (research.md, data-model.md, contracts/)

### Task Generator (`src/core/task_generator.rs`)

Transforms plans into tasks:
- Derives tasks from plan, data model, and contracts
- Organizes by user story with priority ordering
- Marks parallel opportunities
- Includes exact file paths

### Analyzer (`src/core/analyzer.rs`)

Cross-artifact consistency validation:
- Requirement → plan traceability
- Plan → task traceability
- Constitution compliance checking

### Constitution Engine (`src/core/constitution.rs`)

Governance enforcement:
- Loads constitution.md
- Evaluates quality gates (simplicity, anti-abstraction, integration-first)
- Reports violations with justification requirements

### Agent Registry (`src/agents/registry.rs`)

AI agent management:
- Detects installed agents by directory presence
- Registers commands in agent-specific format
- Translates argument placeholders
- Manages command lifecycle (add/remove/update)

### Template Resolver (`src/templates/resolver.rs`)

Multi-layer template resolution:
- Walks the priority hierarchy (overrides → presets → extensions → defaults)
- Renders Tera templates with feature context
- Validates template output against schema

### Template Embedding

Default templates are embedded in the binary at compile time via `include_str!` or `rust-embed`. On `solidspec init`, they are copied to `.solidspec/templates/`. On `solidspec upgrade`, they are refreshed (overrides/ is preserved).

---

## 18. Rust Engineering Best Practices

- Modular architecture: CLI, core, agents, templates, presets, extensions as separate module trees
- Separation of CLI vs core logic (core has no clap dependency)
- Avoid `unwrap()` in production — use `?` with typed errors
- Composable error types via `thiserror` in library, `anyhow` at boundaries
- Minimal dependency surface
- Consistent formatting (`rustfmt`) and linting (`clippy`)
- `#[cfg(test)]` unit tests alongside code
- Integration tests in `tests/` exercising full CLI workflows

---

## 19. Testing Strategy

### Unit Tests

Inside modules using `cargo test`. Cover:
- Spec parsing and validation
- Template resolution logic
- Feature numbering
- Config loading
- Constitution gate evaluation

### Integration Tests

In `tests/` exercising CLI workflows end-to-end:
- `init` → `specify` → `plan` → `tasks` full pipeline
- Preset add/remove/resolve
- Extension add/remove
- Agent detection and command registration

### Snapshot Tests

Verify generated artifacts (specs, plans, tasks) match expected output using `insta` crate.

### Agent Consistency Tests

Validate that all agent configurations stay in sync:
- Agent config table matches command registrar configs
- All agents appear in help text and shell completions
- Frontmatter rendering produces valid YAML/TOML per agent format
- Alias resolution is bijective (no duplicate aliases)

---

## 20. Error Handling

All errors must include clear CLI feedback with:

- What went wrong
- Where (file path, feature ID)
- Suggested fix

**Error categories:**

- Missing specs or invalid feature IDs
- Malformed Markdown (spec/plan/tasks parsing failures)
- Config errors (invalid TOML, missing required fields)
- Template errors (missing variables, resolution failures)
- Extension/preset errors (invalid manifest, version incompatibility)
- Network failures (catalog fetch, HTTP errors)
- Git errors (branch conflicts, uncommitted changes)
- Agent errors (unsupported agent, missing command directory)
- Rate limit errors (GitHub API, with reset time and auth guidance)
- Registry corruption (malformed JSON — recovers with empty registry, does not crash)
- Empty/invalid user input (empty description, invalid agent name — with suggestions)

---

## 21. Acceptance Criteria

Feature is complete when:

- `solidspec init` bootstraps project with constitution, templates, config, and agent commands
- `solidspec specify` creates numbered feature branch + spec.md from template
- `solidspec clarify` identifies and resolves `[NEEDS CLARIFICATION]` markers
- `solidspec plan` generates plan.md + research.md + data-model.md + contracts/
- `solidspec tasks` produces story‑driven, parallelizable task breakdown
- `solidspec implement` sends tasks to configured AI agent
- `solidspec analyze` validates cross‑artifact consistency
- `solidspec checklist` produces quality validation report
- `solidspec preset add/remove/list` manages preset lifecycle
- `solidspec extension add/remove/enable/disable/list` manages extension lifecycle
- `solidspec upgrade` refreshes templates without touching overrides or specs
- Template resolution respects the 4‑layer hierarchy
- At least 4 AI agents supported (Claude, Gemini, Copilot, Mistral Vibe)

---

## 22. Development Phases

Implementation is divided into 6 phases. Each phase produces a working, testable increment. A phase MUST be complete before starting the next.

**Testing rule:** Every implementation task MUST have a corresponding unit test. Tests are written FIRST (red), then implementation makes them pass (green). The task table uses the convention: implementation tasks are odd‑numbered, their corresponding test tasks are even‑numbered and immediately follow.

### Phase 1: Foundation (MVP Core)

**Goal:** Minimal working CLI that can initialize a project and generate a spec from a template.

**Delivers:** `solidspec init` + `solidspec specify` + `solidspec check`

| Task | Description |
|------|-------------|
| P1‑01 | Scaffold Rust project (`Cargo.toml`, `src/main.rs`, module structure) |
| P1‑02 | **Test:** Verify project compiles, `main` runs with `--help`, module tree resolves |
| P1‑03 | Implement config system (`solidspec.toml`, `.solidspec/config.toml`, serde + toml) |
| P1‑04 | **Test:** Load valid config, reject malformed TOML, verify defaults when file missing, round‑trip serialize/deserialize |
| P1‑05 | Implement `solidspec init` — create `.solidspec/` directory tree, copy embedded templates, generate default constitution, persist `init-options.json` |
| P1‑06 | **Test:** Verify directory structure created, all template files present, `init-options.json` contains correct fields, idempotent re‑init preserves existing files |
| P1‑07 | Implement Git integration — repo init, initial commit, `--no-git` flag, `is_git_repo()` detection |
| P1‑08 | **Test:** Verify `.git/` created, initial commit exists, `--no-git` skips repo init, `is_git_repo()` returns correct bool for git and non‑git dirs |
| P1‑09 | Implement feature numbering — scan `specs/` for next number using **global numbering** (highest existing + 1), validate `\d{3}-.*` pattern |
| P1‑10 | **Test:** Empty `specs/` → 001, existing 001+002 → 003, non‑sequential gaps handled (001+003 → 004, not 002), invalid names rejected, branch name generation (2–4 words, action‑noun), empty description → error |
| P1‑11 | Implement template engine — Tera rendering with context variables (`feature_name`, `feature_id`, `branch_name`, `date`, `project_name`, `arguments`) |
| P1‑12 | **Test:** Render template with all variables populated, missing variable returns error, special characters escaped, empty arguments handled |
| P1‑13 | Implement `solidspec specify` — branch creation, spec generation from template, feature directory creation, mandatory quality checklist output |
| P1‑14 | **Test:** Verify branch created with correct name, `specs/<id>-<name>/spec.md` exists and contains expected sections, `checklists/requirements.md` created (mandatory), duplicate feature name handled, max 3 `[NEEDS CLARIFICATION]` markers enforced |
| P1‑15 | Implement `solidspec check` — verify Git, report environment status |
| P1‑16 | **Test:** Reports Git present/absent correctly, output format matches expected structure |
| P1‑17 | Implement error handling framework — `thiserror` types for each error category, `anyhow` at CLI boundary |
| P1‑18 | **Test:** Each error type displays correct message, error context (file path, feature ID) preserved, `Display` and `Error` impls correct |
| P1‑19 | Implement CLI UX basics — colored output (`console`), step tracker with status states |
| P1‑20 | **Test:** Step tracker transitions (pending → running → done/error/skipped), output contains expected symbols, no‑color mode works |
| P1‑21 | Embed default templates (spec, plan, tasks, checklist, constitution, agent-file) via `include_str!` or `rust-embed` |
| P1‑22 | **Test:** All embedded templates load successfully, none are empty, each contains expected header markers |
| P1‑23 | **Integration test:** Full `solidspec init my-project && solidspec specify "user auth"` pipeline produces `specs/001-user-auth/spec.md` from template in a new Git branch |

**Acceptance:** All unit tests pass. Integration test produces correct artifacts. `cargo test` exits 0.

---

### Phase 2: Planning Pipeline

**Goal:** Complete the specify → clarify → plan → tasks pipeline. No agent integration yet — artifacts are generated locally from templates.

**Delivers:** `solidspec clarify` + `solidspec plan` + `solidspec tasks` + `solidspec analyze` + `solidspec checklist`

| Task | Description |
|------|-------------|
| P2‑01 | Implement spec parser — extract user stories, priorities, `[NEEDS CLARIFICATION]` markers, functional requirements, entities |
| P2‑02 | **Test:** Parse spec with 3 stories → correct priorities extracted, markers identified with count, requirements numbered correctly, empty spec handled, malformed markdown returns parse error |
| P2‑03 | Implement `solidspec clarify` — identify markers, generate structured questions (max 5), atomic file saves per integration, terminology normalization, session markers |
| P2‑04 | **Test:** Spec with 3 markers → 3 questions generated, spec with 7 markers → capped at 5, zero markers → no‑op with message, `clarifications.md` contains `### Session YYYY-MM-DD` header, spec saved to disk after each resolved question (atomic), duplicate terminology normalized with `(formerly referred to as "X")` annotation, no vague placeholders remain after write |
| P2‑05 | Implement constitution engine — load `constitution.md`, evaluate quality gates (simplicity, anti-abstraction, integration-first) |
| P2‑06 | **Test:** Valid constitution loads all gates, missing constitution returns error with path, gate evaluation: pass when criteria met, fail with violation details, custom constitution with extra gates |
| P2‑07 | Implement `solidspec plan` — generate plan in phases (Phase 0: research → Phase 1: design → Phase 2: report), dual constitution check (before research + after design), prerequisite check for unresolved markers, AGENT.md update |
| P2‑08 | **Test:** All output files created, `plan.md` contains Technical Context and Constitution Check sections, `contracts/` directory exists, plan references spec requirements, missing spec → error, spec with unresolved `[NEEDS CLARIFICATION]` → error (prerequisite), constitution check runs twice (verify both invocations), `.solidspec/AGENT.md` updated after Phase 1 |
| P2‑09 | Implement task generator — derive tasks from plan + data model + contracts, organize by user story, mark `[P]` parallel tasks, include file paths, strict `- [ ] T### [P?] [Story?] Description` format |
| P2‑10 | **Test:** Tasks match strict format (`- [ ] T###`), IDs zero‑padded 3 digits (T001 not T1), organized by phase (setup → foundational → stories → polish), `[P]` markers ONLY on tasks affecting different files, `[US1]`/`[US2]` labels ONLY in story phases (not in setup/foundational/polish), file paths included, plan with no entities → no data‑model tasks, `after_tasks` hook fires after generation |
| P2‑11 | Implement `solidspec tasks` — produce `tasks.md` with phase structure |
| P2‑12 | **Test:** `tasks.md` created at correct path, contains all phases, checkpoint markers between phases, dependency section present |
| P2‑13 | Implement quality validation in `specify` — auto-generate `checklists/requirements.md`, iterate up to 3 times on failures |
| P2‑14 | **Test:** Spec with impl details triggers validation failure, max 3 iterations enforced (no infinite loop), clean spec passes on first iteration, `checklists/requirements.md` contains checkbox items |
| P2‑15 | Implement feature branch detection — 4‑level resolution (arg → `SOLIDSPEC_FEATURE` env → git branch → latest `specs/`) |
| P2‑16 | **Test:** Explicit arg wins over env var, env var wins over git branch, git branch wins over specs/ scan, non‑matching branch falls through, empty specs/ returns error, multiple matching specs/ dirs picks latest |
| P2‑17 | Implement `solidspec checklist` — strict `CHK###` format, prohibited/required question patterns, soft cap 40 items, `--append` continues numbering from last ID |
| P2‑18 | **Test:** Creates new checklist with `CHK001` starting ID, `--append` continues from last ID (CHK015 → CHK016), items match `- [ ] CHK### <question> [dimension] [reference]` format, prohibited patterns rejected ("Verify implementation…" → error), required patterns present ("Are requirements defined…"), >40 candidates → capped with prioritization, checklist validates requirements NOT implementation |
| P2‑19 | Implement `solidspec analyze` — read‑only consistency validation, severity heuristic (CRITICAL/HIGH/MEDIUM/LOW), findings cap, optional remediation |
| P2‑20 | **Test:** Fully traced artifacts → 100% score, missing plan section → HIGH finding, orphan task → MEDIUM, constitution violation → auto CRITICAL (non‑negotiable), missing spec → error, max 50 findings enforced (overflow aggregated), analyze does NOT modify any files (read‑only verified), remediation suggestions present per finding |
| P2‑21 | Cross-platform shell scripts — bash + powershell helpers (`common.sh`/`common.ps1`, `create-new-feature`, `setup-plan`, `check-prerequisites`) |
| P2‑22 | **Test:** `get_repo_root()` returns correct path, `get_current_branch()` resolves in priority order, `find_feature_dir_by_prefix()` handles single match/multiple matches/no match, `json_escape()` escapes special chars |
| P2‑23 | **Integration test:** Full `init` → `specify` → `clarify` → `plan` → `tasks` → `analyze` pipeline; **snapshot tests** for all generated artifacts |

**Acceptance:** All unit tests pass. `solidspec analyze 001` reports 100% traceability on generated output. Snapshot tests capture baseline.

---

### Phase 3: AI Agent Integration

**Goal:** Register SolidSpec commands with AI coding agents. Implement the agent registry, format translation, and command lifecycle.

**Delivers:** Agent detection + command registration for 4+ agents + `solidspec implement`

| Task | Description |
|------|-------------|
| P3‑01 | Define agent config table — struct per agent with: ID, command dir, file extension, format (Markdown/TOML), argument placeholder, requires_cli flag, aliases |
| P3‑02 | **Test:** Config table contains all 21 agents, no duplicate IDs, no duplicate aliases, each agent has non‑empty command dir and format, CLI agents have `requires_cli: true` |
| P3‑03 | Implement agent detection — scan repo for known agent directories, check CLI availability for CLI agents |
| P3‑04 | **Test:** Repo with `.claude/` → claude detected, repo with `.gemini/` + `.cursor/` → both detected, empty repo → none detected, CLI agent without binary → flagged as unavailable but dir detected |
| P3‑05 | Implement command registrar — write command files in agent-specific format, YAML frontmatter rendering, TOML rendering |
| P3‑06 | **Test:** Markdown agent → `.md` file with `---` frontmatter delimiters + valid YAML, TOML agent → `.toml` file with `description` + `prompt` fields, file written to correct directory |
| P3‑07 | Implement argument placeholder translation (`$ARGUMENTS` ↔ `{{args}}`) |
| P3‑08 | **Test:** Markdown → `$ARGUMENTS` preserved, TOML → `$ARGUMENTS` replaced with `{{args}}`, reverse translation works, no double‑replacement on already‑translated content |
| P3‑09 | Implement Copilot special handling — `.agent.md` extension, companion `.prompt.md` generation/cleanup |
| P3‑10 | **Test:** Copilot commands use `.agent.md` extension, `.prompt.md` created in `.github/prompts/`, removal deletes both files, non‑Copilot agents unaffected |
| P3‑11 | Implement Kimi special handling — directory‑based skills (`<cmd>/SKILL.md`), dot‑separator naming (`solidspec.specify` not `solidspec-specify`) |
| P3‑12 | **Test:** Kimi command creates `<cmd>/SKILL.md` directory structure, uses dot‑separator (`solidspec.specify`), all other agents use hyphen‑separator (`solidspec-specify`), removal deletes directory, content matches Markdown format |
| P3‑13 | Implement command alias support — write alias files for all agents |
| P3‑14 | **Test:** Command with 2 aliases → 3 files written (main + 2 aliases), alias content matches main command, aliases cleaned up on removal |
| P3‑15 | Implement frontmatter script path adjustment (`../../scripts/` → `.solidspec/scripts/`) |
| P3‑16 | **Test:** Paths adjusted in `scripts.sh` and `scripts.ps` frontmatter fields, non‑script frontmatter fields untouched, already‑adjusted paths not double‑adjusted |
| P3‑17 | Implement `solidspec implement` — blocking checklist check, parse `tasks.md`, respect dependencies and `[P]` markers, mark tasks `[X]` on completion, fire `before_implement`/`after_implement` hooks, `--pass N` support, detection‑driven ignore files |
| P3‑18 | **Test:** Unchecked checklist items → blocking prompt before proceeding, parse tasks with dependencies → correct execution order, `[P]` tasks grouped for parallel, completed tasks marked `[X]` in tasks.md, `--pass 2` runs twice, missing `tasks.md` → error, `before_implement` hook fires before first task, `after_implement` hook fires after last task, `.dockerignore` created only if Dockerfile detected |
| P3‑19 | Implement `.solidspec/AGENT.md` generation from constitution + specs |
| P3‑20 | **Test:** AGENT.md contains constitution principles, lists available commands, updates when specs change |
| P3‑21 | Implement Cursor `.mdc` agent file handling — YAML frontmatter with `description`, `globs`, `alwaysApply` fields |
| P3‑22 | **Test:** `.mdc` file created with frontmatter (`alwaysApply: true`), update preserves existing frontmatter (no duplication — exactly 1 occurrence of `alwaysApply`), non‑`.mdc` agent files have no frontmatter |
| P3‑23 | Implement AI skills installation — install to `skills/` subdir (not `commands/`), additive (never overwrite), `--ai-skills` requires `--ai` flag |
| P3‑24 | **Test:** Skills installed to `{agent}/skills/`, pre‑existing skill files preserved (additive), `--ai-skills` without `--ai` → error, returns true only if ≥1 skill written, Kimi uses dot‑separator skill names |
| P3‑25 | Wire agent registration into `solidspec init` — auto‑detect and register on initialization |
| P3‑26 | **Test:** `init` in repo with `.claude/` → commands registered, `init --ai gemini` in repo without `.gemini/` → `.gemini/commands/` created and populated, `init --ai invalid` → error with available agents list |
| P3‑27 | **Agent consistency tests:** Config table matches registrar, all agents in help text, alias bijection, no format mismatch between config and registrar |

**Acceptance:** All unit tests pass. `solidspec init --ai claude` registers valid commands. Agent consistency tests verify sync across all 21 agent configs.

---

### Phase 4: Template Resolution & Presets

**Goal:** Multi‑layer template resolution and the full preset system.

**Delivers:** Template override hierarchy + `solidspec preset` subcommands

| Task | Description |
|------|-------------|
| P4‑01 | Implement template resolver — walk 4‑layer priority hierarchy (overrides → presets → extensions → embedded defaults) |
| P4‑02 | **Test:** Override present → override wins, preset present (no override) → preset wins, extension present (no preset/override) → extension wins, nothing present → embedded default, empty override file still wins (exists = takes precedence) |
| P4‑03 | Implement preset manifest parser — validate `preset.yml` (schema version, required fields, semver, template types enum) |
| P4‑04 | **Test:** Valid manifest parses all fields, missing `schema_version` → error, invalid semver → error, unknown template type → error, `requires.solidspec_version` specifier validated |
| P4‑05 | Implement preset registry — `.solidspec/presets/.registry` JSON, priority field per preset |
| P4‑06 | **Test:** Add preset → appears in registry with correct priority, remove → gone from registry, duplicate ID → error, priority ordering correct (lower number = higher precedence) |
| P4‑07 | Implement `solidspec preset add` — install from catalog or URL, `--priority` flag, register commands to agents |
| P4‑08 | **Test:** Add from catalog → files copied + registry updated + commands registered, `--priority 5` → stored in registry, add same preset twice → error, commands appear in agent dirs |
| P4‑09 | Implement `solidspec preset remove` — unregister commands, remove from registry |
| P4‑10 | **Test:** Remove → files deleted + registry entry removed + commands unregistered from all agents, remove non‑existent → error |
| P4‑11 | Implement `solidspec preset list/search/info` |
| P4‑12 | **Test:** `list` shows all installed with priorities, `search` filters by keyword in name/description/tags, `info` shows full manifest details, empty results handled |
| P4‑13 | Implement preset catalog system — URL‑based discovery, multi‑catalog stack with priority ordering, SHA256‑hashed cache files, 1‑hour TTL |
| P4‑14 | **Test:** Catalog fetched and cached, second fetch within TTL uses cache, expired cache re‑fetches, SHA256 filename correct, multi‑catalog merge respects priority, ID conflict → higher priority wins |
| P4‑15 | Implement `solidspec preset catalog add/remove/list` |
| P4‑16 | **Test:** Add catalog URL → appears in config, remove → gone, list shows all with priorities, duplicate URL → error |
| P4‑17 | Implement `SOLIDSPEC_PRESET_CATALOG_URL` env var override |
| P4‑18 | **Test:** Env var set → overrides all configured catalogs, env var unset → uses config, env var with invalid URL → error |
| P4‑19 | Wire template resolution into all artifact‑generating commands (`specify`, `plan`, `tasks`, `checklist`) |
| P4‑20 | **Test (integration):** Install preset overriding `spec-template.md` → `solidspec specify` uses preset template, two presets with different priorities → lower‑number priority wins, remove preset → falls back to default |

**Acceptance:** All unit tests pass. Template resolution hierarchy verified end‑to‑end. Multiple presets stack correctly by priority.

---

### Phase 5: Extension System

**Goal:** Full extension lifecycle — install, enable/disable, hooks, catalogs.

**Delivers:** `solidspec extension` subcommands + hook execution

| Task | Description |
|------|-------------|
| P5‑01 | Implement extension manifest parser — validate `extension.yml` (schema version, requires, provides, hooks), enforce ID regex `^[a-z0-9-]+$`, command name regex `^solidspec\.[a-z0-9-]+\.[a-z0-9-]+$`, description max 200 chars, non‑empty commands list |
| P5‑02 | **Test:** Valid manifest parses all sections, missing required fields → specific error, invalid `requires.solidspec_version` → error, hook triggers validated against allowed list (`after_init`, `after_tasks`, etc.), invalid extension ID (uppercase, spaces) → regex error, invalid command name (missing dot segment) → error, description >200 chars → error, empty commands list → error |
| P5‑03 | Implement extension registry — `.solidspec/extensions/.registry` JSON, enabled flag, per-agent command tracking, deep‑copy access, timestamp preservation |
| P5‑04 | **Test:** Add → registry entry with `enabled: true` + timestamp + command list per agent, `get()` returns deep copy (mutations don't affect registry), `update()` preserves original `installed_timestamp`, `update()` on non‑existent → error, disable → `enabled: false`, enable → `enabled: true`, remove → entry deleted, corrupted `.registry` JSON → starts fresh (no crash), load empty registry → empty map |
| P5‑05 | Implement extension install from catalog — enforce `install_allowed` policy, dependency resolution |
| P5‑06 | **Test:** Install from `install_allowed: true` catalog → succeeds, install from `install_allowed: false` → error with message, extension requiring other extension → checks dependency present, missing dependency → error listing what's needed |
| P5‑07 | Implement extension install from URL — ZIP download, HTTPS enforcement (HTTP for localhost), auto‑unwrap single root, `.extensionignore` support (gitignore‑compatible patterns, backslash normalization for Windows) |
| P5‑08 | **Test:** HTTPS URL → download succeeds, HTTP non‑localhost → error, localhost HTTP → allowed, ZIP with single root dir → unwrapped, `.extensionignore` excludes matching files (patterns with `/` anchored to root, trailing `/` restricts to dirs), `.extensionignore` file itself always excluded, backslashes normalized to forward slashes, corrupt ZIP → error, script paths adjusted (`../../scripts/` → `.solidspec/scripts/`) |
| P5‑09 | Implement extension install from local dir (`--dev`) |
| P5‑10 | **Test:** Valid dir with `extension.yml` → installed, dir without manifest → error, `--dev` flag set in registry metadata |
| P5‑11 | Implement extension command registration — register to all detected agents, format translation, alias support |
| P5‑12 | **Test:** Extension with 2 commands + 1 alias → 3 files per agent, Markdown vs TOML format correct, placeholder translated, Copilot `.prompt.md` companions created, registry tracks commands per agent |
| P5‑13 | Implement `solidspec extension remove` — unregister commands (including Copilot `.prompt.md` cleanup), remove from registry |
| P5‑14 | **Test:** Remove → all command files deleted from all agents + Copilot companions deleted + Kimi skill dirs deleted + registry entry removed, remove non‑existent → error |
| P5‑15 | Implement `solidspec extension enable/disable` — toggle command registration without removing |
| P5‑16 | **Test:** Disable → commands unregistered from all agents + `enabled: false` in registry + extension files still on disk, enable → commands re‑registered + `enabled: true`, disable already‑disabled → no‑op |
| P5‑17 | Implement `solidspec extension list/search/info` — name resolution (ID + display name), ambiguity detection |
| P5‑18 | **Test:** `list` shows all with enabled status, `search` by keyword, `info` by ID works, `info` by display name works, ambiguous name (2 extensions match) → error listing matches with IDs |
| P5‑19 | Implement extension catalog system — same multi‑catalog stack as presets, `SOLIDSPEC_EXTENSION_CATALOG_URL` override |
| P5‑20 | **Test:** Catalog stack merges correctly, env var overrides stack, caching with TTL, `install_allowed` per-catalog enforced |
| P5‑21 | Implement `solidspec extension catalog add/remove/list` |
| P5‑22 | **Test:** Add/remove/list CRUD operations, priority ordering, duplicate URL rejected |
| P5‑23 | Implement hook executor — load hook scripts, pass env vars (`EXTENSION_ID`, `PROJECT_ROOT`), capture output, silent failure |
| P5‑24 | **Test:** Hook script runs with correct env vars, exit code 0 → success, non‑zero exit → logged but not fatal, missing hook file → warning logged, `EXTENSION_ID` and `PROJECT_ROOT` env vars set correctly |
| P5‑25 | Wire hooks into workflow commands (`after_tasks`, `before_implement`, `after_implement`) |
| P5‑26 | **Test (integration):** Extension with `after_tasks` hook → hook runs after `solidspec tasks`, `before_implement` hook → runs before implement starts, disabled extension → hooks skipped |
| P5‑27 | **Integration test:** Full lifecycle: install from URL → commands registered → `after_tasks` hook fires → disable → commands gone but extension present → enable → commands back → remove → fully cleaned up |

**Acceptance:** All unit tests pass. Full extension lifecycle verified end‑to‑end including hook execution and enable/disable toggling.

---

### Phase 6: Polish & Production Readiness

**Goal:** CLI UX polish, remaining edge cases, cross‑platform hardening, documentation.

**Delivers:** Production‑ready v0.1.0 binary

| Task | Description |
|------|-------------|
| P6‑01 | Interactive selection UI — arrow key navigation for agent/preset/extension choosers (`dialoguer`) |
| P6‑02 | **Test:** Selection with 3 items → returns correct choice, single item → auto‑selects, ESC → returns None/cancel, empty list → error |
| P6‑03 | GitHub API rate limiting — parse rate‑limit headers, display reset time in local timezone, auth guidance |
| P6‑04 | **Test:** Parse `X-RateLimit-Remaining: 0` + `X-RateLimit-Reset: <epoch>` → correct local time string, `Retry-After` header (seconds and HTTP‑date), missing headers → graceful fallback, authenticated vs unauthenticated message differs |
| P6‑05 | Non‑Git repository support — graceful degradation, fallback feature detection, warning messages |
| P6‑06 | **Test:** Non‑git dir → `init` succeeds without `.git/`, `specify` skips branch creation with warning, feature detection falls back to `specs/` scan, `check` reports Git as unavailable |
| P6‑07 | VSCode settings merging during init — deep merge preserving existing keys |
| P6‑08 | **Test:** Merge into empty → creates file, merge into existing → preserves existing keys + adds new, nested objects deep‑merged, arrays replaced (not merged), non‑JSON existing file → error |
| P6‑09 | Unix script permissions — set executable bit on `.sh` files with shebang |
| P6‑10 | **Test:** `.sh` with `#!/bin/bash` → executable bit set, `.sh` without shebang → not modified, `.ps1` files → not modified, Windows → no‑op (function returns Ok) |
| P6‑11 | Constitution preservation on re‑init — don't overwrite existing `constitution.md` |
| P6‑12 | **Test:** First init → constitution created from template, second init → existing constitution preserved (content unchanged), missing constitution on re‑init → re‑created |
| P6‑13 | `--github-token` flag — CLI arg takes precedence over `GH_TOKEN`/`GITHUB_TOKEN` env vars |
| P6‑14 | **Test:** CLI flag set → uses flag value, flag unset + `GH_TOKEN` set → uses env, flag unset + `GITHUB_TOKEN` set → uses env, both envs set → `GH_TOKEN` wins, empty string → treated as None |
| P6‑15 | Shell completions — generate for bash, zsh, fish, PowerShell via clap |
| P6‑16 | **Test:** Completion script generated for each shell, bash completion contains all subcommands, PowerShell completion is valid syntax |
| P6‑17 | Full error message review — every error includes what/where/fix suggestion |
| P6‑18 | **Test:** Each error variant produces message with 3 parts: what went wrong, where (path/ID), suggested fix; no error message is empty or generic |
| P6‑19 | `--debug` flag wiring — env_logger integration for verbose output |
| P6‑20 | **Test:** `--debug` sets `RUST_LOG=debug`, debug output includes module source, non‑debug mode suppresses debug messages |
| P6‑21 | Cross‑platform CI — test on Linux, macOS, Windows |
| P6‑22 | **Test:** CI matrix runs full test suite on all 3 platforms, path separator handling correct, line endings handled |
| P6‑23 | Implement `solidspec upgrade` — refresh templates + scripts, preserve overrides/constitution/specs, `--force` flag |
| P6‑24 | **Test:** Upgrade refreshes templates, preserves `overrides/` untouched, preserves `constitution.md`, preserves `specs/`, agent commands refreshed, `--force` skips confirmation |
| P6‑25 | Release packaging — build binaries, generate per‑agent template bundles |
| P6‑26 | **Test:** Package contains binary + templates, per‑agent bundles contain correct command files in correct format |
| P6‑27 | **Snapshot tests:** All generated artifacts (spec, plan, tasks, checklist, AGENT.md, constitution) match baseline snapshots |

**Acceptance:** All unit tests pass on all 3 platforms. All acceptance criteria from section 21 pass. `cargo test` exits 0 with 100% of tasks covered by tests.

---

### Phase Dependency Graph

```text
Phase 1: Foundation ──────────────────────────┐
         (init, specify, check, config,       │
          templates, git, errors)              │
                                               ▼
Phase 2: Planning Pipeline ───────────────────┐
         (clarify, plan, tasks, analyze,      │
          checklist, constitution, scripts)    │
                                               ▼
Phase 3: Agent Integration ──────┬────────────┐
         (registry, detection,   │            │
          registration, implement)│            │
                                  ▼            ▼
Phase 4: Presets ────────────  Phase 5: Extensions
         (resolver, catalog,   (manifest, hooks,
          priority stack)       catalog, enable/disable)
                    │                     │
                    └──────────┬──────────┘
                               ▼
                    Phase 6: Polish
                    (UX, cross-platform,
                     rate-limiting, release)
```

Phases 4 and 5 can be developed in parallel after Phase 3.

---

## 23. Performance Considerations

- Avoid excessive filesystem scanning — cache directory listings
- Cache parsed configuration and templates within a session
- Lazy‑load extension/preset catalogs (fetch on demand, cache with TTL)
- Parallelize agent command registration across multiple agent directories

---

## 24. Future Enhancements

- Interactive TUI mode (e.g., **ratatui**)
- Task dependency visualization (DAG rendering)
- GitHub issue automation (create issues from tasks)
- Multi-language spec support (i18n templates)
- CI/CD integration (spec validation in pipelines)
- Conditional hook execution in extensions
- Preset/extension publishing workflow
- `solidspec diff` — compare spec versions across branches

---

## 25. Success Metrics

Measured by:

- Time from idea to structured spec + plan + tasks (target: <15 minutes)
- Completeness of generated artifacts (no missing sections)
- Traceability score (% of requirements covered by tasks)
- Number of supported AI agents
- Adoption: presets and extensions published by community

---

## 26. Known Design Gaps

These are known limitations in the current design. They should be addressed in future versions:

- **No extension version conflict resolution** — if two extensions provide the same command name, no conflict detection
- **No cryptographic signature verification** — downloaded extensions/presets validated structurally but not signed
- **No rollback mechanism** — extension/preset removal is permanent; no version history or undo
- **No automatic dependency install** — if extension A depends on extension B, B is not auto‑installed
- **No extension update mechanism** — must remove + re‑add to update
- **No catalog authentication** — private catalog URLs cannot use auth headers
- **No install dry‑run** — no preview of what will be added/removed before execution

---

## 27. Open Questions

- Should constitution principles be customizable per feature or strictly project‑wide?
- Should task dependencies be expressed as a DAG with explicit edges?
- How should conflicting preset templates be resolved (error vs priority)?
- Should SolidSpec validate that AI‑generated artifacts match templates, or trust the agent?
- Should extension hooks support conditional execution (run only if certain files changed)?
- Should release packaging generate per‑agent ZIP bundles?
