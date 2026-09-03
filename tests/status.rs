use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn status_shows_artifacts_after_pipeline_scaffold() {
    let dir = TempDir::new().unwrap();

    // Explicit --schema: init's own default is "minimal" when unset (see
    // src/cli/init.rs), but this test exercises the full spec-driven
    // artifact set (spec/plan/tasks).
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git", "--schema", "spec-driven"])
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

#[test]
fn status_warns_instead_of_panicking_on_cyclic_schema() {
    // A project-local schema with a dependency cycle (spec <-> plan) must not
    // crash `solidspec status` — it should warn and still print the table.
    let dir = TempDir::new().unwrap();

    // Explicit --schema: this test overrides the spec-driven schema
    // specifically to make it cyclic, so the project's default must
    // actually be spec-driven (init's own default is "minimal" when unset).
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git", "--schema", "spec-driven"])
        .current_dir(dir.path())
        .assert()
        .success();
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["specify", "Cyclic schema test"])
        .current_dir(dir.path())
        .assert()
        .success();

    let workflows_dir = dir.path().join(".solidspec/workflows/spec-driven");
    std::fs::create_dir_all(&workflows_dir).unwrap();
    std::fs::write(
        workflows_dir.join("schema.yaml"),
        r#"
name: spec-driven
version: "1.0"
artifacts:
  - id: spec
    generates: ["spec.md"]
    requires: ["plan"]
    instruction: "cyclic"
  - id: plan
    generates: ["plan.md"]
    requires: ["spec"]
    instruction: "cyclic"
"#,
    )
    .unwrap();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["status", "001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stderr(predicate::str::contains("dependency graph"))
        .stdout(predicate::str::contains("spec"))
        .stdout(predicate::str::contains("plan"));
}
