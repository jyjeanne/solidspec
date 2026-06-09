use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

// ── helpers ───────────────────────────────────────────────────────────────────

fn solidspec() -> Command {
    Command::cargo_bin("solidspec").unwrap()
}

fn init_project() -> TempDir {
    let dir = TempDir::new().unwrap();
    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();
    dir
}

/// Create a feature with spec.md + plan.md + tasks.md and return its path.
fn create_feature(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
    solidspec()
        .args(["specify", name])
        .current_dir(dir)
        .assert()
        .success();

    let feature_dir = first_feature_dir(dir);

    std::fs::write(
        feature_dir.join("plan.md"),
        "# Plan\nFR-001 addressed by auth module.\n## User Stories\n- US1: login\n",
    )
    .unwrap();
    std::fs::write(
        feature_dir.join("tasks.md"),
        "# Tasks\n\
         ## Phase 1: Setup\n\
         - [ ] T001 Bootstrap project structure [US1]\n\
         - [ ] T002 Configure database [P]\n\
         ## Phase 2: Foundational\n\
         - [ ] T003 Implement auth service [US1]\n",
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

// ── A1: apex subcommand is registered in the CLI ─────────────────────────────

#[test]
fn apex_command_appears_in_help() {
    solidspec()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("apex"));
}

#[test]
fn apex_help_shows_flags() {
    solidspec()
        .args(["apex", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--sync"))
        .stdout(predicate::str::contains("--context-only"))
        .stdout(predicate::str::contains("--dry-run"));
}

// ── A2: prerequisite guard ────────────────────────────────────────────────────

#[test]
fn apex_fails_without_tasks_md() {
    let dir = init_project();

    // Create spec.md only — no tasks.md
    solidspec()
        .args(["specify", "login feature"])
        .current_dir(dir.path())
        .assert()
        .success();

    solidspec()
        .args(["apex", "001"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("tasks.md not found"));
}

#[test]
fn apex_fails_outside_project_root() {
    let dir = TempDir::new().unwrap();

    solidspec()
        .args(["apex", "001"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("solidspec init").or(predicate::str::contains("project")));
}

// ── A3: context file generation ───────────────────────────────────────────────

#[test]
fn apex_writes_context_file() {
    let dir = init_project();
    create_feature(dir.path(), "auth system");

    solidspec()
        .args(["apex", "001"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        dir.path().join(".solidspec/apex-context.md").exists(),
        ".solidspec/apex-context.md must be written"
    );
}

#[test]
fn apex_context_file_contains_feature_id() {
    let dir = init_project();
    create_feature(dir.path(), "auth system");

    solidspec()
        .args(["apex", "001"])
        .current_dir(dir.path())
        .assert()
        .success();

    let context =
        std::fs::read_to_string(dir.path().join(".solidspec/apex-context.md")).unwrap();
    assert!(
        context.contains("001-auth-system"),
        "context file must contain the feature ID"
    );
}

#[test]
fn apex_context_file_contains_pending_tasks() {
    let dir = init_project();
    create_feature(dir.path(), "auth system");

    solidspec()
        .args(["apex", "001"])
        .current_dir(dir.path())
        .assert()
        .success();

    let context =
        std::fs::read_to_string(dir.path().join(".solidspec/apex-context.md")).unwrap();
    assert!(
        context.contains("T001") || context.contains("pending"),
        "context file must reference pending tasks"
    );
}

// ── A4: --context-only ────────────────────────────────────────────────────────

#[test]
fn apex_context_only_writes_file_without_instructions() {
    let dir = init_project();
    create_feature(dir.path(), "auth system");

    solidspec()
        .args(["apex", "001", "--context-only"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Context written"))
        // Must NOT print the full invocation instructions
        .stdout(predicate::str::contains("solidspec-apex").not());

    assert!(dir.path().join(".solidspec/apex-context.md").exists());
}

// ── A5: --dry-run ─────────────────────────────────────────────────────────────

#[test]
fn apex_dry_run_prints_would_write_and_creates_no_file() {
    let dir = init_project();
    create_feature(dir.path(), "auth system");

    solidspec()
        .args(["apex", "001", "--dry-run"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Would write"));

    assert!(
        !dir.path().join(".solidspec/apex-context.md").exists(),
        "dry-run must not write the context file"
    );
}

// ── A6: --sync with no log ────────────────────────────────────────────────────

#[test]
fn apex_sync_with_no_log_reports_nothing_to_sync() {
    let dir = init_project();
    create_feature(dir.path(), "auth system");

    solidspec()
        .args(["apex", "001", "--sync"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Nothing to sync"));
}

// ── A7: --sync with execute log ───────────────────────────────────────────────

#[test]
fn apex_sync_marks_completed_tasks_from_log() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "auth system");

    // Simulate an APEX run: create apex/run-001/03-execute.md with a completion marker
    let run_dir = feature_dir.join("apex/run-001");
    std::fs::create_dir_all(&run_dir).unwrap();
    std::fs::write(
        run_dir.join("03-execute.md"),
        "# Execute Log\n\
         ## T001 Bootstrap project structure\n\
         ✓ T001 complete\n\
         ## T002 Configure database\n\
         Work in progress...\n",
    )
    .unwrap();

    solidspec()
        .args(["apex", "001", "--sync"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("marked done"));

    let tasks = std::fs::read_to_string(feature_dir.join("tasks.md")).unwrap();
    assert!(
        tasks.contains("- [x] T001") || tasks.contains("- [X] T001"),
        "T001 must be marked done after sync"
    );
    assert!(
        tasks.contains("- [ ] T002"),
        "T002 must remain pending after sync"
    );
}

// ── A8: stdout instructions ───────────────────────────────────────────────────

#[test]
fn apex_output_shows_task_summary_and_invocation() {
    let dir = init_project();
    create_feature(dir.path(), "auth system");

    solidspec()
        .args(["apex", "001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("APEX:"))
        .stdout(predicate::str::contains("pending"))
        .stdout(predicate::str::contains("/solidspec-apex"));
}

// ── A9: pipeline --schema apex-driven ─────────────────────────────────────────

#[test]
fn pipeline_apex_driven_dry_run_shows_apex_phase() {
    let dir = init_project();

    solidspec()
        .args([
            "pipeline",
            "--new",
            "Apex test feature",
            "--schema",
            "apex-driven",
            "--dry-run",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("apex"))
        .stdout(predicate::str::contains("[dry-run]"));
}

#[test]
fn pipeline_apex_driven_dry_run_excludes_implement_phase() {
    let dir = init_project();

    let output = solidspec()
        .args([
            "pipeline",
            "--new",
            "Apex test feature",
            "--schema",
            "apex-driven",
            "--dry-run",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();

    // "implement" must not appear as a phase label (only "apex" should)
    let has_implement_phase = stdout
        .lines()
        .any(|l| l.contains("implement") && l.starts_with("  Phase"));
    assert!(
        !has_implement_phase,
        "apex-driven schema must not include an 'implement' phase"
    );
}

#[test]
fn pipeline_apex_driven_dry_run_shows_handoff_for_apex() {
    let dir = init_project();

    solidspec()
        .args([
            "pipeline",
            "--new",
            "Apex test feature",
            "--schema",
            "apex-driven",
            "--dry-run",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("[HANDOFF]"));
}

// ── A10: pipeline --schema intent-apex ────────────────────────────────────────

#[test]
fn pipeline_intent_apex_dry_run_includes_intent_apex_evidence() {
    let dir = init_project();

    solidspec()
        .args([
            "pipeline",
            "--new",
            "Intent apex test",
            "--schema",
            "intent-apex",
            "--dry-run",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("intent"))
        .stdout(predicate::str::contains("apex"))
        .stdout(predicate::str::contains("evidence"));
}

#[test]
fn pipeline_intent_apex_dry_run_excludes_implement_phase() {
    let dir = init_project();

    let output = solidspec()
        .args([
            "pipeline",
            "--new",
            "Intent apex test",
            "--schema",
            "intent-apex",
            "--dry-run",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let has_implement_phase = stdout
        .lines()
        .any(|l| l.contains("implement") && l.starts_with("  Phase"));
    assert!(
        !has_implement_phase,
        "intent-apex schema must not include an 'implement' phase"
    );
}

// ── A11: existing schemas are unaffected ─────────────────────────────────────

#[test]
fn pipeline_spec_driven_dry_run_has_no_apex_phase() {
    let dir = init_project();

    let output = solidspec()
        .args([
            "pipeline",
            "--new",
            "SDD test",
            "--dry-run",
            // default schema = spec-driven
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let has_apex_phase = stdout
        .lines()
        .any(|l| l.contains(": apex") && l.starts_with("  Phase"));
    assert!(
        !has_apex_phase,
        "spec-driven schema must not contain an 'apex' phase"
    );
}

#[test]
fn pipeline_intent_driven_dry_run_has_no_apex_phase() {
    let dir = init_project();

    let output = solidspec()
        .args([
            "pipeline",
            "--new",
            "IDSD test",
            "--schema",
            "intent-driven",
            "--dry-run",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let has_apex_phase = stdout
        .lines()
        .any(|l| l.contains(": apex") && l.starts_with("  Phase"));
    assert!(
        !has_apex_phase,
        "intent-driven schema must not contain an 'apex' phase"
    );
}

// ── A12: status shows apex artifact ──────────────────────────────────────────

#[test]
fn status_shows_apex_artifact_in_apex_driven_schema() {
    let dir = init_project();
    create_feature(dir.path(), "apex status test");

    solidspec()
        .args(["status", "001", "--schema", "apex-driven"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("apex"));
}

#[test]
fn status_apex_driven_does_not_show_implement_artifact() {
    let dir = init_project();
    create_feature(dir.path(), "apex status test");

    let output = solidspec()
        .args(["status", "001", "--schema", "apex-driven"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    // The artifact rows should not list "implement" as a separate artifact
    assert!(
        !stdout.lines().any(|l| {
            let trimmed = l.trim();
            trimmed.starts_with("| implement") || trimmed.starts_with("implement ")
        }),
        "apex-driven status must not list 'implement' as an artifact"
    );
}

// ── A13: context is idempotent ────────────────────────────────────────────────

#[test]
fn apex_context_generation_is_idempotent() {
    let dir = init_project();
    create_feature(dir.path(), "auth system");

    // Run twice — must succeed both times, second run overwrites cleanly
    solidspec()
        .args(["apex", "001", "--context-only"])
        .current_dir(dir.path())
        .assert()
        .success();

    solidspec()
        .args(["apex", "001", "--context-only"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(dir.path().join(".solidspec/apex-context.md").exists());
}

// ── B1 (AC16–AC17): init registers APEX skill for Claude ─────────────────────

#[test]
fn init_registers_apex_skill_for_claude() {
    let dir = TempDir::new().unwrap();
    // Pre-create .claude/ so solidspec init detects the agent
    std::fs::create_dir_all(dir.path().join(".claude")).unwrap();

    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        dir.path().join(".claude/commands/apex/SKILL.md").exists(),
        ".claude/commands/apex/SKILL.md must be installed by init"
    );
}

#[test]
fn init_registers_apex_skill_subdirectories() {
    let dir = TempDir::new().unwrap();
    std::fs::create_dir_all(dir.path().join(".claude")).unwrap();

    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();

    let apex_dir = dir.path().join(".claude/commands/apex");
    assert!(apex_dir.join("steps").is_dir(), "steps/ subdir missing");
    assert!(
        apex_dir.join("templates").is_dir(),
        "templates/ subdir missing"
    );
    assert!(
        apex_dir.join("scripts").is_dir(),
        "scripts/ subdir missing"
    );
}

// ── B2 (AC18): init registers APEX skill for Kimi ────────────────────────────

#[test]
fn init_registers_apex_skill_for_kimi() {
    let dir = TempDir::new().unwrap();
    std::fs::create_dir_all(dir.path().join(".kimi")).unwrap();

    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        dir.path().join(".kimi/skills/apex/SKILL.md").exists(),
        ".kimi/skills/apex/SKILL.md must be installed by init"
    );
}

// ── B3 (AC19): init registers /solidspec-apex slash command for all agents ───

#[test]
fn init_registers_apex_slash_command_for_claude() {
    let dir = TempDir::new().unwrap();
    std::fs::create_dir_all(dir.path().join(".claude")).unwrap();

    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();

    let cmd_file = dir.path().join(".claude/commands/solidspec-apex.md");
    assert!(cmd_file.exists(), "solidspec-apex.md slash command must be registered");

    let content = std::fs::read_to_string(&cmd_file).unwrap();
    assert!(content.contains("APEX"), "slash command must describe APEX");
    assert!(
        content.contains("Analyze"),
        "slash command must mention the Analyze step"
    );
}

// ── B4 (AC20): upgrade restores APEX skill files ─────────────────────────────

#[test]
fn upgrade_refreshes_apex_skill_files() {
    let dir = TempDir::new().unwrap();
    std::fs::create_dir_all(dir.path().join(".claude")).unwrap();

    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify init installed the skill
    assert!(dir.path().join(".claude/commands/apex/SKILL.md").exists());

    // Simulate an old installation by removing the apex skill dir
    std::fs::remove_dir_all(dir.path().join(".claude/commands/apex")).unwrap();
    assert!(!dir.path().join(".claude/commands/apex").exists());

    // Upgrade must restore it
    solidspec()
        .args(["upgrade", "--force"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        dir.path().join(".claude/commands/apex/SKILL.md").exists(),
        "upgrade must restore .claude/commands/apex/SKILL.md"
    );
}

// ── B5 (AC21): unsupported agents get slash command, not skill directory ──────

#[test]
fn init_unsupported_agent_gets_slash_command_but_no_skill_dir() {
    let dir = TempDir::new().unwrap();
    // Cursor does not support full APEX skill dirs (apex_skill_dir returns None)
    std::fs::create_dir_all(dir.path().join(".cursor")).unwrap();
    // Also create claude so we have a reference comparison
    std::fs::create_dir_all(dir.path().join(".claude")).unwrap();

    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();

    // Claude (supported) must have the full skill directory
    assert!(dir.path().join(".claude/commands/apex").is_dir());

    // Cursor (unsupported) must NOT have an apex skill directory under any path
    let cursor_apex_rules = dir.path().join(".cursor/rules/apex");
    let cursor_apex_commands = dir.path().join(".cursor/commands/apex");
    assert!(
        !cursor_apex_rules.exists() && !cursor_apex_commands.exists(),
        "cursor must not get an apex skill directory"
    );
}

// ── B6 (T6/AC24): auto-detect feature ID ─────────────────────────────────────

#[test]
fn apex_auto_detects_feature_id() {
    let dir = init_project();
    create_feature(dir.path(), "auth system");

    // Omit feature ID — must auto-detect 001-auth-system
    solidspec()
        .args(["apex", "--context-only"])
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        dir.path().join(".solidspec/apex-context.md").exists(),
        "auto-detected feature must produce a context file"
    );
}

// ── B7 (T8): context includes FR-### requirements from spec.md ───────────────

#[test]
fn apex_context_includes_fr_requirements_from_spec() {
    let dir = init_project();
    let feature_dir = create_feature(dir.path(), "auth system");

    // Overwrite spec.md with explicit FR-### lines
    std::fs::write(
        feature_dir.join("spec.md"),
        "# Feature Specification\n\
         ## Functional Requirements\n\
         - FR-001: Users can log in with email and password\n\
         - FR-002: Sessions expire after 24 hours\n\
         ## User Stories\n\
         - US1: As a user I can authenticate\n",
    )
    .unwrap();

    solidspec()
        .args(["apex", "001", "--context-only"])
        .current_dir(dir.path())
        .assert()
        .success();

    let context =
        std::fs::read_to_string(dir.path().join(".solidspec/apex-context.md")).unwrap();
    assert!(
        context.contains("FR-001"),
        "context must contain FR-001 from spec.md"
    );
    assert!(
        context.contains("FR-002"),
        "context must contain all FR-### lines from spec.md"
    );
}

// ── B8 (AC30): pipeline --only apex --dry-run ────────────────────────────────

#[test]
fn pipeline_apex_driven_only_apex_dry_run() {
    let dir = init_project();

    solidspec()
        .args([
            "pipeline",
            "--new",
            "Only apex test",
            "--schema",
            "apex-driven",
            "--only",
            "apex",
            "--dry-run",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("apex"))
        .stdout(predicate::str::contains("[dry-run]"));
}

// ── B9 (AC31): pipeline --from tasks --to analyze includes apex ───────────────

#[test]
fn pipeline_apex_driven_from_tasks_to_analyze_dry_run() {
    let dir = init_project();

    let output = solidspec()
        .args([
            "pipeline",
            "--new",
            "Range test",
            "--schema",
            "apex-driven",
            "--from",
            "tasks",
            "--to",
            "analyze",
            "--dry-run",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    // Extract lines that look like phase entries
    let phase_lines: Vec<&str> = stdout
        .lines()
        .filter(|l| l.trim_start().starts_with("Phase"))
        .collect();

    // tasks → tests → apex → analyze  (4 phases)
    assert!(
        phase_lines.iter().any(|l| l.contains("tasks")),
        "tasks must appear in --from tasks range"
    );
    assert!(
        phase_lines.iter().any(|l| l.contains("apex")),
        "apex must appear between tasks and analyze"
    );
    assert!(
        phase_lines.iter().any(|l| l.contains("analyze")),
        "analyze must appear in --to analyze range"
    );

    // Phases before tasks or after analyze must not appear
    assert!(
        !phase_lines.iter().any(|l| l.contains("specify")),
        "specify must not appear when --from tasks"
    );
    assert!(
        !phase_lines.iter().any(|l| l.contains("review")),
        "review must not appear when --to analyze"
    );
}

// ── B10 (AC32): intent-apex pipeline has exactly 10 phases ───────────────────

#[test]
fn pipeline_intent_apex_dry_run_shows_ten_phases() {
    let dir = init_project();

    let output = solidspec()
        .args([
            "pipeline",
            "--new",
            "Ten phase test",
            "--schema",
            "intent-apex",
            "--dry-run",
        ])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    // Lines of the form "Phase X/10: ..." appear exactly 10 times
    let phase_count = stdout
        .lines()
        .filter(|l| l.trim_start().starts_with("Phase") && l.contains("/10"))
        .count();
    assert_eq!(
        phase_count, 10,
        "intent-apex must list exactly 10 phases; found {phase_count}"
    );
}

// ── B11 (T13): apex phase not skipped when 09-finish.md is absent ────────────

#[test]
fn pipeline_apex_driven_runs_apex_when_no_finish_file() {
    let dir = init_project();
    create_feature(dir.path(), "skip test");

    // apex/ exists with an in-progress run but no 09-finish.md
    let run_dir = first_feature_dir(dir.path()).join("apex/run-001");
    std::fs::create_dir_all(&run_dir).unwrap();
    std::fs::write(run_dir.join("03-execute.md"), "# In progress\n").unwrap();

    let output = solidspec()
        .args(["pipeline", "001", "--schema", "apex-driven", "--dry-run"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    // Find the dry-run phase row for apex (starts with "Phase" after whitespace)
    let apex_line = stdout
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("apex"))
        .unwrap_or("");
    assert!(
        !apex_line.contains("○ skip"),
        "apex must not be skipped when 09-finish.md is absent; got: {apex_line:?}"
    );
}

// ── B12 (T14): apex phase skipped when 09-finish.md is present ───────────────

#[test]
fn pipeline_apex_driven_skips_apex_when_finish_exists() {
    let dir = init_project();
    create_feature(dir.path(), "skip test");

    // Place 09-finish.md inside a run subdirectory → skip condition met
    let run_dir = first_feature_dir(dir.path()).join("apex/run-001");
    std::fs::create_dir_all(&run_dir).unwrap();
    std::fs::write(run_dir.join("09-finish.md"), "# Finish\nAPEX complete.\n").unwrap();

    let output = solidspec()
        .args(["pipeline", "001", "--schema", "apex-driven", "--dry-run"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let apex_line = stdout
        .lines()
        .find(|l| l.trim_start().starts_with("Phase") && l.contains("apex"))
        .unwrap_or("");
    assert!(
        apex_line.contains("○ skip"),
        "apex must be skipped when apex/run-*/09-finish.md exists; got: {apex_line:?}"
    );
}
