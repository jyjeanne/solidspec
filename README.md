<p align="center">
  <img src="docs/picture/logo.jpg" alt="SolidSpec Logo" width="600">
  <p align="center">
    <strong>AI-Powered Software Development — Multi-Methodology, One CLI</strong>
  </p>
  <p align="center">
    SolidSpec scaffolds your AI agents into structured development workflows. Pick the methodology that fits
    your feature — from a quick spike to a fully-traced, intent-anchored production build — and SolidSpec
    drives every artifact, every agent prompt, and every quality gate from idea to ship.
  </p>
  <p align="center">
    <a href="#workflows-and-methodologies">Workflows</a> &bull;
    <a href="#choosing-a-workflow">Compare</a> &bull;
    <a href="#install">Install</a> &bull;
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

You describe a feature to your AI coding agent. It generates code. But the code doesn't match what you actually needed &mdash; scope creeps, edge cases are missed, tests are written after the fact (or not at all), and there's no traceability from requirements to implementation.

**SolidSpec fixes this** by inserting a structured layer between your idea and the code. Every feature gets a spec, a plan, and a task list &mdash; all versioned in your repo, all driving the AI. Which structure exactly depends on what you're building:

- Shipping a quick internal tool? Use `minimal` &mdash; four artifacts, done in minutes.
- Building a payment feature? Use `security-first` &mdash; OWASP audit gates the task list.
- Writing a library with strict contracts? Use `tdd-driven` &mdash; real failing tests before any implementation.
- Building a product feature with uncertain scope? Use `intent-driven` &mdash; capture the *why* first, measure it at the end.
- Tackling a complex feature that needs structured implementation? Use `apex-driven`.
- Need all of the above? Use `intent-apex`.

**SolidSpec works with 20 AI agents** (Claude Code, Copilot, Vibe, Gemini, Cursor, Windsurf, Codex, and more), registers slash commands in each agent's native format, and can invoke them automatically via a fully automated or mixed-mode pipeline.

---

## Workflows and Methodologies

SolidSpec ships **7 built-in workflows** covering the full spectrum from lightweight to rigorous. All share the same DAG engine, schema format, agent registration, and pipeline infrastructure.

### At a Glance

| Schema | Artifacts | Methodology | Key addition over `spec-driven` |
|--------|-----------|-------------|--------------------------------|
| `minimal` | 4 | Lean SDD | No tests, no review — just spec → plan → tasks → implement |
| `spec-driven` | 9 | Full SDD | Constitution gates, test scaffolds, analyze, review, ship gate |
| `security-first` | 5 | SDD + security | Mandatory OWASP audit gates the task list |
| `tdd-driven` | 10 | AI-TDD | Real failing tests (RED) before implementation; refactor phase after |
| `intent-driven` | 11 | IDSD | Intent capture (WHY), evidence collection, drift detection |
| `apex-driven` | 9 | SDD + APEX | Structured A-P-E-X implementation replaces manual handoff |
| `intent-apex` | 11 | IDSD + APEX | Intent-anchored with APEX implementation and evidence collection |

---

### `minimal` — Lean Specification

The fastest path from idea to implementation. No test scaffolds, no review phase, no ship gate. Four artifacts, minimal ceremony.

```
  spec.md → plan.md → tasks.md → implement
```

**Use when:**
- Internal utility scripts or tooling with no ambiguity
- Hackathon projects or time-boxed spikes
- The requirements are fully known by the implementer
- You want the discipline of a written spec but not the full SDD ceremony

**Avoid when:** quality gates, traceability, or external stakeholders matter.

---

### `spec-driven` — Full Specification-Driven Development (default)

The standard SolidSpec workflow. Structured spec, architecture plan, phased tasks, test scaffolds, cross-artifact consistency check, preflight review, and a 4-lane parallel ship gate.

```
  spec → clarify → plan → tasks → tests → implement → analyze → review → ship
```

**Use when:**
- Most greenfield features in a team or solo project
- Adding a new capability to an existing codebase
- You need traceability from requirements to tasks but not full intent-to-evidence traceability
- Brownfield features (combine with `solidspec change propose` for delta specs)
- The default if you're unsure which schema to pick

---

### `security-first` — Security-Gated Development

Identical to `spec-driven` through the plan phase, then adds a mandatory OWASP Top 10 security review before tasks can be generated. Security findings must be resolved — every finding becomes a mitigation task.

```
  spec → plan → security-review → tasks → implement
```

**Use when:**
- Payment processing, billing, or financial transactions
- Authentication, authorization, session management, or OAuth flows
- Any feature that stores, transmits, or processes PII, credentials, or sensitive data
- Healthcare, legal, or regulated-industry features
- API endpoints exposed to the public internet
- Features that modify access control or permission models

**Key difference:** Tasks cannot be generated until `security-review.md` exists. This is a hard DAG dependency — it cannot be skipped.

---

### `tdd-driven` — AI Test-Driven Development

Brings RED-GREEN-REFACTOR discipline to AI-assisted development. The agent writes real failing tests first (not scaffolds), then implements one test at a time, then refactors with all tests green. Three human-approval gates: before writing tests, before implementing, before refactoring.

```
  spec → clarify → plan → tasks
       → tdd-tests (RED)
       → implement (GREEN — one test at a time)
       → tdd-refactor (REFACTOR — interface must not grow)
       → analyze → review → ship
```

**Use when:**
- Library code or SDK with a stable public API that multiple consumers depend on
- Complex business logic (pricing engines, rule evaluators, state machines)
- Code that will be maintained by a large team or refactored frequently
- Replacing or rewriting a working system where regressions must be prevented
- Any feature where "all tests green" is the contractual definition of done
- You want the AI to prove that each behavior works before moving to the next

**Key difference over `spec-driven`:**
- `tests` (scaffold) → `tdd-tests` (real failing tests using the project's framework)
- New `tdd-refactor` phase: the AI refactors without adding behavior
- Agent command bodies enforce: tracer-bullet first, vertical slices, mock boundaries (only external systems), interface preservation during refactor

---

### `intent-driven` — IDSD (Intent-Driven Specification Development)

Adds a root intent anchor before the spec and evidence collection after implementation. Every requirement traces back to the original intent. Drift is measured continuously.

```
  intent (WHY) → spec → clarify → plan → tasks → tests
               → implement → evidence → analyze → review → ship
```

**Use when:**
- Greenfield features with uncertain or evolving scope
- Features subject to compliance, audit, or stakeholder approval
- Long-lived features that will evolve over many iterations (drift detection prevents silent requirement creep)
- The team suspects implementation has diverged from the original vision
- You need a versioned, traceable record proving *why* each requirement exists

**Key additions:**
- `intent.md` (ICE model): Goal / Constraints / Evidence before the first spec line
- `evidence-report.md`: per-criterion satisfaction from implemented tests
- Intent drift score in every `solidspec analyze` run
- Full traceability chain: `INT-001 → FR-001 → T001 → test_file`

---

### `apex-driven` — APEX-Enhanced SDD

SDD with the APEX (Analyze-Plan-Execute-eXamine) implementation workflow replacing the manual handoff. APEX injects `spec.md + plan.md + tasks.md` as pre-loaded context and provides parallel exploration agents and adversarial code review.

```
  spec → clarify → plan → tasks → tests → apex → analyze → review → ship
```

**Use when:**
- Complex features where the implementation itself benefits from a structured A-P-E-X cycle
- Team-driven development where the agent's implementation choices need structured examination
- You want to skip the manual `implement` handoff without losing structure

---

### `intent-apex` — Intent + APEX (Maximum Rigor)

The most comprehensive workflow: intent-anchored requirements, evidence-based validation, AND structured APEX implementation. Every requirement traces to intent; every implementation cycle is structured.

```
  intent → spec → clarify → plan → tasks → tests
         → apex → evidence → analyze → review → ship
```

**Use when:**
- Enterprise or product-critical features where every decision must be justifiable
- Compliance-driven development (the evidence report is a versioned, auditable artifact)
- Complex features that are both uncertain in scope (IDSD value) AND complex in implementation (APEX value)
- When you need the maximum possible traceability, structure, and quality assurance

---

## Choosing a Workflow

### Decision table

| Situation | Recommended |
|-----------|-------------|
| Quick script or internal tool, requirements fully known | `minimal` |
| Standard team feature, new capability, brownfield addition | `spec-driven` |
| Payment, auth, PII, or any security-sensitive feature | `security-first` |
| Library, SDK, or API with strict behavioral contracts | `tdd-driven` |
| Business logic that will be refactored often | `tdd-driven` |
| Rewriting existing working code — regressions are unacceptable | `tdd-driven` |
| Feature with uncertain scope or evolving requirements | `intent-driven` |
| Compliance, audit, or stakeholder-approval required | `intent-driven` |
| Feature likely to drift from original intent over iterations | `intent-driven` |
| Complex implementation needing structured A-P-E-X execution | `apex-driven` |
| Regulated feature with both uncertain scope AND complex implementation | `intent-apex` |

### Comparison matrix

| | `minimal` | `spec-driven` | `security-first` | `tdd-driven` | `intent-driven` | `apex-driven` | `intent-apex` |
|--|:---------:|:-------------:|:----------------:|:------------:|:---------------:|:-------------:|:-------------:|
| **Artifacts** | 4 | 9 | 5 | 10 | 11 | 9 | 11 |
| **Ceremony level** | Low | Medium | Medium | Medium | High | Medium | Very High |
| **Test scaffolds** | ✗ | ✓ | ✗ | Real (RED) | ✓ | ✓ | ✓ |
| **TDD RED-GREEN-REFACTOR** | ✗ | ✗ | ✗ | ✓ | ✗ | ✗ | ✗ |
| **Security audit gate** | ✗ | ✗ | ✓ mandatory | ✗ | ✗ | ✗ | ✗ |
| **Intent capture (WHY)** | ✗ | ✗ | ✗ | ✗ | ✓ | ✗ | ✓ |
| **Evidence collection** | ✗ | ✗ | ✗ | ✗ | ✓ | ✗ | ✓ |
| **Intent drift detection** | ✗ | ✗ | ✗ | ✗ | ✓ | ✗ | ✓ |
| **APEX implementation** | ✗ | ✗ | ✗ | ✗ | ✗ | ✓ | ✓ |
| **Parallel ship gate** | ✗ | ✓ | ✗ | ✓ | ✓ | ✓ | ✓ |
| **Full trace chain** | ✗ | partial | ✗ | partial | ✓ full | partial | ✓ full |
| **Suitable for CI gate** | ✗ | ✓ | ✗ | ✓ | ✓ | ✓ | ✓ |

### Rule of thumb

> Use **`minimal`** when you know exactly what to build and speed matters.
> Use **`spec-driven`** when you need structure without overhead.
> Use **`security-first`** when trust boundaries are at stake.
> Use **`tdd-driven`** when contracts matter more than speed.
> Use **`intent-driven`** when you need to prove *why* it was built.
> Use **`apex-driven`** when the implementation itself is the hard part.
> Use **`intent-apex`** when all of the above apply.

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

**Verify the installation:**

```bash
solidspec --version
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

# Run the full pipeline automatically (scaffold only, no AI agent)
solidspec pipeline --new "Feature name" --no-agent

# Run with a specific workflow schema
solidspec pipeline --new "Feature name" --schema tdd-driven --no-agent
solidspec pipeline --new "Feature name" --schema intent-driven --no-agent
solidspec pipeline --new "Feature name" --schema security-first --no-agent

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
- `.claude/commands/solidspec-*.md` &mdash; slash commands for your agent

### 2. Choose your workflow and describe your feature

```bash
# Standard workflow (spec-driven is the default)
solidspec specify "TODO list with CRUD operations and local storage"

# Or run the full pipeline for a specific workflow in one command
solidspec pipeline --new "TODO list with CRUD" --schema spec-driven --no-agent
solidspec pipeline --new "Payment checkout flow" --schema tdd-driven --no-agent
solidspec pipeline --new "User auth system" --schema security-first --no-agent
solidspec pipeline --new "Task manager feature" --schema intent-driven --no-agent
```

### 3. Let your AI agent build it

Use the slash command in your AI agent:

```
/solidspec-implement
```

---

## SDD vs IDSD vs TDD: Which to Choose?

### Where SDD alone falls short

| SDD limitation | Impact |
|----------------|--------|
| Spec describes *what* but not *why* | Implementations can satisfy requirements while missing the actual user need |
| No measurement of whether the code achieves the original goal | Drift goes undetected until user feedback |
| Test scaffolds are stubs, not real failing tests | AI can implement without ever running a test |
| "All tests green" can mask intent drift | Technically correct, functionally wrong |

### What IDSD adds

| IDSD addition | Benefit |
|---------------|---------|
| `intent.md` (ICE model) | Anchors the *why* before the *what* |
| Intent drift score | `solidspec analyze` reports % of evidence criteria not yet covered |
| Evidence-based validation | Maps each evidence criterion to implemented tests |
| Full traceability chain | `INT-001 → FR-001 → T001 → test_file.md` |
| `IntentAlignment` review dimension | Scores 0–10 in `review-report.md` |

### What TDD adds

| TDD addition | Benefit |
|--------------|---------|
| Real failing tests before implementation | Agent cannot skip to code — tests must compile and fail first |
| Tracer-bullet first cycle | Most critical behavior proved end-to-end before expanding |
| One-test-at-a-time GREEN phase | No bulk implementation; each behavior proven in isolation |
| Dedicated REFACTOR phase | Code quality improved without adding behavior; tests protect against regression |
| Mock boundary enforcement | Only external systems mocked; internal collaborators are never mocked |

### IDSD Quick Start

```bash
# One command runs the full IDSD pipeline (scaffold only, no agent needed)
solidspec pipeline --new "Allow users to manage tasks" --schema intent-driven --no-agent
```

Or step by step:

```bash
solidspec intent "Allow users to manage tasks"
solidspec specify "Task manager with CRUD and local persistence"
solidspec plan 001
solidspec tasks 001
solidspec tests 001
solidspec implement 001
solidspec evidence 001
solidspec analyze 001
```

For a complete walkthrough, see [docs/idsd-workflow-guide.md](docs/idsd-workflow-guide.md).

### TDD Quick Start

```bash
# Full TDD pipeline scaffold
solidspec pipeline --new "Authentication service" --schema tdd-driven --no-agent
```

Or step by step (human-approval gates at tdd-tests, implement, tdd-refactor):

```bash
solidspec specify "Auth service with JWT and refresh tokens"
solidspec plan 001
solidspec tasks 001

# RED phase — agent writes real failing tests from acceptance criteria
solidspec tdd-tests 001
# Review tdd-red-report.md, then open your agent: /solidspec-tdd-tests

# GREEN phase — implement one failing test at a time
solidspec implement 001
# Agent follows the cycle table in tdd-red-report.md

# REFACTOR phase — improve without changing behavior
solidspec tdd-refactor 001
# Agent produces tdd-refactor-report.md with per-change audit
```

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

Claude Code gets slash commands automatically registered in `.claude/commands/`.

### Setup

```bash
mkdir -p .claude
solidspec init --here
```

### Available slash commands

| Slash Command | What it does |
|---------------|-------------|
| `/solidspec-specify` | Create a new feature spec from a description |
| `/solidspec-clarify` | Resolve ambiguities in a spec |
| `/solidspec-plan` | Generate architecture plan + supporting docs |
| `/solidspec-tasks` | Generate phased task breakdown |
| `/solidspec-tests` | Generate test scaffolds from acceptance scenarios |
| `/solidspec-tdd-tests` | Write real failing tests — TDD RED phase |
| `/solidspec-implement` | Execute tasks from the breakdown |
| `/solidspec-tdd-refactor` | Refactor while keeping all tests GREEN |
| `/solidspec-analyze` | Validate cross-artifact consistency |
| `/solidspec-review` | Review spec quality with preflight heuristics |
| `/solidspec-checklist` | Generate quality validation checklist |

### Step-by-step with Claude Code

**Step 1** &mdash; Specify your feature:

```
/solidspec-specify Simple TODO app with add, edit, delete, and local storage
```

**Step 2** &mdash; Plan the architecture:

```
/solidspec-plan
```

**Step 3** &mdash; Generate tasks:

```
/solidspec-tasks
```

**Step 4** &mdash; Build it:

```
/solidspec-implement
```

**Step 5** &mdash; Validate:

```
/solidspec-analyze
```

---

## Using with Mistral Vibe

Mistral Vibe gets skills registered as directories in `.vibe/skills/`.

```bash
mkdir -p .vibe
solidspec init --here
```

Skills are created at `.vibe/skills/solidspec-*/SKILL.md`. Usage is identical to Claude Code but with Vibe's `/skill-name` format.

---

## Using with GitHub Copilot

Copilot gets `.agent.md` command files in `.github/agents/` with companion `.prompt.md` files in `.github/prompts/`.

```bash
mkdir -p .github
solidspec init --here
```

---

## Using Multiple Agents Together

SolidSpec registers commands for **all detected agents simultaneously**. If your project has both `.claude/` and `.vibe/`:

```bash
mkdir .claude .vibe
solidspec init --here
# Registered commands for 2 agent(s): claude, vibe
```

Both agents get the same commands and work from the same spec artifacts. The artifacts in `specs/` are agent-agnostic — any agent can read and build from them.

### Automated multi-agent pipeline

```bash
# Configure agent assignments in solidspec.toml, then:
solidspec pipeline --new "Todo list REST API" --auto
solidspec pipeline --new "Payment checkout" --schema tdd-driven --auto
solidspec pipeline --new "Auth feature" --schema intent-driven --auto
```

---

## Use Cases

### Use Case 1: Standard greenfield feature (spec-driven)

You're starting a personal finance tracker. Full structure, test scaffolds, review, ship gate.

```bash
mkdir finance-tracker && cd finance-tracker
mkdir .claude
solidspec init --here

solidspec specify "Dashboard showing income, expenses, and monthly balance"
solidspec plan 001
solidspec tasks 001
# In Claude Code: /solidspec-implement, /solidspec-analyze
solidspec ship --no-agent    # SHIP/HOLD decision
```

---

### Use Case 2: Library or API with strict contracts (tdd-driven)

You're building an authentication library that multiple services will depend on. Behavioral contracts must be precise and regression-free.

```bash
solidspec pipeline --new "JWT authentication library" --schema tdd-driven --no-agent
```

The pipeline scaffolds:
1. `spec.md` — acceptance criteria become the exact test behaviors
2. `plan.md` — interface design and mock boundaries documented
3. `tasks.md` — per-task AC links for the one-test-one-impl cycle
4. `tdd-red-report.md` — scaffold for real failing tests (agent fills in)
5. `tests/` — empty directory for the RED-phase test files
6. `tdd-refactor-report.md` — scaffold for the REFACTOR phase audit

**Step by step:**

```bash
# Prepare the spec and plan
solidspec specify "JWT library with sign, verify, refresh"
solidspec plan 001
solidspec tasks 001

# RED phase: open agent and write all failing tests first
solidspec tdd-tests 001
# In Claude Code: /solidspec-tdd-tests
# Agent: designs interfaces, writes tracer bullet test (most critical AC),
#         writes remaining tests — all must FAIL at end of this phase

# GREEN phase: implement one test at a time
solidspec implement 001
# Agent follows the cycle table in tdd-red-report.md: one failing test targeted,
# minimal code written, full suite re-run, repeat

# REFACTOR phase: clean up without adding behavior
solidspec tdd-refactor 001
# Agent produces tdd-refactor-report.md with before/after for each change
# Every change followed by a full test run — all GREEN required

solidspec analyze 001
solidspec ship --no-agent
```

---

### Use Case 3: Payment or auth feature (security-first)

You're adding Stripe integration. OWASP audit required before any code is written.

```bash
solidspec pipeline --new "Stripe payment integration" --schema security-first --no-agent
```

The DAG enforces: `tasks.md` cannot be generated until `security-review.md` exists. The security review artifact contains OWASP findings; every finding becomes a mandatory mitigation task.

```bash
solidspec specify "Checkout with Stripe, subscription billing, refunds"
solidspec plan 001
# Plan is reviewed by AI for OWASP Top 10 issues
solidspec pipeline 001 --from security-review --schema security-first --no-agent
# Now check security-review.md: Critical/High findings must be resolved
solidspec tasks 001   # Only available after security-review.md is written
solidspec implement 001
```

---

### Use Case 4: Product feature with uncertain scope (intent-driven)

You're building a notification system for a SaaS product. The scope is unclear; stakeholders have varying expectations. You need a traceable record.

```bash
# Capture the WHY before any requirements are written
solidspec intent "Allow users to receive timely notifications across channels"
# Fill in intent.md: Goal (one sentence), Constraints, Evidence criteria, Risks

solidspec specify "Real-time notification system with email, push, in-app"
solidspec plan 001
solidspec tasks 001
solidspec tests 001
# Implement (handoff to AI agent)

solidspec evidence 001        # per-criterion satisfaction from implemented tests
solidspec evidence 001 --update  # rewrite intent.md Status automatically
solidspec analyze 001         # drift score + INT→FR→T traceability tree
solidspec review 001
solidspec ship 001 --no-agent
```

---

### Use Case 5: Adding SolidSpec to an existing project

You have a Node.js API that's been running for 6 months. New features going forward should be structured.

```bash
cd ~/projects/my-existing-api
mkdir -p .claude
solidspec init --here   # adds .solidspec/, specs/ — existing code untouched

# Edit the constitution to match your team's actual patterns
$EDITOR .solidspec/constitution.md

# New feature — reference existing models and patterns in the spec
solidspec specify "Real-time notification system with email, push, and in-app channels"
solidspec plan 001
solidspec tasks 001
# /solidspec-implement in Claude Code
```

---

### Use Case 6: Ship gate before merging a feature

Quick smoke check, then full AI review, then CI gate.

```bash
# Heuristic-only run — instant, no agent tokens consumed
solidspec ship --no-agent
# Fix any HOLD findings, then:

# Full AI review
solidspec ship 001

# CI gate
solidspec ship --fail-on-hold --ignore-timeout
```

Add to CI:
```yaml
- name: Ship gate
  run: solidspec ship --fail-on-hold --ignore-timeout
```

---

### Use Case 7: Complex enterprise feature (intent-apex)

You're building a compliance reporting module. Scope is uncertain, implementation is complex, and every decision must be auditable.

```bash
solidspec pipeline --new "Compliance reporting engine" --schema intent-apex --no-agent
```

The `intent-apex` pipeline provides:
- `intent.md` — Goal, Constraints, Evidence criteria anchoring everything
- Full SDD spec, plan, tasks
- APEX structured implementation with parallel exploration agents
- `evidence-report.md` — per-criterion satisfaction from APEX output
- Full `INT→FR→T→code` traceability chain
- Parallel ship gate with 4 review lanes

---

## Multi-Agent Pipeline

Run the entire workflow with one command, using different AI agents per phase.

```toml
# solidspec.toml
[pipeline]
specify = "claude"
plan    = "claude"
tasks   = "claude"
tests   = "claude"
implement = "vibe"
analyze = "claude"
review  = "claude"
```

```bash
# Full spec-driven pipeline
solidspec pipeline --new "User auth with OAuth" --auto

# TDD pipeline
solidspec pipeline --new "User auth with OAuth" --schema tdd-driven --auto

# IDSD pipeline
solidspec pipeline --new "User auth with OAuth" --schema intent-driven --auto

# Partial pipeline
solidspec pipeline 001 --from plan --to tasks

# Preview without executing
solidspec pipeline 001 --dry-run --schema tdd-driven

# Scaffold only — generate templates without invoking AI agents
solidspec pipeline --new "Feature name" --no-agent --schema security-first
```

### Pipeline phases per schema

| Schema | Phases (in order) |
|--------|-------------------|
| `minimal` | specify → plan → tasks → implement |
| `spec-driven` | specify → clarify → plan → tasks → tests → implement → analyze → review |
| `security-first` | specify → plan → security-review → tasks → implement |
| `tdd-driven` | specify → clarify → plan → tasks → tdd-tests → implement → tdd-refactor → analyze → review |
| `intent-driven` | intent → specify → clarify → plan → tasks → tests → implement → evidence → analyze → review |
| `apex-driven` | specify → clarify → plan → tasks → tests → apex → analyze → review |
| `intent-apex` | intent → specify → clarify → plan → tasks → tests → apex → evidence → analyze → review |

`ship` is available as a separate command (`solidspec ship`) for all schemas that include it in their DAG.

### Execution modes

```
Pipeline: 001-todo-api [fully automated]     # All agents have CLI support
Pipeline: 001-todo-api [mixed mode]          # Some need manual handoff
Pipeline: 001-todo-api [scaffold-only]       # --no-agent flag used
```

---

## Spec-to-Test Generation

Generate runnable test scaffolds from your spec's acceptance scenarios (`spec-driven`, `intent-driven`, `apex-driven`):

```bash
solidspec tests 001
# Detected: Jest (JavaScript)
# Parsed: 6 acceptance scenarios from 4 user stories
# Generated: 4 test files
```

Auto-detects your test framework from project files (Jest, Vitest, pytest, cargo test, Go, generic). Each test has Given/When/Then comments and a failing body.

**TDD mode** (`tdd-driven`) uses `solidspec tdd-tests` instead — the agent writes actual, framework-specific failing tests from acceptance criteria. The difference:

| `tests` (SDD) | `tdd-tests` (TDD) |
|---------------|-------------------|
| Scaffold with TODO body | Real assertions that fail because code is absent |
| Agent fills in later | Agent must run the tests; all must FAIL before moving on |
| Any order | Tracer bullet first, remaining in AC order |
| No interface design step | Agent designs interfaces before writing any test |

---

## Generated Artifacts

### `spec-driven` / `intent-driven`

```
specs/001-feature-name/
  spec.md                  # User stories, requirements, acceptance criteria
  plan.md                  # Architecture plan with constitution gates
  tasks.md                 # Phased task breakdown
  tests/                   # Test scaffolds from acceptance scenarios
  analysis-report.md       # Cross-artifact consistency report
  review-report.md         # Preflight heuristic review report
  ship-report.md           # Fan-out SHIP/HOLD decision
  intent.md                # (IDSD only) ICE model — Goal/Constraints/Evidence
  evidence-report.md       # (IDSD only) per-criterion satisfaction
  pipeline-log.md          # Pipeline execution log
```

### `tdd-driven`

```
specs/001-feature-name/
  spec.md
  plan.md
  tasks.md
  tdd-red-report.md        # Interface design + tracer bullet + cycle table + quality checklist
  tests/                   # Real failing tests (RED phase output)
  tdd-refactor-report.md   # Per-change audit: refactor type, before/after, test result
  analysis-report.md
  review-report.md
  ship-report.md
```

### `security-first`

```
specs/001-feature-name/
  spec.md
  plan.md
  security-review.md       # OWASP audit — findings by severity
  tasks.md                 # Every finding has a mitigation task
```

---

## Agent Guardrails

Every command and pipeline prompt includes built-in guardrails:

| Agent excuse | Built-in rebuttal |
|-------------|-------------------|
| "I'll add tests after" | Tests written after cover 30% fewer edge cases |
| "This spec section is boilerplate" | Every section serves a purpose — empty = incomplete |
| "The constitution check is unnecessary here" | Constitution gates are NON-NEGOTIABLE |
| "I can just infer the missing requirements" | Inferred requirements diverge — make them explicit |
| "It works — ship it" | "It works" is not a review |
| "I'll update the docs later" | Docs and code rot at different rates |

TDD-specific guardrails:

| TDD excuse | Built-in rebuttal |
|------------|-------------------|
| "I'll write all tests first then all code" | That is horizontal slicing — DO NOT DO THIS |
| "The test passes unexpectedly" | STOP — record in tdd-red-report.md and investigate before proceeding |
| "I'll mock the internal service" | Mock ONLY external systems — never your own modules |
| "I'll add this public method during refactor" | Interface must not grow — FORBIDDEN |

---

## DAG-Based Workflow (Schema-Driven)

SolidSpec uses a **DAG (Directed Acyclic Graph)** artifact engine. Workflows are defined as dependency graphs in YAML schema files:

```yaml
# schemas/tdd-driven/schema.yaml (excerpt)
artifacts:
  - id: tdd-tests
    generates: ["tests/", "tdd-red-report.md"]
    requires: ["spec", "tasks"]
  - id: implement
    generates: ["tasks.md"]
    requires: ["tdd-tests"]
  - id: tdd-refactor
    generates: ["tdd-refactor-report.md"]
    requires: ["implement"]
```

### Workflow status at a glance

```bash
solidspec status 001 --schema tdd-driven

# Feature: 001-auth  |  Schema: tdd-driven (built-in)
# 10 artifacts, 3 complete, 1 ready
#
# #   Artifact       Status          Depends On
# ─────────────────────────────────────────────────
# 1   spec           ✓ done          —
# 2   clarify        ✓ done          spec
# 3   plan           ✓ done          spec
# 4   tasks          ▶ ready         spec, plan
# 5   tdd-tests      ⏸ blocked       spec, tasks
# 6   implement      ⏸ blocked       tdd-tests
# 7   tdd-refactor   ⏸ blocked       implement
# ...
```

### Custom workflows

Drop a `schema.yaml` in `.solidspec/workflows/<name>/` and use it via `--schema`:

```bash
solidspec status 001 --schema custom
solidspec pipeline 001 --schema custom
```

---

## Change-Based Workflow (Delta Specs)

For **brownfield modifications** — changing existing features without rewriting the entire spec:

```bash
# Propose a change to an existing feature
solidspec change propose "Add social login" --feature-id 001

# List active changes
solidspec change list --feature-id 001

# Archive when done (merges deltas into main spec)
solidspec change archive add-social-login --feature-id 001
```

Delta specs describe only what changed (ADDED/MODIFIED/REMOVED requirements). On archive, SolidSpec merges deltas into `spec.md` automatically.

---

## Project Constitution

Every SolidSpec project gets a `constitution.md` defining architectural principles:

| Gate | What it checks |
|------|---------------|
| **Simplicity** | Max 3 projects, no speculative features |
| **Anti-Abstraction** | Use frameworks directly, no wrapper layers |
| **Integration-First** | Contract tests before implementation, real services over mocks |

The `plan` command evaluates these gates automatically.

---

## Project Context Configuration

```toml
# solidspec.toml
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

---

## All Commands

### Core workflow commands

| Command | Description |
|---------|-------------|
| `solidspec init [name]` | Initialize project with constitution, templates, agent commands |
| `solidspec specify <desc>` | Create feature spec with user stories and quality checklist |
| `solidspec clarify [id]` | Resolve `[NEEDS CLARIFICATION]` markers |
| `solidspec plan [id]` | Generate plan + research + data model + contracts |
| `solidspec tasks [id]` | Generate phased task breakdown with `[P]` parallel markers |
| `solidspec tests [id]` | Generate test scaffolds from Given/When/Then scenarios (`--framework`) |
| `solidspec implement [id]` | Execute tasks with hook support |
| `solidspec analyze [id]` | Validate consistency with severity levels; trace tree and drift in IDSD mode |
| `solidspec review [id]` | Review spec quality with dimension scoring |
| `solidspec checklist [id]` | Generate/append quality checklists |

### TDD commands

| Command | Description |
|---------|-------------|
| `solidspec tdd-tests [id]` | Write real failing tests from acceptance criteria (TDD RED phase). Creates `tdd-red-report.md` with interface design, tracer bullet, cycle table, and quality checklist. |
| `solidspec tdd-refactor [id]` | Scaffold the REFACTOR phase report. Requires `tdd-red-report.md`. Produces `tdd-refactor-report.md` with refactor candidates checklist and change audit table. |

Both commands accept `--dry-run` (print scaffold without writing files) and an optional `feature-id` positional argument.

### IDSD commands

| Command | Description |
|---------|-------------|
| `solidspec intent <title>` | Capture intent using the ICE model (`intent.md`). IDSD phase 0. |
| `solidspec evidence [id]` | Cross-reference evidence criteria against implemented tests. Writes `evidence-report.md`. Add `--update` to rewrite `intent.md` Status automatically. |

### Pipeline and status commands

| Command | Description |
|---------|-------------|
| `solidspec pipeline [id]` | Run multi-agent pipeline (`--new`, `--from`, `--to`, `--only`, `--auto`, `--no-agent`, `--schema`, `--force`, `--dry-run`) |
| `solidspec status [id]` | Show artifact completion status (DAG-based, `--schema`); intent drift in IDSD mode |
| `solidspec ship [id]` | Run parallel fan-out review (4 concurrent AI lanes) → `SHIP` / `HOLD` decision (`--lane`, `--no-agent`, `--fail-on-hold`, `--dry-run`, `--timeout`, `--ignore-timeout`) |

### Project management commands

| Command | Description |
|---------|-------------|
| `solidspec change <cmd>` | Manage changes: `propose "Title"`, `list`, `archive <slug>` (`--feature-id`) |
| `solidspec preset <cmd>` | Manage presets (`add`, `remove`, `list`, `search`, `info`) |
| `solidspec extension <cmd>` | Manage extensions (`add`, `remove`, `enable`, `disable`, `list`) |
| `solidspec upgrade` | Refresh templates + agent commands after update |
| `solidspec completions <shell>` | Generate shell completions (bash, zsh, fish, powershell) |
| `solidspec check` | Verify system prerequisites |

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

[fan_out]
security_agent    = "gemini"
security_threshold = 80
block_on_critical  = true
timeout            = 300
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
  cli/          Command handlers (clap derive) — one module per subcommand
  core/         All business logic: spec parser, planner, task generator, test generator,
                tdd (RED/REFACTOR scaffolding), pipeline, analyzer, constitution,
                intent_parser, evidence, artifact_graph (DAG + trace), fan_out (ship gate)
  agents/       20-agent config table, detection, format translation, registration, CLI invoker
  templates/    Tera rendering + 4-layer resolver (project-local → embedded default)
  presets/      Manifest validation, registry, manager
  extensions/   Manifest, registry, hooks, manager
  config/       TOML configuration handling (RootConfig, ProjectInternalConfig, FanOutConfig)
schemas/
  spec-driven/    Default 9-artifact SDD workflow
  minimal/        4-artifact lightweight workflow
  security-first/ 5-artifact workflow with mandatory OWASP security review
  tdd-driven/     10-artifact AI-TDD workflow (RED-GREEN-REFACTOR)
  intent-driven/  11-artifact IDSD workflow (intent + evidence + drift detection)
  apex-driven/    9-artifact SDD + APEX implementation
  intent-apex/    11-artifact IDSD + APEX (maximum rigor)
docs/
  idsd-workflow-guide.md              Complete IDSD walkthrough with Task Manager example
  Parallel-Fan-out_orchestration-plan.md  Fan-out ship gate design spec
  tdd/                                TDD skill documentation
```

---

## License

[MIT](LICENSE)
