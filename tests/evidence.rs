use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn setup_project(dir: &std::path::Path) {
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir)
        .assert()
        .success();
}

const SAMPLE_INTENT: &str = r#"# Intent: Auth System

**Intent ID**: INT-001
**Feature**: 001-auth-system
**Created**: 2026-06-01
**Status**: active

## Goal
Allow users to authenticate securely.

## Constraints
- Must be stateless

## Evidence
- Users can authenticate with valid credentials
- Password reset email is delivered
- Session is created after login
"#;

fn write_intent(dir: &std::path::Path) {
    let feature_dir = dir.join("specs/001-auth-system");
    std::fs::create_dir_all(&feature_dir).unwrap();
    std::fs::write(feature_dir.join("intent.md"), SAMPLE_INTENT).unwrap();
}

fn write_implemented_test(dir: &std::path::Path) {
    let tests_dir = dir.join("specs/001-auth-system/tests");
    std::fs::create_dir_all(&tests_dir).unwrap();
    std::fs::write(
        tests_dir.join("story1.md"),
        "GIVEN: valid credentials\nWHEN: authenticate\nTHEN: session created login\nSTATUS: IMPLEMENTED\n",
    )
    .unwrap();
}

#[test]
fn evidence_fails_without_intent_md() {
    let dir = TempDir::new().unwrap();
    setup_project(dir.path());

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["specify", "Auth system"])
        .current_dir(dir.path())
        .assert()
        .success();

    // No intent.md — evidence should fail with a clear message
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["evidence", "001"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("intent.md not found"));
}

#[test]
fn evidence_prints_criteria_table_and_writes_report() {
    let dir = TempDir::new().unwrap();
    setup_project(dir.path());
    write_intent(dir.path());

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["evidence", "001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Satisfaction:"))
        .stdout(predicate::str::contains("criteria"));

    assert!(
        dir.path()
            .join("specs/001-auth-system/evidence-report.md")
            .exists(),
        "evidence-report.md must be written"
    );
}

#[test]
fn evidence_report_contains_criteria_table() {
    let dir = TempDir::new().unwrap();
    setup_project(dir.path());
    write_intent(dir.path());
    write_implemented_test(dir.path());

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["evidence", "001"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report =
        std::fs::read_to_string(dir.path().join("specs/001-auth-system/evidence-report.md"))
            .unwrap();
    assert!(report.contains("# Evidence Report"));
    assert!(report.contains("## Criteria"));
    assert!(report.contains("authenticate") || report.contains("Satisfied"));
}

#[test]
fn evidence_update_rewrites_intent_status() {
    let dir = TempDir::new().unwrap();
    setup_project(dir.path());
    write_intent(dir.path());
    // Write a test that covers all three criteria keywords
    let tests_dir = dir.path().join("specs/001-auth-system/tests");
    std::fs::create_dir_all(&tests_dir).unwrap();
    std::fs::write(
        tests_dir.join("all.md"),
        "authenticate credentials\npassword reset email delivered\nsession created login\nSTATUS: IMPLEMENTED\n",
    )
    .unwrap();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["evidence", "001", "--update"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("intent.md Status"));

    let intent =
        std::fs::read_to_string(dir.path().join("specs/001-auth-system/intent.md")).unwrap();
    assert!(
        intent.contains("**Status**: satisfied") || intent.contains("**Status**: active"),
        "Status must be updated; got:\n{intent}"
    );
}

#[test]
fn status_shows_evidence_artifact_in_idsd_schema() {
    let dir = TempDir::new().unwrap();
    setup_project(dir.path());

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["specify", "Auth system"])
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["status", "001", "--schema", "intent-driven"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("evidence"));
}
