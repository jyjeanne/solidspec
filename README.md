<p align="center">
  <img src="docs/picture/logo.png" alt="SolidSpec Logo" width="600">
  <p align="center">
    <strong>Specification-Driven Development for the AI era</strong>
  </p>
  <p align="center">
    A Rust CLI that transforms feature descriptions into structured specs, plans, and tasks &mdash; then lets your AI agent build them.
    Now with <strong>IDSD</strong> (Intent-Driven Specification Development) for evidence-based validation and full traceability.
  </p>
  <p align="center">
    <a href="#install">Install</a> &bull;
    <a href="#the-8-step-workflow">Workflow</a> &bull;
    <a href="#sdd-vs-idsd-which-to-choose">SDD vs IDSD</a> &bull;
    <a href="#parallel-fan-out-ship-gate">Ship Gate</a> &bull;
    <a href="#using-with-claude-code">Claude Code</a> &bull;
    <a href="#using-with-mistral-vibe">Mistral Vibe</a> &bull;
    <a href="#using-with-github-copilot">Copilot</a> &bull;
    <a href="#use-cases">Use Cases</a> &bull;
    <a href="#all-commands">Commands</a>
  </p>
</p>

---

## The Problem

You describe a feature to your AI coding agent. It generates code. But the code doesn't match what you actually needed &mdash; scope creeps, edge cases are missed, and there's no traceability from requirements to implementation.

**SolidSpec fixes this** by inserting a structured specification layer between your idea and the code. Every feature gets a spec, a plan, and a task list &mdash; all versioned in your repo, all driving the AI's implementation.

### The deeper problem SDD alone can't solve

Even with a perfect spec, a common failure mode remains: **intent drift**. The spec describes *what* to build, but not *why* it must exist. Over multiple iterations, implementations drift away from the original intent without anyone noticing — requirements are technically satisfied while the actual user need is not.

**SolidSpec's IDSD mode** (Intent-Driven Specification Development) adds a root anchor to the chain: an `intent.md` file that captures *why* the capability must exist, what constraints bound it, and what evidence will confirm success. Every subsequent artifact traces back to this intent. Drift is measured continuously and surfaced as a first-class metric.

## The 8-Step Workflow

Each pipeline phase uses a specialized **agent persona** — the agent gets role-specific instructions, an expected output format, and a verification checklist before it starts. An **anti-rationalization table** prevents the most common agent shortcuts ("I'll add tests later", "This is too simple for a spec").

### SDD — Specification-Driven Development (default)

```
                    You describe a feature
                            |
                            v
  +----------------------------------------------------------+
  |                                                          |
  |   1. solidspec specify   -->  spec.md                    |
  |   2. solidspec clarify   -->  resolve ambiguities        |
  |   3. solidspec plan      -->  plan.md + research +       |
  |                               data-model + contracts     |
  |   4. solidspec tasks     -->  tasks.md (phased, parallel)|
  |   5. solidspec tests     -->  test scaffolds (per story) |
  |   6. solidspec implement -->  AI builds from tasks       |
  |   7. solidspec analyze   -->  consistency report         |
  |   8. solidspec review    -->  quality review report      |
  |   9. solidspec ship      -->  SHIP / HOLD decision       |
  |      (4 parallel AI review lanes run concurrently)       |
  |                                                          |
  |   solidspec pipeline --new "feature"                     |
  +----------------------------------------------------------+
                            |
                            v
                  Working, traced code
```

### IDSD — Intent-Driven Specification Development (opt-in)

```
                    You capture the WHY first
                            |
                            v
  +----------------------------------------------------------+
  |                                                          |
  |   0. solidspec intent    -->  intent.md (ICE model)      |
  |      WHY it exists · WHAT bounds it · HOW to measure it  |
  |                          |                               |
  |   1–8. (same SDD phases, enriched by intent)             |
  |                          |                               |
  |   9. solidspec evidence  -->  evidence-report.md         |
  |      per-criterion satisfaction from implemented tests   |
  |                                                          |
  |   solidspec pipeline --new "feature" --schema intent-driven|
  +----------------------------------------------------------+
                            |
                            v
           Working, traced code with measured intent coverage
           INT-001 → FR-001 → T001 → test_file.md
```

Every artifact references the one before it. In IDSD mode, requirements trace all the way back to the original intent. Drift is measured automatically at every `solidspec analyze` run.

---

## Install

### Build from source

```bash
git clone https://github.com/jyjeanne/solidspec.git
cd solidspec
cargo build --release
```

The compiled binary is placed at `target/release/solidspec` (Linux/macOS) or `target\release\solidspec.exe` (Windows).

---

### Add to PATH — Linux / macOS

**Option A — copy to a system directory (recommended)**

```bash
sudo cp target/release/solidspec /usr/local/bin/solidspec
```

**Option B — add the build output directory to your shell profile**

```bash
# Bash (~/.bashrc or ~/.bash_profile)
echo 'export PATH="$PATH:$HOME/solidspec/target/release"' >> ~/.bashrc
source ~/.bashrc

# Zsh (~/.zshrc)
echo 'export PATH="$PATH:$HOME/solidspec/target/release"' >> ~/.zshrc
source ~/.zshrc
```

Replace `$HOME/solidspec` with the actual path where you cloned the repository.

---

### Add to PATH — Windows

**Option A — copy to a permanent directory, then add it to the system PATH (recommended)**

```powershell
# 1. Create a directory for CLI tools (skip if it already exists)
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\bin"

# 2. Copy the binary
Copy-Item .\target\release\solidspec.exe "$env:USERPROFILE\bin\solidspec.exe"

# 3. Add the directory to the permanent user PATH (takes effect in new shells)
[Environment]::SetEnvironmentVariable(
    "PATH",
    "$env:PATH;$env:USERPROFILE\bin",
    [EnvironmentVariableTarget]::User
)
```

**Option B — add the build output directory to PATH for the current session only**

```powershell
$env:PATH += ";$(Get-Location)\target\release"
```

To make Option B permanent, add it to your PowerShell profile (`$PROFILE`):

```powershell
Add-Content $PROFILE "`n`$env:PATH += `";C:\path\to\solidspec\target\release`""
```

---

**Verify the installation:**

```bash
solidspec --version
# solidspec 0.1.0
```

---

## Quick Reference — Most Used Commands

```bash
# Bootstrap a new project
solidspec init --here

# Create a feature spec (auto-numbers to 001, 002, ...)
solidspec specify "Your feature description"

# See what's ready to work on (DAG-based)
solidspec status

# Generate architecture plan + data model + contracts
solidspec plan

# Generate phased task breakdown with [P] parallel markers
solidspec tasks

# Generate test scaffolds from acceptance scenarios
solidspec tests

# Run the full pipeline automatically (scaffold only, no AI agent)
solidspec pipeline --new "Feature name" --no-agent

# Propose a change to an existing feature (brownfield)
solidspec change propose "Add social login" --feature-id 001
```

---

## Quick Start (3 commands)

### 1. Initialize your project

```bash
mkdir my-app && cd my-app

# Create .claude/ or .vibe/ or .github/ directory for your agent
mkdir .claude

# Initialize SolidSpec (auto-detects your AI agent)
solidspec init --here
```

SolidSpec creates:
- `.solidspec/` &mdash; constitution, templates, config
- `specs/` &mdash; where feature artifacts live
- `solidspec.toml` &mdash; project configuration
- `.claude/commands/solidspec-*.md` &mdash; 9 slash commands for your agent

### 2. Describe your feature

```bash
solidspec specify "TODO list with CRUD operations and local storage"
```

This creates `specs/001-todo-list-crud/spec.md` with:
- Prioritized user stories (P1, P2, P3)
- Functional requirements (FR-001, FR-002...)
- Acceptance scenarios (Given/When/Then)
- Quality checklist

### 3. Plan the architecture

```bash
solidspec plan 001
```

Generates `plan.md`, `research.md`, `data-model.md`, `quickstart.md`, and `contracts/`.

### 4. Generate tasks

```bash
solidspec tasks 001
```

Produces a phased task breakdown:

```
Phase 1: Setup
  - [ ] T001 Create project structure
  - [ ] T002 Initialize dependencies

Phase 2: Foundational
  - [ ] T003 Setup data models
  - [ ] T004 [P] Create Task model in src/models/task.rs

Phase 3: User Story 1 - Add a task (P1)
  - [ ] T005 [US1] Implement add task functionality
  - [ ] T006 [P] [US1] Add validation and error handling

Phase 4: User Story 2 - View tasks (P1)
  ...
```

### 5. Let your AI agent build it

Use the slash command in your AI agent:

```
/solidspec-implement
```

---

## SDD vs IDSD: Which to Choose?

Both workflows share the same eight-phase pipeline and the same artifacts. IDSD extends the chain by one phase on each side (intent capture before spec, evidence collection after implement) and enriches `analyze` and `review` with additional metrics.

### Where SDD falls short

| SDD limitation | Impact |
|----------------|--------|
| Spec describes *what* but not *why* | Implementations can satisfy requirements while missing the actual user need |
| No measurement of whether the code achieves the original goal | Drift goes undetected until user feedback |
| Requirements → tasks traceability stops at `tasks.md` | No end-to-end chain to test files |
| Review quality is heuristic-only | No evidence that acceptance criteria were actually implemented |
| "All tests green" can mask intent drift | Technically correct, functionally wrong |

### What IDSD adds

| IDSD addition | Benefit |
|---------------|---------|
| `intent.md` (ICE model) | Anchors the *why* before the *what*; changes must justify themselves against the intent |
| Intent drift score | `solidspec analyze` reports % of evidence criteria not yet covered — visible in every run |
| Evidence-based validation | `solidspec evidence` maps each evidence criterion to implemented tests; produces a satisfaction report |
| Full traceability chain | `INT-001 → FR-001 → T001 → test_file.md` — visible as ASCII tree in `solidspec analyze` |
| `IntentAlignment` review dimension | Scores 0–10: traces every FR to an evidence criterion, flags draft-status intent |
| Intent coverage metric | % of evidence criteria with at least one passing implemented test |

### Decision table

| Situation | Choose |
|-----------|--------|
| Greenfield feature with uncertain scope | **IDSD** — intent anchors the why before requirements proliferate |
| Feature subject to compliance or audit | **IDSD** — `evidence-report.md` is a traceable, versioned artefact |
| Long-lived feature that will evolve over many iterations | **IDSD** — drift score catches requirement creep automatically |
| Team suspects implementation diverged from the original vision | **IDSD** — run `solidspec analyze` to quantify the gap |
| Rapid prototype or spike with well-known requirements | **SDD** — less ceremony, faster to first output |
| Brownfield change to an existing feature | **SDD** + `solidspec change propose` — delta specs are lighter than full intent capture |
| Simple internal utility with no ambiguity | **SDD** — the `minimal` schema (4 phases) is sufficient |

### Rule of thumb

> Use **SDD** when you know *what* to build. Use **IDSD** when you need to prove *why* it was built and *whether* it was built correctly.

### IDSD Quick Start

```bash
# One command runs the full IDSD pipeline (scaffold only, no agent needed)
solidspec pipeline --new "Allow users to manage tasks" --schema intent-driven --no-agent
```

Or step by step:

```bash
# 0. Capture the intent — fill in Goal, Constraints, Evidence, then set Status: active
solidspec intent "Allow users to manage tasks"

# 1–8. Standard SDD phases (clarify, plan, tasks, tests, implement, analyze, review)
solidspec specify "Task manager with CRUD and local persistence"
solidspec plan 001
solidspec tasks 001          # manually add [FR-001] tags to tasks for trace links
solidspec tests 001          # add // T001 comments to test files for Task→Test links
solidspec implement 001      # AI agent handoff

# 9. Collect evidence — see per-criterion satisfaction
solidspec evidence 001
solidspec evidence 001 --update   # rewrite intent.md Status automatically

# Full traceability chain + drift score in every analyze run
solidspec analyze 001
```

For a complete walkthrough with a "Task Manager" example, see [docs/idsd-workflow-guide.md](docs/idsd-workflow-guide.md).

---

## Parallel Fan-Out Ship Gate

`solidspec ship` runs four specialized review lanes **concurrently** and aggregates their scores into a single binary **SHIP / HOLD** decision. Each lane invokes a dedicated AI agent with a deep-focus prompt limited to its review cluster, then extracts a 0–100 score from the agent output.

```
  solidspec ship 001
                    │
        ┌───────────┼───────────┐───────────┐
        ▼           ▼           ▼           ▼
   Code Review  Security    Test        Performance
   (completeness  Audit     Coverage    (pagination,
    clarity,     (OWASP,    (GWT        caching,
    consistency)  PII,       coverage,   load)
                  auth)      traceability)
        │           │           │           │
        └───────────┴───────────┴───────────┘
                        │
               aggregate_results()
                        │
                ┌───────┴────────┐
                │  SHIP ✓        │  HOLD ✗
                │  all lanes ≥   │  any lane < threshold
                │  threshold     │  OR critical finding
                │                │  in security lane
                └───────┬────────┘
                        │
              ship-report.md
              <!-- ship: true|false -->
```

### Quick start

```bash
# Run all 4 lanes with heuristics (no AI agent needed)
solidspec ship --no-agent

# Run with AI agents (configured in solidspec.toml)
solidspec ship 001

# Preview the planned lanes without executing
solidspec ship --dry-run

# CI gate — exit 1 on HOLD
solidspec ship --fail-on-hold

# Run only selected lanes
solidspec ship --lane code,security

# Ignore timed-out lanes (treat as non-blocking for CI)
solidspec ship --ignore-timeout --fail-on-hold
```

### Review lanes

| Lane | Focus | Default threshold |
|------|-------|-------------------|
| `code` | Completeness · Clarity · Consistency · Maintainability | 70 |
| `security` | OWASP Top 10 · PII handling · Auth constraints · Rate limiting | 80 |
| `tests` | GWT scenario coverage · Test scaffold status · Edge-case coverage | 70 |
| `perf` | Pagination · Caching strategy · Unbounded queries · Load targets | 60 |

The security lane enforces an **unconditional block** on any `CRITICAL` finding, regardless of score or `block_on_critical` config.

### Score extraction

Each lane prompt instructs the agent to end its response with `SCORE: N` (0–100). The last `SCORE:` match wins. If the agent omits the score line, a fallback counts `SEVERITY: LEVEL` keywords and applies the penalty formula: `100 - 10×CRITICAL - 5×HIGH - 2×MEDIUM - 0.5×LOW`.

`--no-agent` mode runs `solidspec review` heuristics and filters findings to each lane's dimension cluster, so scores are never placeholder zeros — a clean spec scores 100, a flawed one is penalized.

### HOLD triggers (in priority order)

1. `TimedOut` lane (skip with `--ignore-timeout`)
2. `Failed` lane (agent crash or not available)
3. `CRITICAL` finding in the `security` lane (always blocks)
4. `CRITICAL` finding in any lane when `block_on_critical = true`
5. Lane score below its threshold

### Configuration (`solidspec.toml`)

```toml
[fan_out]
# Per-lane agent overrides (falls back to [ai].default_agent)
code_agent     = "claude"
security_agent = "gemini"
tests_agent    = "claude"
perf_agent     = "claude"

# Pass/fail thresholds (0–100)
code_threshold     = 70
security_threshold = 80
tests_threshold    = 70
perf_threshold     = 60

# Block on any Critical finding in any lane (not just security)
block_on_critical = true

# Per-lane timeout in seconds
timeout = 300
```

### CLI overrides

```bash
# Override the agent for a single lane at the command line
solidspec ship --code-agent claude --security-agent gemini

# Adjust timeout for slow agents
solidspec ship --timeout 600

# Filter to a subset of lanes
solidspec ship --lane security,tests
```

### Output

`solidspec ship` writes `specs/<NNN>-<feature>/ship-report.md` with:

```markdown
# Ship Report: 001-auth-system

<!-- ship: true -->
<!-- generated: 2026-06-09T10:30:00Z -->

**Decision**: SHIP

## Lane Scores

| Lane | Agent | Score | Threshold | Status |
|------|-------|-------|-----------|--------|
| Code Review    | claude | 88/100 | 70 | ✓ Pass |
| Security Audit | gemini | 92/100 | 80 | ✓ Pass |
| Test Coverage  | claude | 76/100 | 70 | ✓ Pass |
| Performance    | claude | 65/100 | 60 | ✓ Pass |

## Re-run

\`\`\`bash
solidspec ship 001-auth-system
\`\`\`
```

The `<!-- ship: true|false -->` header is machine-readable for CI parsing.

### CI / CD integration

```yaml
# GitHub Actions example
- name: Ship gate
  run: solidspec ship --fail-on-hold --ignore-timeout
  # Exits 0 on SHIP, 1 on HOLD
```

```bash
# Pre-merge hook
solidspec ship --fail-on-hold --lane security,code && git push
```

---

## Using with Claude Code

Claude Code gets 9 slash commands automatically registered in `.claude/commands/`.

### Setup

```bash
# Ensure .claude/ exists (Claude Code creates it automatically)
mkdir -p .claude

# Initialize SolidSpec
solidspec init --here
```

You'll see:
```
Registered commands for 1 agent(s): claude
```

### Available slash commands

| Slash Command | What it does |
|---------------|-------------|
| `/solidspec-specify` | Create a new feature spec from a description |
| `/solidspec-clarify` | Resolve ambiguities in a spec |
| `/solidspec-plan` | Generate architecture plan + supporting docs |
| `/solidspec-tasks` | Generate phased task breakdown |
| `/solidspec-implement` | Execute tasks from the breakdown |
| `/solidspec-tests` | Generate and enhance test scaffolds |
| `/solidspec-analyze` | Validate cross-artifact consistency |
| `/solidspec-review` | Review spec quality with preflight heuristics |
| `/solidspec-checklist` | Generate quality validation checklist |

### Step-by-step with Claude Code

**Step 1** &mdash; Open your project in Claude Code and run:

```
/solidspec-specify Simple TODO app with add, edit, delete, and local storage
```

Claude reads the AGENT.md context, creates a feature branch, and generates `spec.md` with structured user stories, requirements, and acceptance scenarios.

**Step 2** &mdash; Review and refine the spec, then:

```
/solidspec-plan
```

Claude generates the architecture plan, data model, API contracts, and research document. Constitution gates are checked automatically.

**Step 3** &mdash; Generate tasks:

```
/solidspec-tasks
```

Claude creates `tasks.md` with phased, parallelizable tasks linked to user stories.

**Step 4** &mdash; Build it:

```
/solidspec-implement
```

Claude reads the task list and implements each task in order, respecting dependencies and `[P]` parallel markers. Completed tasks are marked `[X]`.

**Step 5** &mdash; Validate:

```
/solidspec-analyze
```

Claude checks that all requirements trace to plan sections, all plan sections trace to tasks, and the constitution is respected.

---

## Using with Mistral Vibe

Mistral Vibe gets 9 skills registered as directories in `.vibe/skills/`. Each skill has a `SKILL.md` with the `user-invocable: true` frontmatter so it appears in Vibe's slash command list.

### Setup

```bash
mkdir -p .vibe
solidspec init --here
```

You'll see:
```
Registered commands for 1 agent(s): vibe
```

Skills are created at:
```
.vibe/skills/
  solidspec-specify/SKILL.md
  solidspec-clarify/SKILL.md
  solidspec-plan/SKILL.md
  solidspec-tasks/SKILL.md
  solidspec-implement/SKILL.md
  solidspec-tests/SKILL.md
  solidspec-analyze/SKILL.md
  solidspec-review/SKILL.md
  solidspec-checklist/SKILL.md
```

### Step-by-step with Mistral Vibe

**Step 1** &mdash; In Vibe, run:

```
/solidspec-specify Real-time chat with message history and user presence
```

Vibe generates a structured spec with prioritized user stories and quality checklist.

**Step 2** &mdash; Generate the plan:

```
/solidspec-plan
```

Vibe creates the architecture plan with constitution compliance checks, data model, and API contracts.

**Step 3** &mdash; Break it into tasks:

```
/solidspec-tasks
```

**Step 4** &mdash; Implement:

```
/solidspec-implement
```

Vibe reads `tasks.md` and builds each task, marking them complete as it goes.

**Step 5** &mdash; Quality check:

```
/solidspec-analyze
```

---

## Using with GitHub Copilot

Copilot gets `.agent.md` command files in `.github/agents/` with companion `.prompt.md` files in `.github/prompts/`.

### Setup

```bash
mkdir -p .github
solidspec init --here
```

You'll see:
```
Registered commands for 1 agent(s): copilot
```

### How it works

Copilot commands are registered as:
- `.github/agents/solidspec-specify.agent.md`
- `.github/agents/solidspec-plan.agent.md`
- `.github/agents/solidspec-tasks.agent.md`
- `.github/agents/solidspec-implement.agent.md`
- etc.

Each also gets a companion `.github/prompts/solidspec-*.prompt.md`.

### Step-by-step with Copilot

The workflow is the same as Claude Code &mdash; use the slash commands:

```
/solidspec-specify E-commerce cart with checkout and payment
/solidspec-plan
/solidspec-tasks
/solidspec-implement
/solidspec-analyze
```

---

## Using Multiple Agents Together

SolidSpec registers commands for **all detected agents simultaneously**. If your project has both `.claude/` and `.vibe/`:

```bash
mkdir .claude .vibe
solidspec init --here
# Registered commands for 2 agent(s): claude, vibe
```

Both agents get the same commands and work from the same spec artifacts. You can:
- Use Claude Code for specification and planning
- Switch to Vibe for implementation
- Use either for analysis
- Or automate everything with `solidspec pipeline` &mdash; assign agents per phase in `solidspec.toml`

The artifacts in `specs/` are agent-agnostic &mdash; any agent can read and build from them.

### Automated multi-agent pipeline

```bash
# Configure agent assignments in solidspec.toml, then:
solidspec pipeline --new "Todo list REST API" --auto
```

The pipeline invokes each agent's CLI automatically. Claude Code gets `-p` with `--allowedTools`, Vibe gets `-p` (auto-approve). Agents that don't have CLI support fall back to manual handoff.

---

## Use Cases

### Use Case 1: New project from scratch

You're starting a brand new project and want structured, AI-driven development from day one.

**Scenario:** You're building a personal finance tracker as a web app.

#### Step 1 &mdash; Create and initialize the project

```bash
mkdir finance-tracker && cd finance-tracker

# Set up your AI agent directory
mkdir .claude    # or .vibe, .github, etc.

# Initialize SolidSpec
solidspec init --here
```

Your project now has:
```
finance-tracker/
  .solidspec/          # Constitution, templates, config
  .claude/commands/    # 7 slash commands for Claude Code
  specs/               # Empty — ready for features
  solidspec.toml       # Project config
  .git/                # Git repo with initial commit
```

#### Step 2 &mdash; Specify your first feature

```bash
solidspec specify "Dashboard showing income, expenses, and monthly balance with charts"
```

SolidSpec creates a feature branch `001-dashboard-showing-income-expenses`, generates `spec.md` with user stories, requirements, and a quality checklist. Edit the spec to refine it.

#### Step 3 &mdash; Plan and generate tasks

```bash
solidspec plan 001
solidspec tasks 001
```

You now have a full plan (architecture, data model, API contracts) and a phased task list ready for your AI agent.

#### Step 4 &mdash; Build with your AI agent

Open the project in Claude Code (or Vibe, Copilot, etc.) and run:

```
/solidspec-implement
```

The agent reads the task list and builds each task in order. When done, validate:

```
/solidspec-analyze
```

#### Step 5 &mdash; Add the next feature

```bash
solidspec specify "Transaction import from CSV and bank API"
```

SolidSpec auto-numbers it `002`, creates a new branch, and the cycle repeats. Each feature is self-contained in its own `specs/002-*` directory.

---

### Use Case 2: Adding SolidSpec to an existing project

You have an existing codebase and want to use SolidSpec for new features going forward.

**Scenario:** You have a Node.js API that's been running for 6 months. You want to add a notification system using structured SDD.

#### Step 1 &mdash; Initialize SolidSpec in your existing repo

```bash
cd ~/projects/my-existing-api

# Create your AI agent directory if it doesn't exist
mkdir -p .claude

# Initialize SolidSpec without overwriting anything
solidspec init --here
```

SolidSpec adds its own directories (`.solidspec/`, `specs/`) without touching your existing code. Your `.gitignore`, `package.json`, `src/`, etc. are untouched.

```
my-existing-api/
  src/                 # Your existing code — untouched
  package.json         # Your existing config — untouched
  .solidspec/          # NEW: SolidSpec config + templates
  .claude/commands/    # NEW: 7 slash commands
  specs/               # NEW: empty, ready for features
  solidspec.toml       # NEW: project config
```

#### Step 2 &mdash; Edit the constitution for your project

The default constitution assumes a greenfield project. For an existing project, edit `.solidspec/constitution.md` to match your team's actual principles:

```bash
# Open and customize
$EDITOR .solidspec/constitution.md
```

For example, you might:
- Change "Library-First" to match your monorepo structure
- Add your team's testing conventions
- Reference your existing API patterns

#### Step 3 &mdash; Specify the new feature

```bash
solidspec specify "Real-time notification system with email, push, and in-app channels"
```

SolidSpec creates `specs/001-real-time-notification-system/spec.md`. Edit the spec to reference your existing codebase:

- Mention existing models the notification system needs to integrate with
- Reference your existing auth middleware
- Note your current database and message queue setup

#### Step 4 &mdash; Plan with awareness of existing code

```bash
solidspec plan 001
```

Edit `plan.md` to reference your existing architecture:
- Point to existing services the new feature depends on
- Reference your current test infrastructure
- Note existing patterns to follow

#### Step 5 &mdash; Generate and execute tasks

```bash
solidspec tasks 001
```

The task list includes a Foundational phase for integration points with your existing code. Use your AI agent:

```
/solidspec-implement
```

The agent builds the notification system, following the plan that's aware of your existing codebase.

#### Step 6 &mdash; Continue with more features

```bash
solidspec specify "User preference center for notification settings"
# Auto-numbered as 002, new branch, new spec
```

Each new feature follows the same structured workflow. Old code stays untouched, new features are fully specified and traced.

---

### Use Case 3: Ship gate before merging a feature

You have finished implementing a feature and want a structured, multi-dimensional quality gate before merging to main.

**Scenario:** Feature `001-auth-system` is implemented. You want a SHIP / HOLD decision before the PR is opened.

#### Step 1 — Quick smoke run (no AI agent)

```bash
# Heuristic-only run — instant, no agent tokens consumed
solidspec ship --no-agent

# Output:
# Ship Assessment: 001-auth-system
#
# Launching 4 review lanes (concurrent)...
#   ✓ Code Review      (claude)   88/100  done in 0.0s
#   ✓ Security Audit   (claude)   91/100  done in 0.0s
#   ✗ Test Coverage    (claude)   62/100  done in 0.0s  ← below threshold (70)
#   ✓ Performance      (claude)   78/100  done in 0.0s
#
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#   Ship Decision: HOLD ✗
#
#   Blocking issues (2 findings):
#   [TESTS/HIGH]   Missing test scaffold for US3 password-reset scenario
#   [TESTS/MEDIUM] No edge-case tests for invalid token expiry
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#
# Report: specs/001-auth-system/ship-report.md
# Re-run: solidspec ship 001-auth-system
```

The heuristic run identifies gaps in test coverage using `solidspec review` findings filtered to the `tests` dimension cluster.

#### Step 2 — Fix the identified gaps and re-run

After adding the missing test scaffolds:

```bash
solidspec ship --no-agent
# Ship Decision: SHIP ✓
```

#### Step 3 — Full AI agent run before the PR

Once heuristics pass, run the full AI review for deeper analysis:

```bash
solidspec ship 001
```

Each lane invokes the configured agent with a focused prompt. The security lane uses a stricter threshold (80 vs 70) and unconditionally blocks on any `CRITICAL` finding.

#### Step 4 — CI gate

Add to your CI pipeline:

```yaml
# .github/workflows/ship-gate.yml
- name: Fan-out ship gate
  run: solidspec ship --fail-on-hold --timeout 300
  # Exits 0 → SHIP, exits 1 → HOLD (PR blocked)
```

Or as a pre-push hook:

```bash
# .git/hooks/pre-push
solidspec ship --fail-on-hold --no-agent --lane security,code
```

---

### Use Case 4: Security-focused review for a payment feature

You're adding a payment integration and want to ensure the security lane gets extra scrutiny.

**Scenario:** Feature `003-stripe-integration` — you want the security lane run by a specific model with a higher threshold.

#### Step 1 — Override the security lane agent and threshold at runtime

```bash
solidspec ship 003 --security-agent gemini --lane security
```

Runs only the security lane, using Gemini as the reviewer. The threshold still comes from `solidspec.toml` (default 80).

#### Step 2 — Configure a permanent override in `solidspec.toml`

```toml
[fan_out]
security_agent     = "gemini"
security_threshold = 90          # Stricter: 90/100 required for payment features
block_on_critical  = true        # Any Critical finding in any lane blocks ship
```

```bash
# Now all runs use Gemini for security with 90% threshold
solidspec ship 003
```

#### Step 3 — Dry run to preview the lane configuration

```bash
solidspec ship --dry-run

# Ship Assessment (dry run): 003-stripe-integration
#
# Lane               Agent        Threshold
# --------------------------------------------
# Code Review        claude       70
# Security Audit     gemini       90
# Test Coverage      claude       70
# Performance        claude       60
#
# No files created (dry run).
```

---

### Use Case 5: Run a subset of lanes for a fast hotfix review

You've applied a hotfix and only need to re-check code quality and security — test coverage and performance haven't changed.

```bash
# Run only the code and security lanes
solidspec ship --lane code,security --no-agent

# Or with AI agents for a thorough security check
solidspec ship --lane code,security --security-agent gemini
```

The filtered report shows only the two executed lanes:

```markdown
## Lane Scores

| Lane           | Agent  | Score  | Threshold | Status  |
|----------------|--------|--------|-----------|---------|
| Code Review    | claude | 92/100 | 70        | ✓ Pass  |
| Security Audit | gemini | 88/100 | 80        | ✓ Pass  |
```

---

### Use Case 6: IDSD workflow with full traceability + ship gate

Using the `intent-driven` schema, every artifact traces back to the original intent. The ship gate is the final artifact in the IDSD chain.

```bash
# Full IDSD pipeline including ship gate
solidspec intent "Allow users to authenticate securely"
solidspec pipeline 001 --schema intent-driven --no-agent   # scaffold all artifacts

# After the AI agent fills in the artifacts:
solidspec evidence 001       # measure per-criterion satisfaction
solidspec analyze 001        # drift score + traceability tree
solidspec review 001         # preflight heuristics
solidspec ship 001 --no-agent   # fan-out SHIP/HOLD gate

# Check the full status — all 11 artifacts including ship
solidspec status 001 --schema intent-driven
```

The `intent-driven` schema DAG: `intent → spec → clarify → plan → tasks → tests → implement → evidence → analyze → review → ship`. All 11 artifacts in topological order, all generated and traced.

---

#### Key differences between new and existing projects

| | New project | Existing project |
|---|---|---|
| **Init** | Creates project structure from scratch | Adds `.solidspec/` and `specs/` alongside existing code |
| **Constitution** | Use defaults | Customize to match your team's existing patterns |
| **Spec writing** | Describe features freely | Reference existing models, APIs, and patterns |
| **Plan** | Greenfield architecture | Integration-aware, references existing services |
| **Tasks** | Full setup from scratch | Foundational phase focuses on integration points |
| **Git** | Clean history from first commit | Feature branches alongside your existing branches |

---

## Multi-Agent Pipeline

Run the entire SDD workflow with one command, using different AI agents per phase. The pipeline **invokes each agent's CLI automatically** to fill spec artifacts with real content &mdash; not just empty templates.

```toml
# solidspec.toml
[pipeline]
specify = "claude"       # Claude writes specs
plan = "claude"          # Claude for architecture
tasks = "claude"         # Claude for task breakdown
tests = "claude"         # Claude for test generation
implement = "vibe"       # Mistral Vibe for code
analyze = "claude"       # Claude for cross-checking
review = "claude"        # Claude for quality review
```

```bash
# Full SDD pipeline on a new feature (agents invoked automatically)
solidspec pipeline --new "User auth with OAuth" --auto

# Full IDSD pipeline — adds intent capture (phase 0) and evidence collection (phase 8)
solidspec pipeline --new "User auth with OAuth" --schema intent-driven --auto

# Partial pipeline
solidspec pipeline 001 --from plan --to tasks

# Preview without executing
solidspec pipeline 001 --dry-run

# Scaffold only — generate templates without invoking AI agents
solidspec pipeline --new "Feature name" --no-agent
solidspec pipeline --new "Feature name" --schema intent-driven --no-agent
```

### How it works

The SDD pipeline runs 9 phases in order: **specify → clarify → plan → tasks → tests → implement → analyze → review → ship**.

The IDSD pipeline runs 11 phases: **intent → specify → clarify → plan → tasks → tests → implement → evidence → analyze → review → ship**. The `intent` phase (phase 0) captures the ICE model before the spec is written. The `evidence` phase (phase 8) cross-references implemented test scaffolds against each evidence criterion in `intent.md`. The `ship` phase runs the 4-lane parallel fan-out review.

For each Auto phase (specify, clarify, plan, tasks, tests, analyze):
1. SolidSpec generates the template scaffold (spec.md, plan.md, etc.)
2. SolidSpec invokes the agent's CLI non-interactively with detailed, phase-specific instructions
3. The agent reads the scaffold and fills it with real content

The `implement` phase is always a **handoff** &mdash; it tells you which agent to open and waits for confirmation.

### Execution modes

The pipeline detects agent CLI availability upfront and reports the mode:

```
Pipeline: 001-todo-api [fully automated]     # All agents have CLI support
Pipeline: 001-todo-api [mixed mode]          # Some need manual handoff
Pipeline: 001-todo-api [scaffold-only]       # --no-agent flag used
```

If an agent's CLI is not installed, the pipeline falls back to handoff mode for that phase (shows the manual command to run).

### Supported agent CLI invocations

| Agent | CLI Binary | Non-interactive Flag |
|-------|-----------|---------------------|
| Claude Code | `claude` | `-p` + `--allowedTools` |
| opencode | `opencode` | `-p` |
| Mistral Vibe | `vibe` | `-p` (auto-approves tools) |
| Gemini CLI | `gemini` | `-p` |
| Codex CLI | `codex` | `exec` subcommand |
| Copilot CLI | `copilot` | `-p` + `--allow-all-tools` |
| Kimi Code | `kimi` | `--yolo` |
| Qwen Code | `qwen` | `-p` |
| Cursor | `cursor-agent` | `-n` |
| Auggie CLI | `auggie` | `-p` |
| CodeBuddy | `codebuddy` | `-p` |
| Roo Code | `roo-code-cli` | `--headless` |

A `pipeline-log.md` is generated in the feature directory with timestamps, agents, duration, and status per phase.

### Real-world example

A pipeline test with Claude Code (specify/plan/tasks/tests/analyze) + Mistral Vibe (implement) on a "Todo List REST API" produced:

| Phase | Agent | Duration | Output |
|-------|-------|----------|--------|
| specify | Claude | 40s | 5 user stories, 14 acceptance scenarios, 12 FRs |
| plan | Claude | 95s | Node.js + Express + SQLite stack, API contracts |
| tasks | Claude | 75s | 20 tasks across 8 phases with FR references |
| tests | Claude | 88s | 5 test files with concrete assertions |
| implement | Vibe | manual | Full CRUD API, 22 passing integration tests |

Total: ~5 minutes for automated phases, fully working API with tests.

---

## Spec-to-Test Generation

Generate runnable test scaffolds from your spec's acceptance scenarios:

```bash
solidspec tests 001
# Detected: Jest (JavaScript)
# Parsed: 6 acceptance scenarios from 4 user stories
# Generated: 4 test files
```

Auto-detects your test framework from project files (Jest, Vitest, pytest, cargo test, Go, generic). Each test has Given/When/Then comments and a failing body:

```javascript
describe('US1: Add a new task', () => {
  test('task appears in list with pending status', () => {
    // Given: the app is open
    // When: user types a task title and clicks "Add"
    // Then: the task appears in the list with status "pending"
    throw new Error('TODO: implement this test');
  });
});
```

Override framework with `--framework pytest`, preview with `--dry-run`.

---

## Generated Artifacts

For each feature, SolidSpec generates a complete artifact tree:

```
specs/001-todo-list-crud/
  spec.md                  # User stories, requirements, acceptance criteria
  clarifications.md        # Decision log with session dates
  plan.md                  # Architecture plan with constitution gates
  research.md              # Technology investigation
  data-model.md            # Entity definitions and relationships
  quickstart.md            # Key validation scenarios
  contracts/               # API specifications
    api.md
  tasks.md                 # Phased task breakdown
  tests/                   # Test scaffolds from acceptance scenarios
    us1_add_task.test.js
    us2_view_tasks.test.js
  checklists/
    requirements.md        # Quality validation checklist
  analysis-report.md       # Cross-artifact consistency report
  review-report.md         # Preflight heuristic review report
  ship-report.md           # Fan-out SHIP/HOLD decision (<!-- ship: true|false -->)
  pipeline-log.md          # Pipeline execution log (agents, timing, status)
```

All artifacts are Markdown. All are version-controlled. All trace back to the original spec.

---

## Agent Guardrails

Every command and pipeline prompt includes built-in guardrails that prevent the most common AI agent shortcuts:

| Agent excuse | Built-in rebuttal |
|-------------|-------------------|
| "I'll add tests after" | Tests written after cover 30% fewer edge cases |
| "This spec section is boilerplate" | Every section serves a purpose — empty = incomplete |
| "The constitution check is unnecessary here" | Constitution gates are NON-NEGOTIABLE |
| "I can just infer the missing requirements" | Inferred requirements diverge — make them explicit |
| "It works — ship it" | "It works" is not a review |
| "I'll update the docs later" | Docs and code rot at different rates |

Each pipeline phase also gets a **specialized persona** — the agent is given a role (Spec Writer, Architect, Test Engineer, Code Reviewer, etc.), an expected output format, and a mission checklist of verification items before it can consider its work complete.

These guardrails are injected automatically into every agent prompt — no configuration needed.

---

## Project Context Configuration

SolidSpec can inject project-specific conventions into every agent prompt via the `[context]` section in `solidspec.toml`:

```toml
[context]
description = """
We use Rust edition 2024.
Testing: inline #[cfg(test)] mod per file.
Architecture: strict cli/core layering.
"""

[context.rules]
spec = "Use **FR-###**: format. Every user story needs Given/When/Then."
plan = "Document decisions with rationale. Complete the Constitution Check."
tasks = "Tasks under 2h. Mark parallel-safe with [P]."
implement = "One task at a time. Update checkboxes as you go."
review = "Check for placeholders, ambiguous language, traceability gaps."
```

When configured, the context appears at the top of every agent prompt, keeping the agent aligned with project conventions without repetition.

---

## DAG-Based Workflow (Schema-Driven)

SolidSpec uses a **DAG (Directed Acyclic Graph)** artifact engine instead of a rigid linear pipeline. Workflows are defined as dependency graphs in YAML schema files:

```yaml
# schemas/spec-driven/schema.yaml
artifacts:
  - id: spec
    generates: ["spec.md"]
    requires: []
  - id: plan
    generates: ["plan.md"]
    requires: ["spec"]
  - id: tasks
    generates: ["tasks.md"]
    requires: ["spec", "plan"]
  - id: implement
    generates: ["tasks.md"]
    requires: ["tasks"]
```

Artifacts can be created in **any order** as long as their dependencies are met. The tool computes what's ready via Kahn's algorithm topological sort.

### Workflow status at a glance

```bash
solidspec status 001

# Feature: 001-auth  |  Schema: spec-driven (built-in)
# 8 artifacts, 2 complete, 3 ready
#
# #   Artifact   Status            Depends On
# --------------------------------------------------------
# 1   spec       ✓ done            —
# 2   clarify    ▶ ready           spec
# 3   plan       ✓ done            spec
# 4   tasks      ⏸ blocked (plan)  spec, plan
# 5   tests      ▶ ready           spec
# ...
```

### Built-in workflow schemas

| Schema | Artifacts | Use case |
|--------|-----------|----------|
| `spec-driven` (default) | 9 | Full SDD pipeline, constitution gates, review, ship gate |
| `minimal` | 4 | Spec → Plan → Tasks → Implement |
| `security-first` | 5 | Adds mandatory OWASP security review before tasks |
| `intent-driven` | 11 | Full IDSD pipeline — adds intent capture (phase 0), evidence collection (phase 8), drift detection, full traceability chain, and ship gate |

### Custom workflows

Drop a `schema.yaml` in `.solidspec/workflows/<name>/` and use it via `--schema`:

```bash
solidspec status 001 --schema custom
solidspec pipeline 001 --schema security-first
```

Schema resolution: project-local → built-in → default fallback.

---

## Change-Based Workflow (Delta Specs)

For **brownfield modifications** — changing existing features without rewriting the entire spec. Each change is a lightweight folder with a proposal, delta specs (ADDED/MODIFIED/REMOVED), and tasks:

```bash
# Propose a change to an existing feature
solidspec change propose "Add social login" --feature-id 001

# List active changes
solidspec change list --feature-id 001

# Archive when done (merges deltas into main spec)
solidspec change archive add-social-login --feature-id 001
```

### How delta specs work

Instead of restating the full specification, delta specs describe only what changed:

```markdown
# Delta Spec: Add social login

## Added Requirements
- **FR-042**: System MUST support OAuth2 login via Google
- **FR-043**: System MUST support OAuth2 login via GitHub

## Modified Requirements
- **FR-012**: User profile MUST include OAuth provider (was: email only)

## Removed Requirements
- FR-008
```

On archive, SolidSpec automatically merges deltas into the main `spec.md` — preserving existing requirements, updating modified ones in-place, and appending new ones.

### Change directory structure

```
specs/001-auth/
├── spec.md                        # Main spec (current)
├── plan.md
├── tasks.md
├── changes/                        # Active change proposals
│   ├── add-social-login/
│   │   ├── proposal.md             # Why + what + impact + non-goals
│   │   ├── delta-spec.md           # ADDED/MODIFIED/REMOVED
│   │   └── .change.yaml            # Metadata (status, created_at)
│   └── add-two-factor/
│       └── ...
└── archive/                        # Completed changes
    └── add-dark-mode/
        └── ...
```

---

## Project Constitution

Every SolidSpec project gets a `constitution.md` defining architectural principles:

| Gate | What it checks |
|------|---------------|
| **Simplicity** | Max 3 projects, no speculative features |
| **Anti-Abstraction** | Use frameworks directly, no wrapper layers |
| **Integration-First** | Contract tests before implementation, real services over mocks |

The `plan` command evaluates these gates automatically. Violations are reported but don't block &mdash; the plan is generated with warnings so you can decide how to proceed.

---

## Template System

Templates control what gets generated. They follow a 4-layer priority hierarchy:

| Priority | Location | Use case |
|----------|----------|----------|
| 1 (highest) | `.solidspec/templates/overrides/` | Project-specific tweaks |
| 2 | `.solidspec/presets/<id>/templates/` | Team workflow presets |
| 3 | `.solidspec/extensions/<id>/templates/` | Extension templates |
| 4 (lowest) | Embedded in binary | Defaults |

Install a custom preset:

```bash
solidspec preset add ./my-team-preset --priority 5
```

---

## All Commands

### SDD commands

| Command | Description |
|---------|-------------|
| `solidspec init [name]` | Initialize project with constitution, templates, agent commands |
| `solidspec specify <desc>` | Create feature spec with user stories and quality checklist |
| `solidspec clarify [id]` | Resolve `[NEEDS CLARIFICATION]` markers |
| `solidspec plan [id]` | Generate plan + research + data model + contracts |
| `solidspec tasks [id]` | Generate phased task breakdown with `[P]` parallel markers |
| `solidspec implement [id]` | Execute tasks with hook support and `--pass` for iterations |
| `solidspec tests [id]` | Generate test scaffolds from Given/When/Then scenarios (`--framework`) |
| `solidspec analyze [id]` | Validate consistency (read-only) with severity levels; shows trace tree and drift in IDSD mode |
| `solidspec review [id]` | Review spec quality with preflight heuristics and dimension scoring (8 dimensions in IDSD mode) |
| `solidspec checklist [id]` | Generate/append quality checklists (`--append`) |
| `solidspec ship [id]` | Run parallel fan-out review (4 concurrent AI lanes) → `SHIP` / `HOLD` decision + `ship-report.md` (`--lane`, `--no-agent`, `--fail-on-hold`, `--dry-run`, `--timeout`, `--ignore-timeout`) |
| `solidspec pipeline [id]` | Run multi-agent pipeline (`--new`, `--from`, `--to`, `--auto`, `--no-agent`, `--schema`) |
| `solidspec status [id]` | Show artifact completion status (DAG-based, `--schema`); shows intent drift in IDSD mode |
| `solidspec change <cmd>` | Manage changes: `propose "Title"`, `list`, `archive <slug>` (`--feature-id`) |
| `solidspec preset <cmd>` | Manage presets (`add`, `remove`, `list`, `search`, `info`) |
| `solidspec extension <cmd>` | Manage extensions (`add`, `remove`, `enable`, `disable`, `list`) |
| `solidspec upgrade` | Refresh templates + agent commands after update |
| `solidspec completions <shell>` | Generate shell completions (bash, zsh, fish, powershell) |
| `solidspec check` | Verify system prerequisites |

### IDSD-only commands

| Command | Description |
|---------|-------------|
| `solidspec intent <title>` | Capture intent using the ICE model (`intent.md`): Goal, Constraints, Evidence, Risks, Open Questions. IDSD phase 0. |
| `solidspec evidence [id]` | Cross-reference evidence criteria from `intent.md` against implemented test scaffolds. Prints a per-criterion satisfaction table and writes `evidence-report.md`. Add `--update` to rewrite `intent.md` Status automatically (`active` / `satisfied` / `drifted`). IDSD phase 8. |

Feature ID is auto-detected from git branch or latest spec if omitted.

---

## Supported AI Agents (20)

| Agent | Directory | Format |
|-------|-----------|--------|
| Claude Code | `.claude/commands/` | Markdown |
| Mistral Vibe | `.vibe/skills/` | Markdown (directory-based) |
| GitHub Copilot | `.github/agents/` | Markdown + `.prompt.md` |
| Gemini CLI | `.gemini/commands/` | TOML |
| Cursor | `.cursor/commands/` | Markdown |
| Windsurf | `.windsurf/workflows/` | Markdown |
| Codex CLI | `.codex/prompts/` | Markdown |
| Kiro CLI | `.kiro/prompts/` | Markdown |
| Kimi Code | `.kimi/skills/` | Markdown (directory-based) |
| Tabnine CLI | `.tabnine/agent/commands/` | TOML |
| Qwen Code | `.qwen/commands/` | Markdown |
| opencode | `.opencode/skills/` | Markdown (SKILL.md) |
| Kilo Code | `.kilocode/workflows/` | Markdown |
| Auggie CLI | `.augment/commands/` | Markdown |
| Roo Code | `.roo/commands/` | Markdown |
| CodeBuddy | `.codebuddy/commands/` | Markdown |
| Qoder CLI | `.qoder/commands/` | Markdown |
| Amp | `.agents/commands/` | Markdown |
| SHAI | `.shai/commands/` | Markdown |
| IBM Bob | `.bob/commands/` | Markdown |

---

## Configuration

### `solidspec.toml`

```toml
[project]
name = "my_project"
version = "0.1.0"

[ai]
default_agent = "claude"

[git]
auto_branch = true
auto_commit = true

[context]
description = "We use Rust edition 2024 with strict layering."
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `SOLIDSPEC_AI` | Override default AI agent |
| `SOLIDSPEC_FEATURE` | Override feature detection (skip git/scan) |
| `GH_TOKEN` | GitHub API authentication |

---

## Development

```bash
# Run all tests
cargo test

# Build release binary
cargo build --release

# Generate shell completions
solidspec completions bash > ~/.bash_completion.d/solidspec
solidspec completions zsh > ~/.zfunc/_solidspec
solidspec completions fish > ~/.config/fish/completions/solidspec.fish
```

### Architecture

```
src/
  cli/          20 command handlers (clap derive) — includes intent, evidence, ship
  core/         Spec parser, planner, task generator, test generator, pipeline,
                analyzer, constitution, intent_parser, evidence, artifact_graph (trace),
                fan_out (parallel review lanes, ship gate, score extraction)
  agents/       20-agent config table, detection, format translation, registration, CLI invoker
  templates/    Tera rendering + 4-layer resolver (includes IDSD templates)
  presets/      Manifest validation, registry, manager
  extensions/   Manifest, registry, hooks, manager
  config/       TOML configuration handling (includes FanOutConfig)
schemas/
  spec-driven/  Default 9-artifact SDD workflow (includes ship gate)
  minimal/      4-artifact lightweight workflow
  security-first/ 5-artifact workflow with mandatory security review
  intent-driven/  11-artifact IDSD workflow (includes ship gate)
docs/
  idsd-workflow-guide.md              Complete IDSD walkthrough with Task Manager example
  Parallel-Fan-out_orchestration-plan.md  Fan-out ship gate design spec
```

---

## License

[MIT](LICENSE)
