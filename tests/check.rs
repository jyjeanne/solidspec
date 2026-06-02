use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn check_detects_non_project_directory() {
    let dir = TempDir::new().unwrap();

    Command::cargo_bin("solidspec")
        .unwrap()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Not a SolidSpec project"));
}

#[test]
fn check_detects_initialized_project() {
    let dir = TempDir::new().unwrap();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("solidspec")
        .unwrap()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("solidspec.toml found"))
        .stdout(predicate::str::contains(".solidspec/ directory found"))
        .stdout(predicate::str::contains("Constitution file present"));
}
