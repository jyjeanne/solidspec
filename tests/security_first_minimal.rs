//! End-to-end coverage for the `minimal` and `security-first` schemas.
//!
//! Neither schema had dedicated pipeline-level integration coverage before
//! this file: `minimal` only got a `status` smoke check (tests/status.rs),
//! and `security-first` had none at all, despite being a headline README
//! workflow (README "Use Case 3").

use predicates::prelude::*;

mod common;
use common::{first_feature_dir, init_project, solidspec};

// ── minimal: full scaffold pipeline succeeds end-to-end ─────────────────────

#[test]
fn minimal_pipeline_no_agent_scaffolds_all_four_artifacts() {
    let dir = init_project();

    solidspec()
        .args([
            "pipeline",
            "--new",
            "Quick internal tool",
            "--schema",
            "minimal",
            "--no-agent",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("scaffold-only"))
        .stdout(predicate::str::contains("Phase 1/4: specify"))
        .stdout(predicate::str::contains("Phase 2/4: plan"))
        .stdout(predicate::str::contains("Phase 3/4: tasks"))
        .stdout(predicate::str::contains("Phase 4/4: implement"))
        .stdout(predicate::str::contains("Pipeline complete: 4 phases"));

    let feature_dir = first_feature_dir(dir.path());
    assert!(feature_dir.join("spec.md").exists());
    assert!(feature_dir.join("plan.md").exists());
    assert!(feature_dir.join("tasks.md").exists());

    // minimal has no clarify/tests/analyze/review artifacts.
    assert!(!feature_dir.join("analysis-report.md").exists());
    assert!(!feature_dir.join("review-report.md").exists());
}

#[test]
fn minimal_status_shows_only_four_artifacts_and_no_clarify_or_review() {
    let dir = init_project();

    solidspec()
        .args([
            "pipeline",
            "--new",
            "Quick internal tool",
            "--schema",
            "minimal",
            "--no-agent",
        ])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    let feature_name = feature_dir.file_name().unwrap().to_str().unwrap();

    solidspec()
        .args(["status", feature_name, "--schema", "minimal"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Schema: minimal"))
        .stdout(predicate::str::contains("4 artifacts"))
        .stdout(predicate::str::contains("spec"))
        .stdout(predicate::str::contains("plan"))
        .stdout(predicate::str::contains("tasks"))
        .stdout(predicate::str::contains("implement"))
        .stdout(predicate::str::contains("clarify").not())
        .stdout(predicate::str::contains("review").not());
}

#[test]
fn minimal_tasks_require_only_spec_and_plan_no_security_review() {
    // minimal's DAG requires ["spec", "plan"] for tasks — unlike security-first,
    // there is no extra gate, so `tasks` must succeed right after `plan`.
    let dir = init_project();

    solidspec()
        .args(["specify", "Quick tool"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    let feature_name = feature_dir.file_name().unwrap().to_str().unwrap();

    solidspec()
        .args(["plan", feature_name])
        .current_dir(dir.path())
        .assert()
        .success();

    solidspec()
        .args(["tasks", feature_name])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("tasks.md"));

    assert!(feature_dir.join("tasks.md").exists());
}

// ── security-first: DAG gate is enforced by `status` ────────────────────────

#[test]
fn security_first_status_lists_security_review_between_plan_and_tasks() {
    let dir = init_project();

    solidspec()
        .args(["specify", "Stripe payment integration"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    let feature_name = feature_dir.file_name().unwrap().to_str().unwrap();

    solidspec()
        .args(["status", feature_name, "--schema", "security-first"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Schema: security-first"))
        .stdout(predicate::str::contains("5 artifacts"))
        .stdout(predicate::str::contains("security-review"));
}

#[test]
fn security_first_tasks_blocked_until_security_review_md_exists() {
    // The schema declares `tasks.requires: ["plan", "security-review"]`, a hard
    // DAG dependency (README: "Tasks cannot be generated until
    // security-review.md exists. This is a hard DAG dependency — it cannot be
    // skipped."). `solidspec tasks` itself doesn't consult the schema graph
    // (it only checks for plan.md — see src/cli/tasks.rs), so the gate is only
    // visible through `solidspec status`. This test pins that down: status
    // must show `tasks` as blocked (not ready) when security-review.md is
    // absent, and ready once it's created.
    let dir = init_project();

    solidspec()
        .args(["specify", "Stripe payment integration"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    let feature_name = feature_dir.file_name().unwrap().to_str().unwrap();

    solidspec()
        .args(["plan", feature_name])
        .current_dir(dir.path())
        .assert()
        .success();

    // Before security-review.md exists: tasks must not be marked ready.
    let before = solidspec()
        .args(["status", feature_name, "--schema", "security-first"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    let before_stdout = String::from_utf8_lossy(&before.stdout);
    let tasks_line_before = before_stdout
        .lines()
        .find(|l| l.contains("tasks"))
        .expect("tasks row present");
    assert!(
        !tasks_line_before.contains("ready"),
        "tasks should be blocked without security-review.md, got: {tasks_line_before}"
    );

    // Create the security-review artifact via the real command.
    solidspec()
        .args(["security-review", feature_name])
        .current_dir(dir.path())
        .assert()
        .success();

    let after = solidspec()
        .args(["status", feature_name, "--schema", "security-first"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    let after_stdout = String::from_utf8_lossy(&after.stdout);
    let tasks_line_after = after_stdout
        .lines()
        .find(|l| l.contains("tasks"))
        .expect("tasks row present");
    assert!(
        tasks_line_after.contains("ready") || tasks_line_after.contains("done"),
        "tasks should be unblocked once security-review.md exists, got: {tasks_line_after}"
    );
}

#[test]
fn tasks_command_itself_blocks_without_security_review_md() {
    // Regression: `solidspec tasks` used to only check plan.md's existence
    // and never consulted the schema DAG, so a user calling it directly
    // (bypassing `solidspec pipeline`) could generate tasks.md on a
    // security-first project before security-review.md existed — silently
    // skipping the gate the schema and README describe as non-skippable.
    // `solidspec tasks` now accepts `--schema` and enforces the same DAG
    // `solidspec status` displays.
    let dir = init_project();

    solidspec()
        .args(["specify", "Stripe payment integration"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    let feature_name = feature_dir.file_name().unwrap().to_str().unwrap();

    solidspec()
        .args(["plan", feature_name])
        .current_dir(dir.path())
        .assert()
        .success();

    // Blocked: no security-review.md yet.
    solidspec()
        .args(["tasks", feature_name, "--schema", "security-first"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("blocked"))
        .stderr(predicate::str::contains("security-review"));
    assert!(!feature_dir.join("tasks.md").exists());

    // Unblocked once security-review.md exists.
    solidspec()
        .args(["security-review", feature_name])
        .current_dir(dir.path())
        .assert()
        .success();

    solidspec()
        .args(["tasks", feature_name, "--schema", "security-first"])
        .current_dir(dir.path())
        .assert()
        .success();
    assert!(feature_dir.join("tasks.md").exists());
}

#[test]
fn tasks_without_schema_flag_defaults_to_spec_driven_and_is_unaffected() {
    // No --schema passed anywhere in the codebase before this change ever
    // called `solidspec tasks` with a non-default schema, so the default
    // (spec-driven, no security-review gate) must behave exactly as before.
    let dir = init_project();

    solidspec()
        .args(["specify", "Quick tool"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    let feature_name = feature_dir.file_name().unwrap().to_str().unwrap();

    solidspec()
        .args(["plan", feature_name])
        .current_dir(dir.path())
        .assert()
        .success();

    solidspec()
        .args(["tasks", feature_name])
        .current_dir(dir.path())
        .assert()
        .success();
    assert!(feature_dir.join("tasks.md").exists());
}

// ── security-first: live pipeline runs end-to-end (README Use Case 3) ───────

#[test]
fn security_first_pipeline_no_agent_scaffolds_all_five_artifacts() {
    // README Use Case 3 demos exactly this command:
    //
    //   solidspec pipeline --new "Stripe payment integration" \
    //       --schema security-first --no-agent
    //
    // `security-review` now has a real executor (`solidspec security-review`,
    // `src/core/security_review.rs`): an OWASP Top 10 heuristic audit of
    // plan.md that requires no AI agent, so the full pipeline completes.
    let dir = init_project();

    solidspec()
        .args([
            "pipeline",
            "--new",
            "Stripe payment integration",
            "--schema",
            "security-first",
            "--no-agent",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Phase 3/5: security-review"))
        .stdout(predicate::str::contains("security-review.md created"))
        .stdout(predicate::str::contains("Pipeline complete: 5 phases"));

    let feature_dir = first_feature_dir(dir.path());
    assert!(feature_dir.join("spec.md").exists());
    assert!(feature_dir.join("plan.md").exists());
    assert!(feature_dir.join("security-review.md").exists());
    assert!(feature_dir.join("tasks.md").exists());

    // A payment feature must trip the cryptographic-failures heuristic.
    let security_review = std::fs::read_to_string(feature_dir.join("security-review.md")).unwrap();
    assert!(security_review.contains("CRITICAL"));
    assert!(security_review.contains("Cryptographic Failures"));
}

#[test]
fn security_review_command_is_idempotent() {
    let dir = init_project();

    solidspec()
        .args(["specify", "Stripe payment integration"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    let feature_name = feature_dir.file_name().unwrap().to_str().unwrap();

    solidspec()
        .args(["plan", feature_name])
        .current_dir(dir.path())
        .assert()
        .success();

    solidspec()
        .args(["security-review", feature_name])
        .current_dir(dir.path())
        .assert()
        .success();

    let first_content = std::fs::read_to_string(feature_dir.join("security-review.md")).unwrap();

    // Second run without --force must not overwrite (matches tdd-tests convention).
    solidspec()
        .args(["security-review", feature_name])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("already exists"));

    let second_content = std::fs::read_to_string(feature_dir.join("security-review.md")).unwrap();
    assert_eq!(first_content, second_content);
}

#[test]
fn security_review_dry_run_prints_without_writing() {
    let dir = init_project();

    solidspec()
        .args(["specify", "Stripe payment integration"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    let feature_name = feature_dir.file_name().unwrap().to_str().unwrap();

    solidspec()
        .args(["plan", feature_name])
        .current_dir(dir.path())
        .assert()
        .success();

    solidspec()
        .args(["security-review", feature_name, "--dry-run"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Security Review"));

    assert!(!feature_dir.join("security-review.md").exists());
}

#[test]
fn security_review_fails_without_plan_md() {
    let dir = init_project();

    solidspec()
        .args(["specify", "Stripe payment integration"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir.path());
    let feature_name = feature_dir.file_name().unwrap().to_str().unwrap();

    solidspec()
        .args(["security-review", feature_name])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("plan.md not found"));
}

#[test]
fn security_first_dry_run_previews_all_five_phases_without_executing() {
    // --dry-run must still work (it never calls execute_phase), so it's the
    // only way to preview a security-first pipeline today without hitting
    // the Unknown-phase failure.
    let dir = init_project();

    solidspec()
        .args([
            "pipeline",
            "--new",
            "Stripe payment integration",
            "--schema",
            "security-first",
            "--dry-run",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("dry"))
        .stdout(predicate::str::contains("specify"))
        .stdout(predicate::str::contains("plan"))
        .stdout(predicate::str::contains("security-review"))
        .stdout(predicate::str::contains("tasks"))
        .stdout(predicate::str::contains("implement"));
}
