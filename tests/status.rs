use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn status_shows_artifacts_after_pipeline_scaffold() {
    let dir = TempDir::new().unwrap();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["specify", "Status test feature"])
        .current_dir(dir.path())
        .assert()
        .success();
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["plan", "001"])
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["status", "001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Schema: spec-driven"))
        .stdout(predicate::str::contains("artifacts"))
        .stdout(predicate::str::contains("done"))
        .stdout(predicate::str::contains("ready"))
        .stdout(predicate::str::contains("spec"))
        .stdout(predicate::str::contains("plan"))
        .stdout(predicate::str::contains("tasks"));
}

#[test]
fn status_with_minimal_schema() {
    let dir = TempDir::new().unwrap();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["specify", "Minimal test"])
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["status", "001", "--schema", "minimal"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Schema: minimal"))
        .stdout(predicate::str::contains("artifacts"));
}

#[test]
fn status_fails_in_non_solidspec_dir() {
    let dir = TempDir::new().unwrap();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["status", "001"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Not a SolidSpec project"));
}
