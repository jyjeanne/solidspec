# Killer Feature Ideas

Features that would differentiate SolidSpec from Spec Kit and other SDD tools.

---

## Priority: User Value Ranking

Based on real-world testing (todo-app project), the biggest pain points observed were:
1. The AI agent ignored the plan and went straight to coding — specs stayed as documentation
2. No way to verify the code actually matches the spec
3. Writing specs is the hardest part — blank template intimidation

Ranked by **what users care about most**:

| Rank | Feature | Why users want it |
|------|---------|-------------------|
| **1** | **Spec-to-Test Generation** | Turns specs from documentation into executable contracts. AI agent can't skip them — tests must pass. Gives immediate tangible output (real code, not another markdown file). |
| **2** | **Interactive TUI Builder** | Eliminates blank-template intimidation. Guided questions + live quality score turns spec writing from a chore into a conversation. Lowers the barrier to entry for every developer. |
| **3** | **Spec Import from Issues** | Meets users where they already are. Requirements live in GitHub Issues / Jira — one command to pre-fill a spec removes the cold-start problem entirely. |
| **4** | **Live Traceability Matrix** | Answers "is my spec actually built?" by scanning source code. Appeals to tech leads and teams who need audit trails. |
| **5** | **Multi-Agent Pipeline** | Power-user feature. Lets teams use Claude for specs, Copilot for code, Gemini for review. First multi-agent SDD tool. |
| **6** | **Spec Drift Detection** | Long-term maintenance value. Detects when code diverges from spec over time. Most useful for projects with ongoing development. |
| **7** | **AI-Powered Spec Review** | Nice to have. LLM reviews specs for quality. Requires API integration and prompt engineering — high effort for incremental value. |

**Recommended starting point:** Spec-to-Test Generation — it's the feature that would make someone choose SolidSpec over Spec Kit in 30 seconds.

---

## Roadmap

| Version | Feature | Status |
|---------|---------|--------|
| v0.2.0 | **Spec-to-Test Generation** | ✅ Implemented |
| v0.3.0 | **Multi-Agent Pipeline** + Anti-Rationalization + Personas + DAG Graph + Schema Workflows + Delta Specs + OpenCode Skills | ✅ Implemented |
| v0.4.0 | Parallel Fan-Out + Doubt-Driven Development + Spec Import from Issues | 🚧 Planned |
| v1.0.0 | TUI Builder + Traceability Matrix + Drift Detection | 📅 Future |

Full roadmap with status: [ROADMAP.md](../../ROADMAP.md)

---

## Features by Implementation Difficulty

Ordered from easiest to hardest.

---

## 1. Spec Import from Issues

**Difficulty:** Low | **Impact:** Medium | **Uniqueness:** Convenience

Import requirements from GitHub Issues or Jira tickets and generate a pre-filled spec.md.

```bash
solidspec import --github 42
solidspec import --jira PROJ-123
```

**What it does:**
- Pulls issue title, description, labels, comments
- Generates spec.md with user stories derived from issue body
- Pre-fills acceptance criteria from structured comments
- Adds `[NEEDS CLARIFICATION]` markers for missing details

**Example output:**
```
Imported: "Add dark mode support" (#42)
Created: specs/003-add-dark-mode/spec.md
Pre-filled: 2 user stories from issue description
Pre-filled: 3 requirements from acceptance criteria in comments
Markers: 1 [NEEDS CLARIFICATION] (priority not specified)
```

**Implementation:** Use `ureq` for GitHub API, parse issue body with regex, map labels to priorities.

---

## 2. Spec-to-Test Generation

**Difficulty:** Medium | **Impact:** Very High | **Uniqueness:** No other tool does this

Automatically generate runnable test scaffolds from Given/When/Then acceptance scenarios in spec.md.

```bash
solidspec tests 001
```

**What it does:**
- Parses acceptance scenarios from spec.md
- Detects tech stack from plan.md (JavaScript/Jest, Python/pytest, Rust/cargo, etc.)
- Generates test file scaffolds with one test per scenario
- Each test has a descriptive name, Given/When/Then comments, and a TODO body

**Example:**
```
# From spec.md:
# **Given** the app is open, **When** user types a task and clicks "Add",
# **Then** the task appears in the list with status "pending"

# Generated: tests/us1-add-task.test.js
describe("US1: Add a new task", () => {
  test("task appears in list with pending status", () => {
    // Given: the app is open
    // When: user types a task and clicks "Add"
    // Then: the task appears in the list with status "pending"
    throw new Error("TODO: implement");
  });

  test("empty task shows error message", () => {
    // Given: user tries to add empty task
    // When: clicking "Add"
    // Then: an error message is shown
    throw new Error("TODO: implement");
  });
});
```

**Why it's a killer:** Creates a `spec → test → code` pipeline. Tests become the executable contract between the spec and the implementation. The AI agent fills in the test bodies, then writes code to make them pass — true test-driven SDD.

**Implementation:** Extend spec parser to extract Given/When/Then tuples; add test template per language/framework; detect framework from plan.md or `package.json`/`Cargo.toml`.

---

## 3. Multi-Agent Pipeline

**Difficulty:** Medium | **Impact:** High | **Uniqueness:** First multi-agent SDD tool

Orchestrate different AI agents for different SDD phases in a single workflow.

**Configuration in `solidspec.toml`:**
```toml
[pipeline]
specify = "claude"       # Claude writes specs (strong at structured text)
plan = "claude"          # Claude for architecture
tasks = "claude"         # Claude for task breakdown
implement = "copilot"    # Copilot for code generation (IDE-integrated)
analyze = "gemini"       # Gemini for cross-checking (different perspective)
```

```bash
solidspec pipeline 001    # Runs full workflow, switching agents per phase
```

**What it does:**
- Reads pipeline config from `solidspec.toml`
- Executes each phase with the configured agent
- Passes output from one phase as input to the next
- Logs which agent handled which phase
- Supports `--dry-run` to preview the pipeline

**Why it matters:** Different agents have different strengths. Claude excels at structured specification, Copilot integrates with the IDE for implementation, Gemini can cross-check from a different perspective. No other tool orchestrates multiple agents.

**Implementation:** Extend init-options with pipeline config; modify each CLI command to check pipeline config and switch agent context; add `pipeline` subcommand.

---

## 4. Interactive TUI Spec Builder

**Difficulty:** Medium | **Impact:** Medium | **Uniqueness:** Rust TUI advantage

Terminal UI for guided spec creation with real-time quality validation.

```bash
solidspec specify --interactive
```

**What it does:**
- Launches a `ratatui`-based TUI
- Guides user through spec creation step by step:
  1. Feature description
  2. User stories with priority selection
  3. Acceptance scenarios per story (Given/When/Then builder)
  4. Functional requirements
  5. Entity definitions
  6. Edge cases
  7. Success criteria
- Real-time quality score updates as you type
- Detects implementation details and warns immediately
- Counts clarification markers live
- Preview rendered spec.md at any point
- Save and continue later

**TUI layout:**
```
┌─ SolidSpec — New Feature Specification ──────────────────┐
│                                                          │
│  Feature: TODO list with CRUD operations                 │
│                                                          │
│  User Stories:                                           │
│  ● US1 [P1] Add a new task                        [Edit] │
│  ● US2 [P1] View all tasks                        [Edit] │
│  ● US3 [P2] Edit a task                           [Edit] │
│  ○ Add another story...                                  │
│                                                          │
│  Quality: ████████░░ 80%                                 │
│  ⚠ 1 story missing acceptance scenario                   │
│                                                          │
│  [Tab: Next] [Shift+Tab: Back] [Ctrl+S: Save] [Ctrl+P: Preview] │
└──────────────────────────────────────────────────────────┘
```

**Why Rust:** `ratatui` is the best TUI framework in any language — fast, responsive, cross-platform. Python's `textual` is noticeably slower.

**Implementation:** Add `ratatui` + `crossterm` dependencies; build state machine for spec creation flow; integrate with existing spec parser for validation; render to spec.md on save.

---

## 5. Live Traceability Matrix

**Difficulty:** High | **Impact:** Very High | **Uniqueness:** Unique in SDD space

Scan actual source code to build a requirement-to-code traceability map.

```bash
solidspec trace 001
```

**Output:**
```
Traceability Matrix: 001-todo-list-crud

  Requirement          Plan     Task    Code                    Status
  ─────────────────────────────────────────────────────────────────────
  FR-001 (create)      ✓        T007    app.js:addTask()        ✓
  FR-002 (display)     ✓        T009    app.js:renderTasks()    ✓
  FR-003 (persist)     ✗        T003    app.js:saveTasks()      ✓
  FR-004 (complete)    ✗        T011    app.js:toggleTask()     ✓
  FR-005 (edit)        ✗        T011    app.js:editTask()       ✓
  FR-006 (delete)      ✗        T013    app.js:deleteTask()     ✓
  FR-007 (count)       ✗        —       app.js:renderTasks()    ✓

  Coverage: 7/7 requirements → code (100%)
  Tracing: 2/7 requirements → plan (29%)
  Gaps: FR-007 has no dedicated task but IS implemented
```

**What it does:**
- Parses requirements from spec.md (FR-001, FR-002...)
- Checks plan.md for requirement references
- Checks tasks.md for requirement-linked tasks
- Scans source code files for:
  - Function/method names matching requirement keywords
  - Comments containing FR-### references
  - Variable names matching entity names from spec
- Uses `tree-sitter` for language-aware AST parsing (supports JS, TS, Python, Rust, Go, Java, etc.)
- Outputs matrix as Markdown table or JSON

**Why it's a killer:** Closes the loop between specification and implementation. No SDD tool does code-level traceability today. Answers the question: "Is my spec actually built?"

**Implementation:** Add `tree-sitter` crate + language grammars; build keyword extraction from spec; walk source files and match against requirement keywords; output matrix. High effort due to multi-language AST support.

---

## 6. Spec Drift Detection

**Difficulty:** High | **Impact:** High | **Uniqueness:** Novel concept

Detect when implementation has diverged from the specification.

```bash
solidspec drift 001
```

**Output:**
```
Drift Analysis: 001-todo-list-crud

  US1 "Add a new task":
    ✓ Scenario 1: task appears with pending status
      Evidence: addTask() sets status='pending' (app.js:145)
    ✓ Scenario 2: empty task shows error
      Evidence: form validation + addError element (app.js:205)

  US4 "Delete a task":
    ✓ Scenario 1: delete with confirmation
      Evidence: confirm() before filter (app.js:170)
    ⚠ Scenario 2: cancel preserves task
      Evidence: confirm() false path exists but no test

  Edge Cases:
    ✗ "local storage full" — no try/catch around setItem
    ✓ "long task titles" — word-break CSS in style.css:157
    ✗ "two browser tabs" — no storage event listener

  Drift Score: 85% aligned (2 gaps, 1 warning)
  Recommendations:
    - Add try/catch around localStorage.setItem for quota errors
    - Add window.addEventListener('storage', ...) for cross-tab sync
```

**What it does:**
- Parses acceptance scenarios and edge cases from spec.md
- Extracts keywords and behavioral expectations from each scenario
- Scans source code for evidence of each behavior
- Reports which scenarios have code evidence and which don't
- Suggests concrete fixes for gaps
- Computes a drift score (% of scenarios with code evidence)

**Why it matters:** Specs decay over time. Features get added without updating specs, or specs describe behaviors that were never implemented. Drift detection makes specs living documents with continuous validation.

**Implementation:** Combines spec parser (Given/When/Then extraction) with code scanner (tree-sitter AST + keyword matching). Needs heuristic mapping from scenario language to code patterns. Most complex feature but highest long-term value.

---

## 7. AI-Powered Spec Review

**Difficulty:** Very High | **Impact:** High | **Uniqueness:** AI-in-the-loop SDD

Use an LLM to review and improve specifications before implementation.

```bash
solidspec review 001 --ai claude
```

**What it does:**
- Sends spec.md to the configured AI agent for structured review
- AI evaluates against the constitution principles
- Returns scored feedback:
  - Completeness (are all sections filled?)
  - Clarity (any ambiguous requirements?)
  - Testability (can each requirement be verified?)
  - Scope (any speculative features?)
  - Consistency (do stories align with requirements?)
- Optionally auto-fixes issues and creates a new spec version

**Output:**
```
Spec Review: 001-todo-list-crud

  Completeness:  ████████░░  80%  (missing: edge case for concurrent access)
  Clarity:       █████████░  90%  (FR-003 could specify storage mechanism)
  Testability:   ██████████  100% (all scenarios have Given/When/Then)
  Scope:         ██████████  100% (no speculative features)
  Consistency:   █████████░  90%  (US2 overlaps with FR-002 wording)

  Suggestions:
  1. Add edge case: "What if localStorage is disabled?"
  2. FR-003: Change "persist tasks" to "persist tasks across page refreshes"
  3. US2/FR-002: Align wording for consistency

  Apply suggestions? [y/N]
```

**Implementation:** Requires HTTP client to call AI API (Claude/OpenAI), structured prompt engineering for review criteria, response parsing, and optional spec rewriting. Very high effort due to API integration and prompt engineering complexity.

---

## Summary

| # | Feature | Difficulty | Impact | Uniqueness |
|---|---------|-----------|--------|------------|
| 1 | Spec Import from Issues | Low | Medium | Convenience |
| 2 | Spec-to-Test Generation | Medium | Very High | No tool does this |
| 3 | Multi-Agent Pipeline | Medium | High | First multi-agent SDD |
| 4 | Interactive TUI Builder | Medium | Medium | Rust TUI advantage |
| 5 | Live Traceability Matrix | High | Very High | Unique in SDD |
| 6 | Spec Drift Detection | High | High | Novel concept |
| 7 | AI-Powered Spec Review | Very High | High | AI-in-the-loop |

**Recommended implementation order:** 1 → 2 → 3 → 4 → 5 → 6 → 7

Features 2 (Spec-to-Test) and 5 (Traceability Matrix) together create the killer combination: a closed `spec → test → code → trace` loop that no other tool offers.
