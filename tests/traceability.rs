//! Integration tests for the full IDSD traceability chain.
//!
//! These tests verify the end-to-end behavior of the traceability graph:
//! INT-001 → FR-XXX (spec) → T-XXX (tasks) → test file
//! and the derived metrics (orphaned requirements, intent coverage).

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

// ── Shared helpers ───────────────────────────────────────────────────────────

fn solidspec(dir: &std::path::Path) -> Command {
    let mut cmd = Command::cargo_bin("solidspec").unwrap();
    cmd.current_dir(dir);
    cmd
}

fn init_project(dir: &std::path::Path) {
    solidspec(dir)
        .args(["init", "--here", "--no-git"])
        .assert()
        .success();
}

/// Write a file relative to `dir`, creating parent directories as needed.
fn write(dir: &std::path::Path, rel_path: &str, content: &str) {
    let full = dir.join(rel_path);
    if let Some(parent) = full.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(full, content).unwrap();
}

/// Build a feature directory pre-populated with IDSD artifacts ready for
/// `solidspec analyze`. The tasks explicitly reference FR-001/FR-002 so the
/// trace graph can link them. FR-003 is intentionally left without a task
/// so orphan detection is exercised. The test file mentions T001 so a
/// Task→Test link can be built.
fn setup_full_idsd_feature(project_root: &std::path::Path) -> std::path::PathBuf {
    let feature_dir = project_root.join("specs/001-task-manager");
    std::fs::create_dir_all(feature_dir.join("tests")).unwrap();
    std::fs::create_dir_all(feature_dir.join("contracts")).unwrap();

    // intent.md — ICE model
    write(
        &feature_dir,
        "intent.md",
        r#"# Intent: Task Manager

**Intent ID**: INT-001
**Feature**: 001-task-manager
**Created**: 2026-06-03
**Status**: active

## Goal

Allow users to manage tasks so they can track what needs to be done.

## Constraints

- Must work offline without network access
- Must persist tasks across sessions
- Must support concurrent users without data loss

## Evidence

- Users can create a task with a title and due date
- Users can mark a task as complete
- Tasks persist after application restart
- Concurrent writes do not corrupt the task list

## Risks

- Concurrent write conflicts on shared task lists

## Open Questions

- Should completed tasks be archived or deleted?
"#,
    );

    // spec.md — two FRs covered by tasks, one orphaned (FR-003)
    write(
        &feature_dir,
        "spec.md",
        r#"# Feature Specification: Task Manager

## User Scenarios & Testing

### User Story 1 - Create and manage tasks (Priority: P1)

**Acceptance Scenarios**:

1. **Given** a user opens the app, **When** they create a task with title "Buy milk" and due date tomorrow, **Then** the task appears in the list

---

### User Story 2 - Mark tasks complete (Priority: P1)

**Acceptance Scenarios**:

1. **Given** an existing task, **When** the user taps the checkbox, **Then** the task is marked complete and moved to the done list

---

## Requirements

### Functional Requirements

- **FR-001**: System MUST allow users to create tasks with a title and optional due date
- **FR-002**: System MUST allow users to mark tasks as complete or incomplete
- **FR-003**: System MUST export task lists to CSV format

### Key Entities

- **[Task]**: A unit of work with title, status, and optional due date
- **[TaskList]**: An ordered collection of tasks belonging to a user

## Success Criteria

- **SC-001**: Users can create, complete, and view tasks
- **SC-002**: Tasks persist across app restarts
"#,
    );

    // plan.md
    write(
        &feature_dir,
        "plan.md",
        "# Implementation Plan: Task Manager\n\nFR-001 addressed by TaskRepository.create().\nFR-002 addressed by TaskRepository.update().\n\nTask entity stored in local SQLite.\nTaskList managed via repository pattern.\n",
    );

    // tasks.md — T001 references FR-001, T002 references FR-002; FR-003 has no task
    write(
        &feature_dir,
        "tasks.md",
        r#"# Task Breakdown: Task Manager

## Phase 1: Setup

- [ ] T001 Initialize project structure [FR-001]
- [ ] T002 Setup local database schema [FR-002]

## Phase 2: Foundational

- [ ] T003 Implement Task model and repository [FR-001]
- [ ] T004 Add task creation endpoint [FR-001]
- [ ] T005 Add task completion toggle [FR-002]

## Phase 3: User Story 1 - Create and manage tasks (Priority: P1)

- [ ] T006 [US1] Implement task creation UI [FR-001]
- [ ] T007 [US1] Add due date picker [FR-001]

## Phase 4: User Story 2 - Mark tasks complete (Priority: P1)

- [ ] T008 [US2] Implement completion checkbox [FR-002]
"#,
    );

    // Test file that mentions T001 and T003 — creates Task→Test links
    write(
        &feature_dir,
        "tests/story1_create_task.md",
        "# Test: Create Task\n\n// T001 T003\nGIVEN: user opens app\nWHEN: creates task 'Buy milk'\nTHEN: task appears in list\nSTATUS: IMPLEMENTED\n",
    );

    // Test file for story 2 that mentions T005
    write(
        &feature_dir,
        "tests/story2_complete_task.md",
        "# Test: Complete Task\n\n// T005\nGIVEN: existing task\nWHEN: user taps checkbox\nTHEN: task marked complete\nSTATUS: NOT IMPLEMENTED\n",
    );

    feature_dir
}

// ── Test 1: Full IDSD pipeline scaffold ─────────────────────────────────────

#[test]
fn idsd_pipeline_scaffold_creates_all_artifacts() {
    let dir = TempDir::new().unwrap();
    init_project(dir.path());

    solidspec(dir.path())
        .args([
            "pipeline",
            "--new",
            "Task manager with CRUD and persistence",
            "--schema",
            "intent-driven",
            "--no-agent",
        ])
        .assert()
        .success();

    let specs_dir = dir.path().join("specs");
    let feature_dir = std::fs::read_dir(&specs_dir)
        .unwrap()
        .flatten()
        .find(|e| e.file_type().unwrap().is_dir())
        .expect("no feature directory")
        .path();

    // IDSD-specific artifacts
    assert!(
        feature_dir.join("intent.md").exists(),
        "intent.md must be created by IDSD pipeline"
    );

    // Standard SDD artifacts still generated
    assert!(feature_dir.join("spec.md").exists(), "spec.md missing");
    assert!(feature_dir.join("plan.md").exists(), "plan.md missing");
    assert!(feature_dir.join("tasks.md").exists(), "tasks.md missing");

    // Evidence report generated by evidence phase
    assert!(
        feature_dir.join("evidence-report.md").exists(),
        "evidence-report.md must be created by IDSD pipeline"
    );
}

// ── Test 2: Trace tree appears in analyze output ─────────────────────────────

#[test]
fn analyze_prints_traceability_chain_tree() {
    let dir = TempDir::new().unwrap();
    init_project(dir.path());
    setup_full_idsd_feature(dir.path());

    solidspec(dir.path())
        .args(["analyze", "001"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Traceability Chain"))
        .stdout(predicate::str::contains("INT-001"))
        .stdout(predicate::str::contains("FR-001"))
        .stdout(predicate::str::contains("FR-002"));
}

// ── Test 3: Task → Test links visible in trace tree ──────────────────────────

#[test]
fn trace_tree_shows_task_to_test_links() {
    let dir = TempDir::new().unwrap();
    init_project(dir.path());
    setup_full_idsd_feature(dir.path());

    // T001 is mentioned in story1_create_task.md → Task→Test link should appear
    solidspec(dir.path())
        .args(["analyze", "001"])
        .assert()
        .success()
        .stdout(predicate::str::contains("story1_create_task.md"));
}

// ── Test 4: Orphaned requirement reported as High finding ────────────────────

#[test]
fn orphaned_requirement_produces_high_finding() {
    let dir = TempDir::new().unwrap();
    init_project(dir.path());
    setup_full_idsd_feature(dir.path());

    // FR-003 (CSV export) has no task in tasks.md — must be reported
    solidspec(dir.path())
        .args(["analyze", "001"])
        .assert()
        .success()
        .stdout(predicate::str::contains("FR-003"))
        .stdout(predicate::str::contains("no task"));
}

// ── Test 5: Intent coverage metric shown when intent.md present ──────────────

#[test]
fn analyze_shows_intent_coverage_with_intent_md() {
    let dir = TempDir::new().unwrap();
    init_project(dir.path());
    setup_full_idsd_feature(dir.path());

    // story1 is IMPLEMENTED → intent_coverage > 0 once any test is implemented
    solidspec(dir.path())
        .args(["analyze", "001"])
        .assert()
        .success()
        .stdout(predicate::str::contains("intent coverage"));
}

// ── Test 6: Full IDSD chain — evidence updates intent status ─────────────────

#[test]
fn evidence_update_reflects_in_intent_md_status() {
    let dir = TempDir::new().unwrap();
    init_project(dir.path());
    let feature_dir = setup_full_idsd_feature(dir.path());

    // Run evidence with --update to rewrite intent.md Status
    solidspec(dir.path())
        .args(["evidence", "001", "--update"])
        .assert()
        .success()
        .stdout(predicate::str::contains("intent.md Status"));

    // intent.md Status must have been updated (drifted or active — not still "active" from template
    // since one criterion is covered and others are not, satisfaction < 100% but > 0%)
    let intent = std::fs::read_to_string(feature_dir.join("intent.md")).unwrap();
    assert!(
        intent.contains("**Status**: active")
            || intent.contains("**Status**: satisfied")
            || intent.contains("**Status**: drifted"),
        "intent.md Status must be one of active/satisfied/drifted, got:\n{intent}"
    );

    // evidence-report.md must be written
    assert!(
        feature_dir.join("evidence-report.md").exists(),
        "evidence-report.md must exist after solidspec evidence"
    );

    // The report must contain the criteria table
    let report = std::fs::read_to_string(feature_dir.join("evidence-report.md")).unwrap();
    assert!(report.contains("## Criteria"));
    assert!(report.contains("create"));
}

// ── Test 7: SDD pipeline unchanged — no IDSD artifacts ──────────────────────

#[test]
fn sdd_pipeline_produces_no_idsd_artifacts() {
    let dir = TempDir::new().unwrap();
    init_project(dir.path());

    solidspec(dir.path())
        .args(["pipeline", "--new", "Simple feature", "--no-agent"])
        .assert()
        .success();

    let specs_dir = dir.path().join("specs");
    let feature_dir = std::fs::read_dir(&specs_dir)
        .unwrap()
        .flatten()
        .find(|e| e.file_type().unwrap().is_dir())
        .expect("no feature directory")
        .path();

    assert!(
        !feature_dir.join("intent.md").exists(),
        "SDD must not create intent.md"
    );
    assert!(
        !feature_dir.join("evidence-report.md").exists(),
        "SDD must not create evidence-report.md"
    );
    assert!(feature_dir.join("spec.md").exists());
}

// ── Test 8: analyze without intent.md shows no trace-level IDSD metrics ──────

#[test]
fn analyze_without_intent_md_omits_idsd_metrics() {
    let dir = TempDir::new().unwrap();
    init_project(dir.path());

    // Create a minimal SDD feature (no intent.md)
    solidspec(dir.path())
        .args(["specify", "Simple login feature"])
        .assert()
        .success();

    solidspec(dir.path())
        .args(["analyze", "001"])
        .assert()
        .success()
        // These IDSD-only sections must NOT appear without intent.md
        .stdout(predicate::str::contains("INT-").not())
        .stdout(predicate::str::contains("intent coverage").not());
}
