use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn setup_project(dir: &std::path::Path, init_cmd: &mut Command) {
    init_cmd
        .arg("init")
        .arg("--here")
        .arg("--no-git")
        .current_dir(dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing SolidSpec project"));
}

#[test]
fn full_pipeline_scaffold_generates_all_artifacts() {
    let dir = TempDir::new().unwrap();

    let mut init = Command::cargo_bin("solidspec").unwrap();
    setup_project(dir.path(), &mut init);

    assert!(dir.path().join("solidspec.toml").exists());
    assert!(dir.path().join(".solidspec/constitution.md").exists());
    assert!(
        dir.path()
            .join(".solidspec/templates")
            .join("spec-template.md")
            .exists()
    );

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["specify", "Todo list with CRUD and local storage"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Feature 001 ready"));

    let specs_dir = dir.path().join("specs");
    assert!(specs_dir.exists());
    let feature_dir = std::fs::read_dir(&specs_dir)
        .unwrap()
        .flatten()
        .find(|e| e.file_type().unwrap().is_dir())
        .expect("No feature directory found")
        .path();
    assert!(feature_dir.join("spec.md").exists());
    let spec = std::fs::read_to_string(feature_dir.join("spec.md")).unwrap();
    assert!(spec.contains("Feature Specification"));
    assert!(spec.contains("Functional Requirements"));

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["plan", "001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Plan complete"));

    assert!(feature_dir.join("plan.md").exists());
    assert!(feature_dir.join("data-model.md").exists());
    assert!(feature_dir.join("research.md").exists());
    assert!(feature_dir.join("quickstart.md").exists());
    assert!(feature_dir.join("contracts/api.md").exists());

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["tasks", "001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created tasks.md"));

    let tasks = std::fs::read_to_string(feature_dir.join("tasks.md")).unwrap();
    assert!(tasks.contains("- [ ] T001"));
    assert!(tasks.contains("Phase 1: Setup"));
    assert!(tasks.contains("Phase 2: Foundational"));
    assert!(tasks.contains("[P]"));

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["tests", "001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated"));

    let test_files: Vec<_> = std::fs::read_dir(feature_dir.join("tests"))
        .unwrap()
        .flatten()
        .collect();
    assert!(!test_files.is_empty(), "No test files generated");
    let test_content = std::fs::read_to_string(test_files[0].path()).unwrap();
    assert!(test_content.contains("GIVEN:"));
    assert!(test_content.contains("WHEN:"));
    assert!(test_content.contains("THEN:"));
    assert!(test_content.contains("STATUS: NOT IMPLEMENTED"));

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["analyze", "001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Traceability Score"));

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["review", "001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Overall Score"));

    assert!(feature_dir.join("review-report.md").exists());
    let review = std::fs::read_to_string(feature_dir.join("review-report.md")).unwrap();
    assert!(review.contains("Dimension Scores"));
    assert!(review.contains("Findings"));
}

#[test]
fn pipeline_status_shows_artifact_table() {
    let dir = TempDir::new().unwrap();

    let mut init = Command::cargo_bin("solidspec").unwrap();
    setup_project(dir.path(), &mut init);

    Command::cargo_bin("solidspec")
        .unwrap()
        .arg("specify")
        .arg("Test feature")
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("solidspec")
        .unwrap()
        .arg("plan")
        .arg("001")
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("solidspec")
        .unwrap()
        .arg("status")
        .arg("001")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Artifact"))
        .stdout(predicate::str::contains("done"))
        .stdout(predicate::str::contains("ready"));
}

#[test]
fn pipeline_dry_run_output_contains_dry_run_marker() {
    let dir = TempDir::new().unwrap();

    let mut init = Command::cargo_bin("solidspec").unwrap();
    setup_project(dir.path(), &mut init);

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["pipeline", "--new", "Dry run feature", "--dry-run"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("[dry-run]"));
}
