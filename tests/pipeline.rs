use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

mod common;
use common::first_feature_dir;

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
    let feature_dir = first_feature_dir(dir.path());
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

/// P2-T8: IDSD pipeline (--schema intent-driven) creates intent.md first, then spec.md
#[test]
fn pipeline_idsd_generates_intent_before_spec() {
    let dir = TempDir::new().unwrap();

    let mut init = Command::cargo_bin("solidspec").unwrap();
    setup_project(dir.path(), &mut init);

    Command::cargo_bin("solidspec")
        .unwrap()
        .args([
            "pipeline",
            "--new",
            "IDSD test feature",
            "--schema",
            "intent-driven",
            "--no-agent",
        ])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());

    assert!(
        feature_dir.join("intent.md").exists(),
        "intent.md must be created by IDSD pipeline"
    );
    assert!(
        feature_dir.join("spec.md").exists(),
        "spec.md must exist after specify phase"
    );
}

/// `pipeline --new` must operate on the feature it just created, even when
/// SOLIDSPEC_FEATURE points at a different (pre-existing) feature. A stale
/// env var previously redirected the plan phase to the old feature and
/// overwrote its artifacts.
#[test]
fn pipeline_new_ignores_stale_feature_env_var() {
    let dir = TempDir::new().unwrap();

    let mut init = Command::cargo_bin("solidspec").unwrap();
    setup_project(dir.path(), &mut init);

    // Pre-existing feature 001
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["specify", "First feature"])
        .current_dir(dir.path())
        .assert()
        .success();

    // New pipeline run with a stale env var pointing at 001
    Command::cargo_bin("solidspec")
        .unwrap()
        .args([
            "pipeline",
            "--new",
            "Second widget feature",
            "--no-agent",
            "--to",
            "plan",
        ])
        .env("SOLIDSPEC_FEATURE", "001")
        .current_dir(dir.path())
        .assert()
        .success();

    let specs_dir = dir.path().join("specs");
    let old_feature = std::fs::read_dir(&specs_dir)
        .unwrap()
        .flatten()
        .find(|e| e.file_name().to_string_lossy().starts_with("001"))
        .expect("feature 001 must still exist")
        .path();
    let new_feature = std::fs::read_dir(&specs_dir)
        .unwrap()
        .flatten()
        .find(|e| e.file_name().to_string_lossy().starts_with("002"))
        .expect("pipeline --new must create feature 002")
        .path();

    assert!(
        new_feature.join("spec.md").exists(),
        "new feature must have spec.md"
    );
    assert!(
        new_feature.join("plan.md").exists(),
        "plan phase must target the NEW feature, not the env-var one"
    );
    assert!(
        !old_feature.join("plan.md").exists(),
        "plan phase must NOT write into the pre-existing feature 001"
    );
}

/// The intent-apex pipeline must keep intent.md and spec.md in ONE feature
/// directory. The specify phase previously allocated a second feature number
/// because only the intent-driven schema was special-cased.
#[test]
fn pipeline_intent_apex_uses_single_feature_dir() {
    let dir = TempDir::new().unwrap();

    let mut init = Command::cargo_bin("solidspec").unwrap();
    setup_project(dir.path(), &mut init);

    Command::cargo_bin("solidspec")
        .unwrap()
        .args([
            "pipeline",
            "--new",
            "Apex intent feature",
            "--schema",
            "intent-apex",
            "--no-agent",
            "--to",
            "specify",
        ])
        .current_dir(dir.path())
        .assert()
        .success();

    let specs_dir = dir.path().join("specs");
    let feature_dirs: Vec<_> = std::fs::read_dir(&specs_dir)
        .unwrap()
        .flatten()
        .filter(|e| e.file_type().unwrap().is_dir())
        .collect();

    assert_eq!(
        feature_dirs.len(),
        1,
        "intent-apex pipeline must create exactly one feature directory, found: {:?}",
        feature_dirs
            .iter()
            .map(|e| e.file_name())
            .collect::<Vec<_>>()
    );

    let feature_dir = feature_dirs[0].path();
    assert!(
        feature_dir.join("intent.md").exists(),
        "intent.md must be in the feature dir"
    );
    assert!(
        feature_dir.join("spec.md").exists(),
        "spec.md must be in the SAME feature dir as intent.md"
    );
}

/// P2-T9: SDD pipeline (default schema) never creates intent.md
#[test]
fn pipeline_sdd_unchanged_no_intent_md() {
    let dir = TempDir::new().unwrap();

    let mut init = Command::cargo_bin("solidspec").unwrap();
    setup_project(dir.path(), &mut init);

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["pipeline", "--new", "SDD test feature", "--no-agent"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());

    assert!(
        !feature_dir.join("intent.md").exists(),
        "intent.md must NOT be created by SDD pipeline"
    );
    assert!(
        feature_dir.join("spec.md").exists(),
        "spec.md must exist in SDD pipeline"
    );
}

/// A project-local schema override that adds an extra `generates` entry to
/// an existing artifact must actually be consulted by `should_skip` — not
/// silently ignored in favor of the hardcoded, name-only check.
#[test]
fn pipeline_dry_run_respects_custom_schema_generates_override() {
    let dir = TempDir::new().unwrap();

    let mut init = Command::cargo_bin("solidspec").unwrap();
    setup_project(dir.path(), &mut init);

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["specify", "Custom schema override test"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());

    // Override spec-driven's "plan" artifact to also require research.md.
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
    requires: []
    instruction: "spec"
  - id: plan
    generates: ["plan.md", "research.md"]
    requires: ["spec"]
    instruction: "plan"
"#,
    )
    .unwrap();

    // plan.md exists but research.md does not: plan must NOT be skipped.
    std::fs::write(feature_dir.join("plan.md"), "# Plan").unwrap();

    let stdout = Command::cargo_bin("solidspec")
        .unwrap()
        .args([
            "pipeline",
            "--only",
            "plan",
            "--dry-run",
            "--auto",
            "--no-agent",
            "--schema",
            "spec-driven",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(stdout).unwrap();
    let plan_line = text
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("plan"))
        .unwrap_or("");
    assert!(
        plan_line.contains("run"),
        "plan must run when research.md (from the schema override) is missing; got: {plan_line:?}"
    );

    // Once research.md also exists, plan must be skipped.
    std::fs::write(feature_dir.join("research.md"), "# Research").unwrap();

    let stdout = Command::cargo_bin("solidspec")
        .unwrap()
        .args([
            "pipeline",
            "--only",
            "plan",
            "--dry-run",
            "--auto",
            "--no-agent",
            "--schema",
            "spec-driven",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(stdout).unwrap();
    let plan_line = text
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("plan"))
        .unwrap_or("");
    assert!(
        plan_line.contains("skip"),
        "plan must be skipped once both plan.md and research.md exist; got: {plan_line:?}"
    );
}

// ── knowledge-graph refresh after `implement` (docs/kg-workflow-vision-gap-analysis.md
// recommendation #2 / docs/okf-rs-integration-plan.md step 4) ──────────────────────────

#[test]
fn pipeline_refreshes_an_existing_knowledge_graph_after_implement() {
    let dir = TempDir::new().unwrap();
    let mut init = Command::cargo_bin("solidspec").unwrap();
    setup_project(dir.path(), &mut init);

    // A bundle generated against the project BEFORE combat.rs is added —
    // simulates the stale-bundle scenario the refresh is meant to fix.
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["okf", "generate", ".", "--output", ".solidspec/knowledge"])
        .current_dir(dir.path())
        .assert()
        .success();

    std::fs::write(
        dir.path().join("combat.rs"),
        "pub fn calculate_damage() {}\n",
    )
    .unwrap();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args([
            "pipeline",
            "--new",
            "Critical hits",
            "--auto",
            "--no-agent",
            "--to",
            "implement",
            "--schema",
            "minimal",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Knowledge graph refreshed"));

    let index_content =
        std::fs::read_to_string(dir.path().join(".solidspec/knowledge/index.md")).unwrap();
    assert!(
        index_content.contains("1"), // at least one function now indexed
        "bundle should have been regenerated to include combat.rs: {index_content}"
    );
}

#[test]
fn pipeline_never_creates_a_knowledge_graph_that_did_not_already_exist() {
    let dir = TempDir::new().unwrap();
    let mut init = Command::cargo_bin("solidspec").unwrap();
    setup_project(dir.path(), &mut init);

    // No 'solidspec okf generate' run here — the project never opted in.
    Command::cargo_bin("solidspec")
        .unwrap()
        .args([
            "pipeline",
            "--new",
            "No graph feature",
            "--auto",
            "--no-agent",
            "--to",
            "implement",
            "--schema",
            "minimal",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Knowledge graph refreshed").not());

    assert!(!dir.path().join(".solidspec/knowledge").exists());
}
