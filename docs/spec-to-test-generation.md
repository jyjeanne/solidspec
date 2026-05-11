# Feature Specification: Spec-to-Test Generation

**Feature ID:** 002
**Feature Name:** Spec-to-Test Generation
**Project Name:** SolidSpec
**Version:** v0.2.0
**Status:** Draft
**Author:** jjeanne
**Date:** 2026-03-16

---

## 1. Overview

`solidspec tests <feature-id>` generates runnable test scaffold files from the acceptance scenarios (Given/When/Then) in a feature's spec.md. The test framework and language are detected from the plan's technical context or project files.

This creates a **spec → test → code** pipeline where tests are the executable contract between the specification and the implementation. The AI agent can't skip the spec — the tests won't pass until every acceptance scenario is implemented.

---

## 2. Problem Statement

In real-world usage (todo-app test project), the AI agent ignored the generated plan and went straight to coding. Acceptance scenarios in spec.md stayed as documentation — never becoming executable. There was no feedback loop between the spec and the code.

Spec-to-Test Generation solves this by turning every Given/When/Then scenario into a real test that fails until the code makes it pass.

---

## 3. User Stories

### User Story 1 — Generate test scaffolds from spec (Priority: P1)

As a developer, I want to generate test scaffold files from my spec's acceptance scenarios so that my AI agent has concrete, failing tests to implement against.

**Acceptance Scenarios:**

1. **Given** a spec.md with 3 user stories and 6 acceptance scenarios, **When** running `solidspec tests 001`, **Then** test files are generated with one test per scenario, each containing Given/When/Then comments and a failing body
2. **Given** no spec.md exists, **When** running `solidspec tests 001`, **Then** an error says "spec.md not found"

### User Story 2 — Auto-detect test framework (Priority: P1)

As a developer, I want SolidSpec to detect my project's test framework automatically so I don't have to configure it.

**Acceptance Scenarios:**

1. **Given** a project with `package.json` containing jest, **When** generating tests, **Then** Jest test syntax is used (`describe`/`test`/`expect`)
2. **Given** a project with `Cargo.toml`, **When** generating tests, **Then** Rust test syntax is used (`#[test]`/`assert!`)
3. **Given** a project with `pytest` in requirements, **When** generating tests, **Then** pytest syntax is used (`def test_`/`assert`)
4. **Given** no recognizable project files, **When** generating tests, **Then** generic pseudocode tests are generated with a warning

### User Story 3 — Organize tests by user story (Priority: P2)

As a developer, I want tests organized by user story so I can run them independently.

**Acceptance Scenarios:**

1. **Given** a spec with 4 user stories, **When** generating tests, **Then** 4 test files are created (one per story) OR 1 file with 4 describe blocks
2. **Given** a user story with 3 scenarios, **When** generating tests, **Then** the test file has 3 test functions/cases

### User Story 4 — Integrate with implement workflow (Priority: P2)

As a developer, I want the implement command to reference generated tests so the AI agent implements code to make tests pass.

**Acceptance Scenarios:**

1. **Given** tests are generated, **When** running `/solidspec-implement`, **Then** the implement instructions tell the agent to make the tests pass
2. **Given** no tests are generated, **When** running `/solidspec-implement`, **Then** implement works as before (no change)

---

## 4. Requirements

### Functional Requirements

- **FR-001**: System MUST parse Given/When/Then acceptance scenarios from spec.md, splitting each scenario into three separate fields (given, when, then)
- **FR-002**: System MUST detect the test framework from project files (package.json, Cargo.toml, pyproject.toml, go.mod, pom.xml, tsconfig.json, etc.)
- **FR-003**: System MUST generate one test per acceptance scenario with descriptive name, Given/When/Then comments as separate lines, and failing body
- **FR-004**: System MUST organize tests by user story (one file per story)
- **FR-005**: System MUST output test files to a `tests/` directory under the feature specs by default
- **FR-006**: System MUST support at least: JavaScript (Jest/Vitest/Mocha), TypeScript, Python (pytest), Rust (cargo test), Go (testing), and generic pseudocode
- **FR-007**: System MUST NOT overwrite existing test files (additive only, warn if file exists)
- **FR-008**: System MUST report number of scenarios parsed and test files generated
- **FR-009**: System MUST support single-line scenarios only (multi-line scenarios are not parsed; a warning is logged if detected)
- **FR-010**: System MUST generate valid, runnable test syntax for each framework (no manual fixes needed for the scaffold to fail correctly)

### Key Entities

- **[AcceptanceScenario]**: A structured triple with `given`, `when`, `then` fields plus a `title` derived from the Then clause, linked to a user story index and priority
- **[TestFramework]**: Detected test framework with name, language, file extension, syntax template identifier
- **[TestScaffold]**: Generated test file content with test functions, scenario comments, and failing body per framework

---

## 5. Scenario Parsing Format

Scenarios MUST follow this single-line format:

```
N. **Given** <given text>, **When** <when text>, **Then** <then text>
```

**Parsing rules:**
- Split on `**When**` and `**Then**` markers (bold markdown)
- Strip leading `**Given**` from the given field
- Strip commas between fields
- Trim whitespace from each field
- The test title is derived from the Then clause (slugified)
- Multi-line scenarios are NOT supported — if a scenario spans multiple lines, it is skipped with a warning

**Slugification rules for test function names:**
- Lowercase
- Spaces and non-alphanumeric characters replaced with underscores (Rust/Python/Go) or preserved as-is (Jest/Vitest/Mocha string)
- Leading/trailing underscores stripped
- Max 80 characters, truncated with no trailing underscore

---

## 6. CLI Command

### `solidspec tests <feature-id>`

**Flags:**

| Flag | Description |
|------|-------------|
| `--framework <name>` | Override auto-detected framework (jest, vitest, mocha, pytest, cargo, go, generic) |
| `--output <dir>` | Override test output directory (default: `specs/<feature>/tests/`) |
| `--dry-run` | Preview what would be generated without writing files |

**Example:**

```bash
solidspec tests 001
# Detected: JavaScript (Jest)
# Parsed: 6 acceptance scenarios from 4 user stories
# Generated:
#   specs/001-todo-crud/tests/us1-add-task.test.js
#   specs/001-todo-crud/tests/us2-view-tasks.test.js
#   specs/001-todo-crud/tests/us3-edit-task.test.js
#   specs/001-todo-crud/tests/us4-delete-task.test.js
```

---

## 7. Test Framework Detection

Detection order (first match wins):

| File | Framework | Language | File extension |
|------|-----------|----------|----------------|
| `package.json` with `jest` + `tsconfig.json` | Jest | TypeScript | `.test.ts` |
| `package.json` with `vitest` + `tsconfig.json` | Vitest | TypeScript | `.test.ts` |
| `package.json` with `jest` in deps/devDeps | Jest | JavaScript | `.test.js` |
| `package.json` with `vitest` in deps/devDeps | Vitest | JavaScript | `.test.js` |
| `package.json` with `mocha` in deps/devDeps | Mocha | JavaScript | `.test.js` |
| `package.json` (any) | Jest (default for JS) | JavaScript | `.test.js` |
| `tsconfig.json` (no package.json) | Jest | TypeScript | `.test.ts` |
| `Cargo.toml` | cargo test | Rust | `.rs` |
| `pyproject.toml` or `requirements.txt` with `pytest` | pytest | Python | `_test.py` |
| `setup.py` or `requirements.txt` (any Python) | pytest (default) | Python | `_test.py` |
| `go.mod` | go test | Go | `_test.go` |
| `pom.xml` or `build.gradle` | JUnit | Java | `Test.java` |
| `*.csproj` or `*.sln` | xUnit | C# | `Tests.cs` |
| (none matched) | Generic pseudocode | — | `.test.txt` |

**Notes:**
- Vitest and Mocha use the same template as Jest (syntax is compatible)
- TypeScript uses same template as JavaScript but with `.ts` extension
- `--framework` flag overrides all detection logic

---

## 8. Generated Test Format

### JavaScript (Jest)

```javascript
// Generated by solidspec tests — DO NOT EDIT test structure (fill in bodies)
// Feature: 001-todo-crud
// User Story 1: Add a new task (P1)

describe('US1: Add a new task', () => {
  test('task appears in list with pending status', () => {
    // Given: the app is open
    // When: user types a task title and clicks "Add"
    // Then: the task appears in the list with status "pending"
    throw new Error('TODO: implement this test');
  });

  test('empty task shows error message', () => {
    // Given: user tries to add empty task
    // When: clicking "Add"
    // Then: an error message is shown
    throw new Error('TODO: implement this test');
  });
});
```

### Python (pytest)

```python
# Generated by solidspec tests — DO NOT EDIT test structure (fill in bodies)
# Feature: 001-todo-crud
# User Story 1: Add a new task (P1)


class TestUS1AddANewTask:
    def test_task_appears_in_list_with_pending_status(self):
        """
        Given: the app is open
        When: user types a task title and clicks "Add"
        Then: the task appears in the list with status "pending"
        """
        raise NotImplementedError("TODO: implement this test")

    def test_empty_task_shows_error_message(self):
        """
        Given: user tries to add empty task
        When: clicking "Add"
        Then: an error message is shown
        """
        raise NotImplementedError("TODO: implement this test")
```

### Rust (cargo test)

```rust
// Generated by solidspec tests — DO NOT EDIT test structure (fill in bodies)
// Feature: 001-todo-crud
// User Story 1: Add a new task (P1)

#[cfg(test)]
mod us1_add_a_new_task {
    #[test]
    fn task_appears_in_list_with_pending_status() {
        // Given: the app is open
        // When: user types a task title and clicks "Add"
        // Then: the task appears in the list with status "pending"
        todo!("implement this test");
    }

    #[test]
    fn empty_task_shows_error_message() {
        // Given: user tries to add empty task
        // When: clicking "Add"
        // Then: an error message is shown
        todo!("implement this test");
    }
}
```

### Go (testing)

```go
// Generated by solidspec tests — DO NOT EDIT test structure (fill in bodies)
// Feature: 001-todo-crud
// User Story 1: Add a new task (P1)

package us1_add_a_new_task

import "testing"

func TestTaskAppearsInListWithPendingStatus(t *testing.T) {
	// Given: the app is open
	// When: user types a task title and clicks "Add"
	// Then: the task appears in the list with status "pending"
	t.Fatal("TODO: implement this test")
}

func TestEmptyTaskShowsErrorMessage(t *testing.T) {
	// Given: user tries to add empty task
	// When: clicking "Add"
	// Then: an error message is shown
	t.Fatal("TODO: implement this test")
}
```

### Generic (pseudocode)

```
# Generated by solidspec tests
# Feature: 001-todo-crud
# User Story 1: Add a new task (P1)

TEST "task appears in list with pending status":
  GIVEN: the app is open
  WHEN: user types a task title and clicks "Add"
  THEN: the task appears in the list with status "pending"
  STATUS: NOT IMPLEMENTED

TEST "empty task shows error message":
  GIVEN: user tries to add empty task
  WHEN: clicking "Add"
  THEN: an error message is shown
  STATUS: NOT IMPLEMENTED
```

---

## 9. Implementation Plan

### Phase 1: Scenario Parser

- Extract Given/When/Then triples from spec.md acceptance scenarios
- Link each scenario to its user story (title, priority, story index)
- Handle multi-line scenarios and numbered lists
- Return `Vec<AcceptanceScenario>` with story context

### Phase 2: Framework Detection

- Scan project root for framework indicator files
- Return `TestFramework` struct with: name, language, file extension, syntax template
- Support `--framework` override flag

### Phase 3: Test Scaffold Generation

- Per-framework template rendering (one template per supported framework)
- Generate test function name from scenario title (slugified)
- Inject Given/When/Then comments
- Inject failing body (`throw`/`raise`/`todo!`/`NOT IMPLEMENTED`)
- Organize by user story (one file per story)

### Phase 4: CLI Integration

- Add `tests` subcommand to CLI
- Wire into template resolution (test templates can be overridden via presets)
- Update implement command body to reference generated tests when they exist
- Fire `after_tests` hook if extensions are installed

---

## 10. Development Tasks

| Task | Description |
|------|-------------|
| T001 | Extend spec parser: extract Given/When/Then triples with story context |
| T002 | **Test:** Parse spec with 3 stories × 2 scenarios = 6 triples extracted with correct story link |
| T003 | Implement framework detection: scan project files, return TestFramework |
| T004 | **Test:** package.json with jest → Jest detected; Cargo.toml → cargo test; no files → generic |
| T005 | Create test templates: Jest, pytest, cargo, Go, generic |
| T006 | **Test:** Each template renders valid syntax with Given/When/Then comments and failing body |
| T007 | Implement scaffold generator: render templates per story, write files |
| T008 | **Test:** 4 stories → 4 test files created; existing file → not overwritten + warning |
| T009 | Add `solidspec tests` CLI subcommand with `--framework` and `--output` flags |
| T010 | **Test:** CLI runs end-to-end, generates correct files, reports counts |
| T011 | Update implement command to reference tests when they exist |
| T012 | **Test:** Implement body mentions tests when test files found; unchanged when no tests |
| T013 | Add `after_tests` hook trigger |
| T014 | **Test:** Hook fires after test generation |

---

## 11. Acceptance Criteria

Feature is complete when:

- `solidspec tests 001` generates test scaffolds from acceptance scenarios
- Framework auto-detection works for JavaScript, Python, Rust, Go, and generic
- Each test has descriptive name, Given/When/Then comments, and failing body
- Tests are organized by user story (one file per story)
- Existing test files are not overwritten
- `--framework` flag overrides detection
- `/solidspec-implement` references generated tests when they exist
- At least 14 unit tests covering parser, detection, templates, generator, and CLI

---

## 12. Edge Cases

- **Spec with no acceptance scenarios** — report "0 scenarios found" and generate no files (not an error)
- **Story with empty scenario list** — skip the story, generate no test file for it
- **Scenario missing When or Then** — skip the scenario with a warning, include comment in test file
- **Duplicate scenario titles within a story** — append `_2`, `_3` suffix to test function names
- **Very long scenario text (>200 chars)** — truncate test function name at 80 chars
- **Special characters in scenario text** — strip for function names, preserve in comments
- **Edge Cases section in spec** — NOT parsed (only acceptance scenarios with Given/When/Then format)

---

## 13. Non-Goals

- NOT generating test implementations (only scaffolds with TODO bodies)
- NOT running the tests (that's the AI agent's or developer's job)
- NOT supporting multi-line scenarios (single-line only for v0.2.0)
- NOT parsing edge cases section (only Given/When/Then acceptance scenarios)
- NOT generating setup/teardown code (only test function scaffolds)

---

## 14. Success Criteria

- Users can go from spec to failing tests in one command
- AI agents can fill in test bodies and then implement code to make tests pass
- Test scaffold syntax is valid for each supported framework (no manual fixes needed)
- Feature adds <2 seconds to the SDD pipeline
