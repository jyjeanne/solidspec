use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn init_project(dir: &std::path::Path) {
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir)
        .assert()
        .success();
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["specify", "Test feature for changes"])
        .current_dir(dir)
        .assert()
        .success();
}

#[test]
fn change_propose_creates_directory_and_files() {
    let dir = TempDir::new().unwrap();
    init_project(dir.path());

    Command::cargo_bin("solidspec")
        .unwrap()
        .args([
            "change",
            "propose",
            "Add social login",
            "--feature-id",
            "001",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created change:"));

    let change_dir = dir
        .path()
        .join("specs/001-test-feature-for-changes/changes/add-social-login");
    assert!(change_dir.exists());
    assert!(change_dir.join("proposal.md").exists());
    assert!(change_dir.join("delta-spec.md").exists());
    assert!(change_dir.join(".change.yaml").exists());

    let prop = std::fs::read_to_string(change_dir.join("proposal.md")).unwrap();
    assert!(prop.contains("Add social login"));
    assert!(prop.contains("## Why"));

    let delta = std::fs::read_to_string(change_dir.join("delta-spec.md")).unwrap();
    assert!(delta.contains("## Added Requirements"));
    assert!(delta.contains("## Modified Requirements"));
    assert!(delta.contains("## Removed Requirements"));
}

#[test]
fn change_list_shows_active_changes() {
    let dir = TempDir::new().unwrap();
    init_project(dir.path());

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["change", "propose", "First change", "--feature-id", "001"])
        .current_dir(dir.path())
        .assert()
        .success();
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["change", "propose", "Second change", "--feature-id", "001"])
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["change", "list", "--feature-id", "001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("First change"))
        .stdout(predicate::str::contains("Second change"))
        .stdout(predicate::str::contains("proposed"));
}

#[test]
fn change_archive_merges_deltas_and_moves_to_archive() {
    let dir = TempDir::new().unwrap();
    init_project(dir.path());

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["change", "propose", "Delete FR-001", "--feature-id", "001"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = dir.path().join("specs/001-test-feature-for-changes");
    let change_dir = feature_dir.join("changes/delete-fr-001");

    std::fs::write(
        change_dir.join("delta-spec.md"),
        "## Removed Requirements\n\n- FR-001\n",
    )
    .unwrap();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["change", "archive", "delete-fr-001", "--feature-id", "001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Archived change"));

    assert!(!change_dir.exists());
    assert!(feature_dir.join("changes/archive/delete-fr-001").exists());

    let updated_spec = std::fs::read_to_string(feature_dir.join("spec.md")).unwrap();
    assert!(!updated_spec.contains("FR-001"));
}
