use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

// ── helpers ───────────────────────────────────────────────────────────────────

fn solidspec() -> Command {
    Command::cargo_bin("solidspec").unwrap()
}

fn init_project() -> TempDir {
    let dir = TempDir::new().unwrap();
    // Create .claude/ so the agent is always detected regardless of whether
    // the `claude` CLI binary is present in PATH (required for Linux CI).
    std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();
    dir
}

fn create_feature(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
    solidspec()
        .args(["specify", name])
        .current_dir(dir)
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir);

    std::fs::write(
        feature_dir.join("spec.md"),
        "# Feature\n\
         ## Acceptance Criteria\n\
         - Users can log in with valid credentials\n\
         - Invalid credentials return 401\n\
         ## Other\n\
         Some other content.\n",
    )
    .unwrap();
    std::fs::write(
        feature_dir.join("plan.md"),
        "# Plan\nFR-001 addressed by auth module.\n",
    )
    .unwrap();
    std::fs::write(
        feature_dir.join("tasks.md"),
        "# Tasks\n\
         - [ ] T001 Bootstrap\n\
         - [ ] T002 Auth service\n",
    )
    .unwrap();

    feature_dir
}

fn first_feature_dir(dir: &std::path::Path) -> std::path::PathBuf {
    let specs = dir.join("specs");
    std::fs::read_dir(&specs)
        .unwrap()
        .flatten()
        .find(|e| e.file_type().unwrap().is_dir())
        .expect("no feature dir found")
        .path()
}

// ── T1: CLI registration ──────────────────────────────────────────────────────

#[test]
fn tdd_tests_appears_in_help() {
    solidspec()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("tdd-tests"));
}

#[test]
fn tdd_refactor_appears_in_help() {
    solidspec()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("tdd-refactor"));
}

#[test]
fn tdd_tests_help_shows_dry_run_flag() {
    solidspec()
        .args(["tdd-tests", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--dry-run"));
}

#[test]
fn tdd_refactor_help_shows_dry_run_flag() {
    solidspec()
        .args(["tdd-refactor", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--dry-run"));
}

// ── T2: project-root guard ────────────────────────────────────────────────────

#[test]
fn tdd_tests_fails_outside_project() {
    let dir = TempDir::new().unwrap();
    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("SolidSpec project"));
}

#[test]
fn tdd_refactor_fails_outside_project() {
    let dir = TempDir::new().unwrap();
    solidspec()
        .args(["tdd-refactor"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("SolidSpec project"));
}

// ── T3: tdd-tests generates tdd-red-report.md ────────────────────────────────

#[test]
fn tdd_tests_generates_red_report() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "login");

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        feature_dir.join("tdd-red-report.md").exists(),
        "tdd-red-report.md must be created"
    );
}

#[test]
fn tdd_tests_creates_tests_directory() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "login");

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        feature_dir.join("tests").is_dir(),
        "tests/ directory must be created"
    );
}

#[test]
fn tdd_tests_report_contains_acceptance_criteria() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "login");

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();
    assert!(
        report.contains("Users can log in with valid credentials"),
        "report must include spec AC items"
    );
    assert!(
        report.contains("Invalid credentials return 401"),
        "report must include spec AC items"
    );
}

#[test]
fn tdd_tests_report_has_coverage_section() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "login");

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();
    assert!(report.contains("Coverage Thresholds"));
}

// ── T4: dry-run doesn't write files ──────────────────────────────────────────

#[test]
fn tdd_tests_dry_run_prints_report_without_writing() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "login");

    solidspec()
        .args(["tdd-tests", "--dry-run"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TDD Red Report"));

    assert!(
        !feature_dir.join("tdd-red-report.md").exists(),
        "dry-run must not write tdd-red-report.md"
    );
}

#[test]
fn tdd_refactor_dry_run_prints_report_without_writing() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "login");

    // Seed the red report so tdd-refactor can proceed
    std::fs::write(
        feature_dir.join("tdd-red-report.md"),
        "# TDD Red Report\nComplete.\n",
    )
    .unwrap();

    solidspec()
        .args(["tdd-refactor", "--dry-run"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TDD Refactor Report"));

    assert!(
        !feature_dir.join("tdd-refactor-report.md").exists(),
        "dry-run must not write tdd-refactor-report.md"
    );
}

// ── T5: tdd-refactor requires red report ─────────────────────────────────────

#[test]
fn tdd_refactor_fails_without_red_report() {
    let dir = init_project();
    create_feature(dir.path(), "login");

    solidspec()
        .args(["tdd-refactor"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("tdd-red-report.md not found"));
}

#[test]
fn tdd_refactor_succeeds_with_red_report() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "login");

    std::fs::write(
        feature_dir.join("tdd-red-report.md"),
        "# TDD Red Report\nComplete.\n",
    )
    .unwrap();

    solidspec()
        .args(["tdd-refactor"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        feature_dir.join("tdd-refactor-report.md").exists(),
        "tdd-refactor-report.md must be created"
    );
}

// ── T6: idempotency ──────────────────────────────────────────────────────────

#[test]
fn tdd_tests_is_idempotent() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "login");

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let content_first = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();

    // Second run should skip (report already exists)
    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("already exists"));

    let content_second = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();
    assert_eq!(
        content_first, content_second,
        "report content must not change on second run"
    );
}

#[test]
fn tdd_refactor_is_idempotent() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "login");

    std::fs::write(
        feature_dir.join("tdd-red-report.md"),
        "# TDD Red Report\nComplete.\n",
    )
    .unwrap();

    solidspec()
        .args(["tdd-refactor"])
        .current_dir(dir.path())
        .assert()
        .success();

    let content_first =
        std::fs::read_to_string(feature_dir.join("tdd-refactor-report.md")).unwrap();

    solidspec()
        .args(["tdd-refactor"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("already exists"));

    let content_second =
        std::fs::read_to_string(feature_dir.join("tdd-refactor-report.md")).unwrap();
    assert_eq!(content_first, content_second);
}

// ── T7: schema registration in solidspec init ─────────────────────────────────

#[test]
fn tdd_driven_schema_listed_by_pipeline_dry_run() {
    let dir = init_project();
    solidspec()
        .args(["specify", "login"])
        .current_dir(dir.path())
        .assert()
        .success();

    solidspec()
        .args(["pipeline", "--schema", "tdd-driven", "--dry-run", "--auto"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("tdd-tests"))
        .stdout(predicate::str::contains("tdd-refactor"));
}

// ── T8: pipeline skip logic ───────────────────────────────────────────────────

#[test]
fn pipeline_tdd_skips_tdd_tests_when_red_report_exists() {
    let dir = init_project();
    let feature_dir = first_feature_dir_after_specify(dir.path(), "login");

    // Seed all prior artifacts + the red report
    seed_tdd_artifacts(&feature_dir);
    std::fs::write(
        feature_dir.join("tdd-red-report.md"),
        "# TDD Red Report\nComplete.\n",
    )
    .unwrap();

    let stdout = solidspec()
        .args(["pipeline", "--schema", "tdd-driven", "--dry-run", "--auto"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(stdout).unwrap();

    let tdd_tests_line = stdout
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("tdd-tests"))
        .unwrap_or("");
    assert!(
        tdd_tests_line.contains("skip") || tdd_tests_line.contains("SKIP"),
        "tdd-tests must be skipped when tdd-red-report.md exists; got: {tdd_tests_line:?}"
    );
}

#[test]
fn pipeline_tdd_skips_tdd_refactor_when_refactor_report_exists() {
    let dir = init_project();
    let feature_dir = first_feature_dir_after_specify(dir.path(), "login");

    seed_tdd_artifacts(&feature_dir);
    std::fs::write(
        feature_dir.join("tdd-red-report.md"),
        "# TDD Red Report\nComplete.\n",
    )
    .unwrap();
    std::fs::write(
        feature_dir.join("tdd-refactor-report.md"),
        "# TDD Refactor Report\nComplete.\n",
    )
    .unwrap();

    let stdout = solidspec()
        .args(["pipeline", "--schema", "tdd-driven", "--dry-run", "--auto"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(stdout).unwrap();

    let refactor_line = stdout
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("tdd-refactor"))
        .unwrap_or("");
    assert!(
        refactor_line.contains("skip") || refactor_line.contains("SKIP"),
        "tdd-refactor must be skipped when tdd-refactor-report.md exists; got: {refactor_line:?}"
    );
}

// ── T9: slash commands are registered on init ─────────────────────────────────

#[test]
fn init_registers_tdd_tests_command_for_claude() {
    let dir = init_project();
    let commands_dir = dir.path().join(".claude/commands");
    let cmd_file = commands_dir.join("solidspec-tdd-tests.md");
    assert!(
        cmd_file.exists(),
        "solidspec-tdd-tests.md must be registered for claude; checked: {}",
        cmd_file.display()
    );
}

#[test]
fn init_registers_tdd_refactor_command_for_claude() {
    let dir = init_project();
    let commands_dir = dir.path().join(".claude/commands");
    let cmd_file = commands_dir.join("solidspec-tdd-refactor.md");
    assert!(
        cmd_file.exists(),
        "solidspec-tdd-refactor.md must be registered for claude; checked: {}",
        cmd_file.display()
    );
}

#[test]
fn tdd_tests_command_body_mentions_red_phase() {
    let dir = init_project();
    let cmd_file = dir.path().join(".claude/commands/solidspec-tdd-tests.md");
    let body = std::fs::read_to_string(&cmd_file).unwrap();
    assert!(
        body.contains("RED") || body.contains("failing"),
        "tdd-tests command must mention RED phase or failing tests"
    );
}

#[test]
fn tdd_refactor_command_body_mentions_green_phase() {
    let dir = init_project();
    let cmd_file = dir
        .path()
        .join(".claude/commands/solidspec-tdd-refactor.md");
    let body = std::fs::read_to_string(&cmd_file).unwrap();
    assert!(
        body.contains("GREEN") || body.contains("REFACTOR"),
        "tdd-refactor command must mention GREEN or REFACTOR"
    );
}

// ── T10: upgrade registers tdd commands ──────────────────────────────────────

#[test]
fn upgrade_registers_tdd_tests_command() {
    let dir = init_project();

    // Remove the command to simulate upgrade scenario
    let cmd_file = dir.path().join(".claude/commands/solidspec-tdd-tests.md");
    std::fs::remove_file(&cmd_file).unwrap();

    solidspec()
        .args(["upgrade", "--force"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(cmd_file.exists(), "upgrade must re-register tdd-tests.md");
}

// ── T11: status shows tdd artifacts ──────────────────────────────────────────

#[test]
fn status_tdd_driven_shows_tdd_phases() {
    let dir = init_project();
    solidspec()
        .args(["specify", "login"])
        .current_dir(dir.path())
        .assert()
        .success();

    solidspec()
        .args(["status", "--schema", "tdd-driven"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("tdd-tests"))
        .stdout(predicate::str::contains("tdd-refactor"));
}

// ── T12: Report structure (skill-aligned content) ────────────────────────────

#[test]
fn red_report_has_interface_design_section() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "auth");

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();
    assert!(
        report.contains("Interface Design"),
        "must have Interface Design section"
    );
    assert!(
        report.contains("Mock boundaries"),
        "must list mock boundaries"
    );
    assert!(
        report.contains("Do NOT mock"),
        "must explicitly state what NOT to mock"
    );
}

#[test]
fn red_report_tracer_bullet_contains_first_ac() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "auth");

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();
    // First AC from spec.md is "Users can log in with valid credentials"
    assert!(
        report.contains("**Behavior**: Users can log in with valid credentials"),
        "first AC must be the tracer bullet"
    );
    // Second AC must appear in the cycle table
    assert!(
        report.contains("Invalid credentials return 401"),
        "remaining ACs must appear in the cycle table"
    );
}

#[test]
fn red_report_has_test_quality_checklist() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "auth");

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();
    assert!(
        report.contains("Test Quality Checklist"),
        "must have quality checklist"
    );
    assert!(
        report.contains("observable behavior"),
        "checklist must mention observable behavior"
    );
    assert!(
        report.contains("public APIs only"),
        "checklist must enforce public-API-only testing"
    );
}

#[test]
fn red_report_has_unexpectedly_passing_field() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "auth");

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();
    assert!(
        report.contains("Unexpectedly passing"),
        "must have unexpectedly passing tests section"
    );
}

#[test]
fn refactor_report_has_candidates_checklist() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "auth");

    std::fs::write(
        feature_dir.join("tdd-red-report.md"),
        "# TDD Red Report\nComplete.\n",
    )
    .unwrap();

    solidspec()
        .args(["tdd-refactor"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-refactor-report.md")).unwrap();
    assert!(
        report.contains("Refactor Candidates"),
        "must have candidates section"
    );
    assert!(report.contains("Duplication"), "must list duplication");
    assert!(
        report.contains("Shallow modules"),
        "must list shallow modules"
    );
    assert!(report.contains("Feature envy"), "must list feature envy");
    assert!(
        report.contains("Interface creep"),
        "must warn about interface growth"
    );
}

#[test]
fn refactor_report_changes_table_has_type_column() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "auth");

    std::fs::write(
        feature_dir.join("tdd-red-report.md"),
        "# TDD Red Report\nComplete.\n",
    )
    .unwrap();

    solidspec()
        .args(["tdd-refactor"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-refactor-report.md")).unwrap();
    assert!(
        report.contains("Refactor Type"),
        "changes table must have a Refactor Type column"
    );
}

#[test]
fn red_report_single_ac_shows_no_remaining_cycles() {
    let dir = init_project();

    solidspec()
        .args(["specify", "one-criterion"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    std::fs::write(
        feature_dir.join("spec.md"),
        "# Feature\n## Acceptance Criteria\n- Only criterion\n",
    )
    .unwrap();

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();
    assert!(
        report.contains("**Behavior**: Only criterion"),
        "single AC must be tracer bullet"
    );
    assert!(
        report.contains("all criteria covered by the tracer bullet"),
        "must say no remaining cycles"
    );
}

// ── T13: Explicit feature ID resolution ──────────────────────────────────────

#[test]
fn tdd_tests_with_explicit_feature_id() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "auth");

    solidspec()
        .args(["tdd-tests", "001"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        feature_dir.join("tdd-red-report.md").exists(),
        "tdd-red-report.md must be created for feature 001"
    );
}

#[test]
fn tdd_refactor_with_explicit_feature_id() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "auth");

    std::fs::write(
        feature_dir.join("tdd-red-report.md"),
        "# TDD Red Report\nComplete.\n",
    )
    .unwrap();

    solidspec()
        .args(["tdd-refactor", "001"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        feature_dir.join("tdd-refactor-report.md").exists(),
        "tdd-refactor-report.md must be created for explicit feature 001"
    );
}

#[test]
fn tdd_tests_fails_gracefully_when_feature_dir_missing() {
    let dir = init_project();

    solidspec()
        .args(["tdd-tests", "999"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("No feature")));
}

// ── T14: Acceptance criteria edge cases ──────────────────────────────────────

#[test]
fn red_report_uses_given_when_then_as_ac_fallback() {
    let dir = init_project();

    solidspec()
        .args(["specify", "checkout"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    // No ## Acceptance Criteria section — only GIVEN/WHEN/THEN patterns
    std::fs::write(
        feature_dir.join("spec.md"),
        "# Feature\n\
         ## User Story 1\n\
         GIVEN: a user has items in cart\n\
         WHEN: they checkout\n\
         THEN: order is confirmed\n",
    )
    .unwrap();

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();
    // GIVEN/WHEN/THEN lines are the fallback criteria
    assert!(
        report.contains("GIVEN:") || report.contains("cart"),
        "GIVEN/WHEN/THEN fallback must appear in tracer bullet"
    );
}

#[test]
fn red_report_captures_criteria_after_subsection_header() {
    let dir = init_project();

    solidspec()
        .args(["specify", "subsection-test"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    std::fs::write(
        feature_dir.join("spec.md"),
        "# Feature\n\
         ## Acceptance Criteria\n\
         - First behavior\n\
         ### Security Notes\n\
         Some note.\n\
         - Second behavior after sub-header\n\
         ## Something Else\n\
         Ignored.\n",
    )
    .unwrap();

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();
    assert!(
        report.contains("Second behavior after sub-header"),
        "criteria after a ### sub-header must still be captured"
    );
}

#[test]
fn red_report_graceful_when_spec_missing() {
    let dir = init_project();

    solidspec()
        .args(["specify", "no-spec"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    // Remove spec.md to simulate missing file
    let spec_path = feature_dir.join("spec.md");
    if spec_path.exists() {
        std::fs::remove_file(&spec_path).unwrap();
    }
    // Write minimal tasks.md so tdd-tests won't fail on feature resolution
    std::fs::write(feature_dir.join("tasks.md"), "# Tasks\n").unwrap();

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let report = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();
    assert!(
        report.contains("no acceptance criteria found"),
        "missing spec.md must produce a graceful placeholder"
    );
}

// ── T15: Pipeline flags and phase ordering ────────────────────────────────────

#[test]
fn pipeline_dry_run_from_tdd_tests_skips_earlier_phases() {
    let dir = init_project();
    first_feature_dir_after_specify(dir.path(), "auth");

    solidspec()
        .args([
            "pipeline",
            "--schema",
            "tdd-driven",
            "--from",
            "tdd-tests",
            "--dry-run",
            "--auto",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("tdd-tests"))
        .stdout(predicate::str::contains("specify").not())
        .stdout(predicate::str::contains("plan").not());
}

#[test]
fn pipeline_dry_run_only_tdd_tests_shows_one_phase() {
    let dir = init_project();
    first_feature_dir_after_specify(dir.path(), "auth");

    let stdout = solidspec()
        .args([
            "pipeline",
            "--schema",
            "tdd-driven",
            "--only",
            "tdd-tests",
            "--dry-run",
            "--auto",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(stdout).unwrap();

    let phase_lines: Vec<_> = text
        .lines()
        .filter(|l| l.trim_start().starts_with("Phase"))
        .collect();
    assert_eq!(
        phase_lines.len(),
        1,
        "exactly one Phase line must appear; got: {phase_lines:?}"
    );
    assert!(phase_lines[0].contains("tdd-tests"));
}

#[test]
fn pipeline_force_reruns_tdd_tests_when_red_report_exists() {
    let dir = init_project();
    let feature_dir = first_feature_dir_after_specify(dir.path(), "auth");

    // Seed red report and all earlier artifacts so pipeline can proceed
    seed_tdd_artifacts(&feature_dir);
    let original = "# TDD Red Report\nOriginal content.\n";
    std::fs::write(feature_dir.join("tdd-red-report.md"), original).unwrap();

    // Without --force, tdd-tests is skipped
    let stdout_no_force = solidspec()
        .args(["pipeline", "--schema", "tdd-driven", "--dry-run", "--auto"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text_no_force = String::from_utf8(stdout_no_force).unwrap();
    let tdd_line = text_no_force
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("tdd-tests"))
        .unwrap_or("");
    assert!(
        tdd_line.contains("skip"),
        "without --force, tdd-tests must be skipped"
    );

    // With --force, tdd-tests must be scheduled to run
    let stdout_force = solidspec()
        .args([
            "pipeline",
            "--schema",
            "tdd-driven",
            "--force",
            "--dry-run",
            "--auto",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text_force = String::from_utf8(stdout_force).unwrap();
    let tdd_line_force = text_force
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("tdd-tests"))
        .unwrap_or("");
    assert!(
        tdd_line_force.contains("run"),
        "with --force, tdd-tests must be scheduled to run; got: {tdd_line_force:?}"
    );
}

#[test]
fn pipeline_dry_run_shows_handoff_label_for_tdd_phases() {
    let dir = init_project();
    first_feature_dir_after_specify(dir.path(), "auth");

    let stdout = solidspec()
        .args(["pipeline", "--schema", "tdd-driven", "--dry-run", "--auto"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(stdout).unwrap();

    let tdd_tests_line = text
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("tdd-tests"))
        .unwrap_or("");
    let tdd_refactor_line = text
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("tdd-refactor"))
        .unwrap_or("");

    assert!(
        tdd_tests_line.contains("HANDOFF"),
        "tdd-tests must be labelled [HANDOFF]; got: {tdd_tests_line:?}"
    );
    assert!(
        tdd_refactor_line.contains("HANDOFF"),
        "tdd-refactor must be labelled [HANDOFF]; got: {tdd_refactor_line:?}"
    );
}

#[test]
fn pipeline_tdd_phase_numbers_are_correct() {
    let dir = init_project();
    first_feature_dir_after_specify(dir.path(), "auth");

    let stdout = solidspec()
        .args(["pipeline", "--schema", "tdd-driven", "--dry-run", "--auto"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(stdout).unwrap();

    // tdd-driven has 9 phases; tdd-tests is #5, implement #6, tdd-refactor #7
    let tdd_tests_line = text
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("tdd-tests"))
        .unwrap_or("");
    let tdd_refactor_line = text
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("tdd-refactor"))
        .unwrap_or("");

    assert!(
        tdd_tests_line.contains("5/9"),
        "tdd-tests must be phase 5/9; got: {tdd_tests_line:?}"
    );
    assert!(
        tdd_refactor_line.contains("7/9"),
        "tdd-refactor must be phase 7/9; got: {tdd_refactor_line:?}"
    );
}

#[test]
fn pipeline_new_tdd_driven_scaffolds_both_reports() {
    let dir = init_project();

    solidspec()
        .args([
            "pipeline",
            "--new",
            "user authentication",
            "--schema",
            "tdd-driven",
            "--no-agent",
            "--auto",
        ])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    assert!(
        feature_dir.join("spec.md").exists(),
        "spec.md must be created"
    );
    assert!(
        feature_dir.join("plan.md").exists(),
        "plan.md must be created"
    );
    assert!(
        feature_dir.join("tasks.md").exists(),
        "tasks.md must be created"
    );
    assert!(
        feature_dir.join("tdd-red-report.md").exists(),
        "tdd-red-report.md must be scaffolded by tdd-tests phase"
    );
    assert!(
        feature_dir.join("tdd-refactor-report.md").exists(),
        "tdd-refactor-report.md must be scaffolded by tdd-refactor phase"
    );
}

// ── T16: Status DAG completion detection ─────────────────────────────────────

#[test]
fn status_tdd_tests_shows_ready_when_no_artifacts() {
    let dir = init_project();

    solidspec()
        .args(["specify", "auth"])
        .current_dir(dir.path())
        .assert()
        .success();

    let stdout = solidspec()
        .args(["status", "--schema", "tdd-driven"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(stdout).unwrap();

    let tdd_line = text.lines().find(|l| l.contains("tdd-tests")).unwrap_or("");
    // spec.md was created by specify, so tdd-tests deps are partially met
    // tdd-tests requires spec + tasks; tasks.md doesn't exist yet → blocked
    assert!(
        tdd_line.contains("blocked") || tdd_line.contains("ready"),
        "tdd-tests must be blocked or ready (not done) before artifacts exist; got: {tdd_line:?}"
    );
    assert!(
        !tdd_line.contains("done"),
        "tdd-tests must not be done before tests are written; got: {tdd_line:?}"
    );
}

#[test]
fn status_tdd_tests_shows_done_when_tests_dir_nonempty_and_report_exists() {
    let dir = init_project();
    let feature_dir = first_feature_dir_after_specify(dir.path(), "auth");

    // Non-empty tests/ directory AND red report = both generates satisfied
    let tests_dir = feature_dir.join("tests");
    std::fs::create_dir_all(&tests_dir).unwrap();
    std::fs::write(tests_dir.join("auth_test.rs"), "// test").unwrap();
    std::fs::write(feature_dir.join("tdd-red-report.md"), "# Report\n").unwrap();

    let stdout = solidspec()
        .args(["status", "--schema", "tdd-driven"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(stdout).unwrap();

    let tdd_line = text.lines().find(|l| l.contains("tdd-tests")).unwrap_or("");
    assert!(
        tdd_line.contains("done"),
        "tdd-tests must be done when tests/ is non-empty and red report exists; got: {tdd_line:?}"
    );
}

#[test]
fn status_tdd_tests_not_done_when_tests_dir_is_empty() {
    let dir = init_project();
    let feature_dir = first_feature_dir_after_specify(dir.path(), "auth");

    // Empty tests/ + red report: the directory pattern requires non-empty
    std::fs::create_dir_all(feature_dir.join("tests")).unwrap(); // empty
    std::fs::write(feature_dir.join("tdd-red-report.md"), "# Report\n").unwrap();

    let stdout = solidspec()
        .args(["status", "--schema", "tdd-driven"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(stdout).unwrap();

    let tdd_line = text.lines().find(|l| l.contains("tdd-tests")).unwrap_or("");
    assert!(
        !tdd_line.contains("done"),
        "tdd-tests must NOT be done when tests/ dir is empty; got: {tdd_line:?}"
    );
}

#[test]
fn status_tdd_refactor_shows_done_when_report_exists() {
    let dir = init_project();
    let feature_dir = first_feature_dir_after_specify(dir.path(), "auth");

    std::fs::write(feature_dir.join("tdd-refactor-report.md"), "# Refactor\n").unwrap();

    let stdout = solidspec()
        .args(["status", "--schema", "tdd-driven"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(stdout).unwrap();

    let refactor_line = text
        .lines()
        .find(|l| l.contains("tdd-refactor"))
        .unwrap_or("");
    assert!(
        refactor_line.contains("done"),
        "tdd-refactor must be done when report exists; got: {refactor_line:?}"
    );
}

#[test]
fn status_tdd_driven_schema_shows_correct_artifact_count() {
    let dir = init_project();
    solidspec()
        .args(["specify", "auth"])
        .current_dir(dir.path())
        .assert()
        .success();

    // tdd-driven schema has 10 artifacts (spec, clarify, plan, tasks, tdd-tests,
    // implement, tdd-refactor, analyze, review, ship)
    solidspec()
        .args(["status", "--schema", "tdd-driven"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("10 artifacts"));
}

// ── T17: Agent command body reflects skill improvements ───────────────────────

#[test]
fn tdd_tests_command_mentions_tracer_bullet() {
    let dir = init_project();
    let cmd = dir.path().join(".claude/commands/solidspec-tdd-tests.md");
    let body = std::fs::read_to_string(&cmd).unwrap();
    assert!(
        body.to_lowercase().contains("tracer bullet") || body.contains("TRACER BULLET"),
        "tdd-tests command must mention tracer bullet"
    );
}

#[test]
fn tdd_tests_command_mentions_interface_design() {
    let dir = init_project();
    let cmd = dir.path().join(".claude/commands/solidspec-tdd-tests.md");
    let body = std::fs::read_to_string(&cmd).unwrap();
    assert!(
        body.contains("INTERFACE DESIGN") || body.contains("Interface Design"),
        "tdd-tests command must mention interface design"
    );
}

#[test]
fn tdd_tests_command_mentions_mock_boundaries() {
    let dir = init_project();
    let cmd = dir.path().join(".claude/commands/solidspec-tdd-tests.md");
    let body = std::fs::read_to_string(&cmd).unwrap();
    assert!(
        body.contains("MOCK BOUNDARIES")
            || body.contains("Mock boundaries")
            || body.contains("mock boundary"),
        "tdd-tests command must explain mock boundaries"
    );
    assert!(
        body.contains("DO NOT mock") || body.contains("do not mock") || body.contains("Don't mock"),
        "tdd-tests command must say what NOT to mock"
    );
}

#[test]
fn tdd_refactor_command_lists_specific_candidates() {
    let dir = init_project();
    let cmd = dir
        .path()
        .join(".claude/commands/solidspec-tdd-refactor.md");
    let body = std::fs::read_to_string(&cmd).unwrap();
    assert!(body.contains("Duplication"), "must list duplication");
    assert!(
        body.contains("Shallow modules") || body.contains("shallow module"),
        "must mention shallow modules"
    );
    assert!(
        body.contains("Feature envy") || body.contains("feature envy"),
        "must mention feature envy"
    );
}

#[test]
fn tdd_refactor_command_warns_about_interface_growth() {
    let dir = init_project();
    let cmd = dir
        .path()
        .join(".claude/commands/solidspec-tdd-refactor.md");
    let body = std::fs::read_to_string(&cmd).unwrap();
    assert!(
        body.contains("interface") && (body.contains("must not") || body.contains("must stay")),
        "tdd-refactor command must warn that interface must not grow"
    );
}

// ── T18: End-to-end multi-feature scenarios ───────────────────────────────────

#[test]
fn tdd_tests_resolves_to_second_feature_when_explicitly_given() {
    let dir = init_project();

    // Feature 001
    solidspec()
        .args(["specify", "feature-one"])
        .current_dir(dir.path())
        .assert()
        .success();

    // Feature 002
    solidspec()
        .args(["specify", "feature-two"])
        .current_dir(dir.path())
        .assert()
        .success();

    // Run tdd-tests on feature 001 explicitly
    let specs_dir = dir.path().join("specs");
    let mut dirs: Vec<_> = std::fs::read_dir(&specs_dir)
        .unwrap()
        .flatten()
        .filter(|e| e.file_type().unwrap().is_dir())
        .collect();
    dirs.sort_by_key(|e| e.file_name());
    let feature_001 = dirs[0].path();
    let feature_002 = dirs[1].path();

    solidspec()
        .args(["tdd-tests", "001"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        feature_001.join("tdd-red-report.md").exists(),
        "red report must be in feature 001"
    );
    assert!(
        !feature_002.join("tdd-red-report.md").exists(),
        "red report must NOT be in feature 002"
    );
}

#[test]
fn full_tdd_workflow_scaffold_is_consistent() {
    // End-to-end: specify → tdd-tests → tdd-refactor, verify artifacts are coherent.
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "payment");

    solidspec()
        .args(["tdd-tests"])
        .current_dir(dir.path())
        .assert()
        .success();

    let red_report = std::fs::read_to_string(feature_dir.join("tdd-red-report.md")).unwrap();

    // Both AC items from create_feature spec.md must appear in red report
    assert!(red_report.contains("Users can log in with valid credentials"));
    assert!(red_report.contains("Invalid credentials return 401"));
    assert!(
        red_report.contains("Tracer Bullet"),
        "must have tracer bullet section"
    );
    assert!(
        !red_report.contains("Refactor Candidates"),
        "red report must not contain refactor section"
    );

    solidspec()
        .args(["tdd-refactor"])
        .current_dir(dir.path())
        .assert()
        .success();

    let refactor_report =
        std::fs::read_to_string(feature_dir.join("tdd-refactor-report.md")).unwrap();
    assert!(refactor_report.contains("Refactor Candidates"));
    assert!(refactor_report.contains("Definition of Done"));
    assert!(
        refactor_report.to_lowercase().contains("interface"),
        "refactor report must mention interface preservation"
    );

    // Status shows tdd-tests NOT done (tests/ is empty scaffold) but tdd-refactor IS done
    let stdout = solidspec()
        .args(["status", "--schema", "tdd-driven"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(stdout).unwrap();

    let refactor_line = text
        .lines()
        .find(|l| l.contains("tdd-refactor"))
        .unwrap_or("");
    assert!(
        refactor_line.contains("done"),
        "tdd-refactor must be done after scaffold; got: {refactor_line:?}"
    );
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn first_feature_dir_after_specify(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
    solidspec()
        .args(["specify", name])
        .current_dir(dir)
        .assert()
        .success();
    first_feature_dir(dir)
}

fn seed_tdd_artifacts(feature_dir: &std::path::Path) {
    std::fs::write(feature_dir.join("spec.md"), "# Spec\n").unwrap();
    std::fs::write(feature_dir.join("plan.md"), "# Plan\n").unwrap();
    std::fs::write(feature_dir.join("tasks.md"), "# Tasks\n- [x] T001 Done\n").unwrap();
    std::fs::write(feature_dir.join("analysis-report.md"), "# Analysis\n").unwrap();
    std::fs::write(feature_dir.join("review-report.md"), "# Review\n").unwrap();
}
