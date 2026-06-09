#![allow(dead_code)]

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

// ── Embedded APEX skill files ─────────────────────────────────────────────────
// All paths are relative to this file (src/core/apex.rs → ../../templates/apex/).
// include_str! embeds text at compile time; include_bytes! is used for shell
// scripts so their bytes are preserved exactly (line endings, permissions).
pub mod skill_files {
    pub const SKILL_MD: &str =
        include_str!("../../templates/apex/SKILL.md");

    pub const STEP_00_INIT: &str =
        include_str!("../../templates/apex/steps/step-00-init.md");
    pub const STEP_00B_BRANCH: &str =
        include_str!("../../templates/apex/steps/step-00b-branch.md");
    pub const STEP_00B_ECONOMY: &str =
        include_str!("../../templates/apex/steps/step-00b-economy.md");
    pub const STEP_00B_INTERACTIVE: &str =
        include_str!("../../templates/apex/steps/step-00b-interactive.md");
    pub const STEP_01_ANALYZE: &str =
        include_str!("../../templates/apex/steps/step-01-analyze.md");
    pub const STEP_02_PLAN: &str =
        include_str!("../../templates/apex/steps/step-02-plan.md");
    pub const STEP_03_EXECUTE: &str =
        include_str!("../../templates/apex/steps/step-03-execute.md");
    pub const STEP_04_VALIDATE: &str =
        include_str!("../../templates/apex/steps/step-04-validate.md");
    pub const STEP_05_EXAMINE: &str =
        include_str!("../../templates/apex/steps/step-05-examine.md");
    pub const STEP_06_RESOLVE: &str =
        include_str!("../../templates/apex/steps/step-06-resolve.md");
    pub const STEP_07_TESTS: &str =
        include_str!("../../templates/apex/steps/step-07-tests.md");
    pub const STEP_08_RUN_TESTS: &str =
        include_str!("../../templates/apex/steps/step-08-run-tests.md");
    pub const STEP_09_FINISH: &str =
        include_str!("../../templates/apex/steps/step-09-finish.md");

    pub const TMPL_00_CONTEXT: &str =
        include_str!("../../templates/apex/templates/00-context.md");
    pub const TMPL_01_ANALYZE: &str =
        include_str!("../../templates/apex/templates/01-analyze.md");
    pub const TMPL_02_PLAN: &str =
        include_str!("../../templates/apex/templates/02-plan.md");
    pub const TMPL_03_EXECUTE: &str =
        include_str!("../../templates/apex/templates/03-execute.md");
    pub const TMPL_04_VALIDATE: &str =
        include_str!("../../templates/apex/templates/04-validate.md");
    pub const TMPL_05_EXAMINE: &str =
        include_str!("../../templates/apex/templates/05-examine.md");
    pub const TMPL_06_RESOLVE: &str =
        include_str!("../../templates/apex/templates/06-resolve.md");
    pub const TMPL_07_TESTS: &str =
        include_str!("../../templates/apex/templates/07-tests.md");
    pub const TMPL_08_RUN_TESTS: &str =
        include_str!("../../templates/apex/templates/08-run-tests.md");
    pub const TMPL_09_FINISH: &str =
        include_str!("../../templates/apex/templates/09-finish.md");
    pub const TMPL_STEP_COMPLETE: &str =
        include_str!("../../templates/apex/templates/step-complete.md");
    pub const TMPL_SOLIDSPEC_CONTEXT: &str =
        include_str!("../../templates/apex/templates/solidspec-context.md");

    // Shell scripts are embedded as bytes to preserve exact content.
    // setup-templates.sh takes feature_name (slug without leading NNN- number)
    // as its first argument; it generates task_id internally.
    pub const SETUP_TEMPLATES_SH: &[u8] =
        include_bytes!("../../templates/apex/scripts/setup-templates.sh");
    pub const UPDATE_PROGRESS_SH: &[u8] =
        include_bytes!("../../templates/apex/scripts/update-progress.sh");
}

// ── extract_skill ─────────────────────────────────────────────────────────────

/// Write all embedded APEX skill files to `target_dir`, preserving the
/// `steps/`, `templates/`, and `scripts/` subdirectory structure.
///
/// For Claude Code the caller passes `.claude/commands/apex/`.
/// For Kimi/Vibe/OpenCode the caller passes their respective skills dirs.
///
/// Script files are written with executable permission on Unix.
/// Existing files are always overwritten so `solidspec upgrade` refreshes them.
pub fn extract_skill(target_dir: &Path) -> Result<()> {
    let steps_dir = target_dir.join("steps");
    let templates_dir = target_dir.join("templates");
    let scripts_dir = target_dir.join("scripts");

    std::fs::create_dir_all(target_dir)?;
    std::fs::create_dir_all(&steps_dir)?;
    std::fs::create_dir_all(&templates_dir)?;
    std::fs::create_dir_all(&scripts_dir)?;

    // Root skill file
    std::fs::write(target_dir.join("SKILL.md"), skill_files::SKILL_MD)?;

    // Step files
    let step_entries: &[(&str, &str)] = &[
        ("step-00-init.md",        skill_files::STEP_00_INIT),
        ("step-00b-branch.md",     skill_files::STEP_00B_BRANCH),
        ("step-00b-economy.md",    skill_files::STEP_00B_ECONOMY),
        ("step-00b-interactive.md",skill_files::STEP_00B_INTERACTIVE),
        ("step-01-analyze.md",     skill_files::STEP_01_ANALYZE),
        ("step-02-plan.md",        skill_files::STEP_02_PLAN),
        ("step-03-execute.md",     skill_files::STEP_03_EXECUTE),
        ("step-04-validate.md",    skill_files::STEP_04_VALIDATE),
        ("step-05-examine.md",     skill_files::STEP_05_EXAMINE),
        ("step-06-resolve.md",     skill_files::STEP_06_RESOLVE),
        ("step-07-tests.md",       skill_files::STEP_07_TESTS),
        ("step-08-run-tests.md",   skill_files::STEP_08_RUN_TESTS),
        ("step-09-finish.md",      skill_files::STEP_09_FINISH),
    ];
    for (name, content) in step_entries {
        std::fs::write(steps_dir.join(name), content)?;
    }

    // Template files
    let template_entries: &[(&str, &str)] = &[
        ("00-context.md",         skill_files::TMPL_00_CONTEXT),
        ("01-analyze.md",         skill_files::TMPL_01_ANALYZE),
        ("02-plan.md",            skill_files::TMPL_02_PLAN),
        ("03-execute.md",         skill_files::TMPL_03_EXECUTE),
        ("04-validate.md",        skill_files::TMPL_04_VALIDATE),
        ("05-examine.md",         skill_files::TMPL_05_EXAMINE),
        ("06-resolve.md",         skill_files::TMPL_06_RESOLVE),
        ("07-tests.md",           skill_files::TMPL_07_TESTS),
        ("08-run-tests.md",       skill_files::TMPL_08_RUN_TESTS),
        ("09-finish.md",          skill_files::TMPL_09_FINISH),
        ("step-complete.md",      skill_files::TMPL_STEP_COMPLETE),
        ("solidspec-context.md",  skill_files::TMPL_SOLIDSPEC_CONTEXT),
    ];
    for (name, content) in template_entries {
        std::fs::write(templates_dir.join(name), content)?;
    }

    // Script files (binary write to preserve exact bytes)
    write_script(&scripts_dir.join("setup-templates.sh"),   skill_files::SETUP_TEMPLATES_SH)?;
    write_script(&scripts_dir.join("update-progress.sh"),   skill_files::UPDATE_PROGRESS_SH)?;

    Ok(())
}

/// Write a script file and set executable bit on Unix.
fn write_script(path: &Path, bytes: &[u8]) -> Result<()> {
    std::fs::write(path, bytes)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(path, perms)?;
    }
    Ok(())
}

// ── build_solidspec_context ───────────────────────────────────────────────────

const MAX_CONTEXT_BYTES: usize = 16 * 1024; // 16 KB hard cap

/// Build the SolidSpec-enriched context string from feature artifacts.
///
/// Extracts:
/// - Lines under `## Functional Requirements` (until next `##`) from spec.md
/// - Lines under `## User Scenarios` (until next `##`) from spec.md
/// - First 60 lines of plan.md
/// - All `- [ ] T\d+` lines from tasks.md (pending tasks)
///
/// Missing files produce `[not yet generated]` placeholder sections.
/// Total output is hard-capped at 16 KB.
pub fn build_solidspec_context(feature_dir: &Path, feature_id: &str) -> Result<String> {
    let spec_path  = feature_dir.join("spec.md");
    let plan_path  = feature_dir.join("plan.md");
    let tasks_path = feature_dir.join("tasks.md");

    // Extract spec sections
    let (spec_requirements, spec_user_stories) = if spec_path.exists() {
        let content = std::fs::read_to_string(&spec_path)
            .with_context(|| format!("reading {}", spec_path.display()))?;
        (
            extract_section(&content, "## Functional Requirements"),
            extract_section(&content, "## User Scenarios"),
        )
    } else {
        (
            "[not yet generated — run `solidspec specify` first]".to_string(),
            "[not yet generated]".to_string(),
        )
    };

    // First 60 lines of plan.md
    let plan_summary = if plan_path.exists() {
        let content = std::fs::read_to_string(&plan_path)
            .with_context(|| format!("reading {}", plan_path.display()))?;
        let lines: Vec<&str> = content.lines().take(60).collect();
        let truncated = lines.len() == 60
            && content.lines().count() > 60;
        let mut out = lines.join("\n");
        if truncated {
            out.push_str(&format!(
                "\n\n[truncated — full plan at specs/{feature_id}/plan.md]"
            ));
        }
        out
    } else {
        "[not yet generated — run `solidspec plan` first]".to_string()
    };

    // Pending tasks
    let (pending_tasks, pending_count, completed_count) = if tasks_path.exists() {
        let content = std::fs::read_to_string(&tasks_path)
            .with_context(|| format!("reading {}", tasks_path.display()))?;
        let pending: Vec<&str> = content
            .lines()
            .filter(|l| is_pending_task(l))
            .collect();
        let completed = content
            .lines()
            .filter(|l| is_completed_task(l))
            .count();
        let pending_count = pending.len();
        let pending_str = if pending.is_empty() {
            "[all tasks completed]".to_string()
        } else {
            pending.join("\n")
        };
        (pending_str, pending_count, completed)
    } else {
        (
            "[not yet generated — run `solidspec tasks` first]".to_string(),
            0,
            0,
        )
    };

    let context = format!(
        "# SolidSpec Feature Context\n\n\
         **Feature:** {feature_id}\n\
         **Spec:** specs/{feature_id}/spec.md\n\
         **Plan:** specs/{feature_id}/plan.md\n\
         **Tasks:** specs/{feature_id}/tasks.md\n\n\
         ---\n\n\
         ## Functional Requirements (from spec.md)\n\n\
         {spec_requirements}\n\n\
         ## User Scenarios (from spec.md)\n\n\
         {spec_user_stories}\n\n\
         ## Architecture Plan (from plan.md — first 60 lines)\n\n\
         {plan_summary}\n\n\
         ## Pending Tasks (from tasks.md)\n\n\
         {pending_tasks}\n\n\
         _({pending_count} pending / {completed_count} done)_\n\n\
         ---\n\n\
         _This context was injected by `solidspec apex`. The APEX analyze phase \
         should treat this as pre-loaded discovery — do NOT re-read these files \
         unless you need full detail beyond what is shown here. Focus analysis on \
         the implementation side (existing code, patterns, dependencies) rather \
         than re-analyzing the spec._\n"
    );

    // Hard cap at 16 KB
    if context.len() > MAX_CONTEXT_BYTES {
        let mut truncated = context[..MAX_CONTEXT_BYTES].to_string();
        truncated.push_str("\n\n[context truncated at 16 KB limit]");
        return Ok(truncated);
    }

    Ok(context)
}

/// Extract lines under a markdown heading until the next `##`-level heading.
/// Returns the section body (not including the heading line itself).
/// Returns a `[not found]` message when the heading is absent.
fn extract_section(content: &str, heading: &str) -> String {
    let mut in_section = false;
    let mut lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.trim_start().starts_with(heading) {
            in_section = true;
            continue;
        }
        if in_section {
            // Stop at the next `##`-level (or higher) heading
            if line.starts_with("## ") || line.starts_with("# ") {
                break;
            }
            lines.push(line);
        }
    }

    if lines.is_empty() {
        if in_section {
            "[section is empty]".to_string()
        } else {
            format!("[section '{heading}' not found in spec.md]")
        }
    } else {
        // Trim leading/trailing blank lines
        let trimmed: Vec<&str> = lines
            .iter()
            .copied()
            .skip_while(|l| l.trim().is_empty())
            .collect();
        let end = trimmed
            .iter()
            .rposition(|l| !l.trim().is_empty())
            .map(|i| i + 1)
            .unwrap_or(0);
        trimmed[..end].join("\n")
    }
}

fn is_pending_task(line: &str) -> bool {
    // Matches `- [ ] T` followed by one or more digits
    let s = line.trim_start();
    if let Some(rest) = s.strip_prefix("- [ ] T") {
        rest.starts_with(|c: char| c.is_ascii_digit())
    } else {
        false
    }
}

fn is_completed_task(line: &str) -> bool {
    let s = line.trim_start();
    if let Some(rest) = s.strip_prefix("- [x] T") {
        rest.starts_with(|c: char| c.is_ascii_digit())
    } else {
        false
    }
}

// ── sync_tasks_from_apex_log ──────────────────────────────────────────────────

/// Result of a task-sync operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncReport {
    pub tasks_found: usize,
    pub tasks_marked_done: usize,
}

/// Parse the APEX execute log and mark completed tasks in `tasks.md`.
///
/// Detection patterns in the log:
///   `✓ T001 …`          — completion marker
///   `- [x] T001 …`      — checkbox completion
///   `### ✓ T001: …`     — section header completion
///
/// Only tasks that appear in tasks.md are counted. Running twice is safe
/// (idempotent): already-checked tasks stay checked.
pub fn sync_tasks_from_apex_log(apex_log: &Path, tasks_md: &Path) -> Result<SyncReport> {
    let log_content = std::fs::read_to_string(apex_log)
        .with_context(|| format!("reading APEX log {}", apex_log.display()))?;
    let tasks_content = std::fs::read_to_string(tasks_md)
        .with_context(|| format!("reading tasks.md {}", tasks_md.display()))?;

    let completed_ids = extract_completed_task_ids(&log_content);
    let tasks_found = completed_ids.len();

    if tasks_found == 0 {
        return Ok(SyncReport { tasks_found: 0, tasks_marked_done: 0 });
    }

    let mut tasks_marked_done = 0;
    let updated: String = tasks_content
        .lines()
        .map(|line| {
            if is_pending_task(line)
                && let Some(id) = task_id_from_pending(line)
                && completed_ids.contains(&id)
            {
                tasks_marked_done += 1;
                return line.replacen("- [ ]", "- [x]", 1);
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Preserve trailing newline if original had one
    let updated = if tasks_content.ends_with('\n') {
        format!("{updated}\n")
    } else {
        updated
    };

    std::fs::write(tasks_md, updated)
        .with_context(|| format!("writing tasks.md {}", tasks_md.display()))?;

    Ok(SyncReport { tasks_found, tasks_marked_done })
}

/// Extract task IDs (e.g. "001", "042") from lines that signal completion.
fn extract_completed_task_ids(log: &str) -> Vec<String> {
    let mut ids = Vec::new();
    // Patterns: `✓ T001`, `- [x] T001`, `### ✓ T001:`
    for line in log.lines() {
        if let Some(id) = find_task_id_after_completion_marker(line)
            && !ids.contains(&id)
        {
            ids.push(id);
        }
    }
    ids
}

/// Returns the T### numeric portion (e.g. `"001"`) if the line contains a
/// recognised completion marker immediately before a T-prefixed task ID.
fn find_task_id_after_completion_marker(line: &str) -> Option<String> {
    // Look for the checkmark character or `[x]` pattern followed by T + digits
    let markers = ["✓ T", "[x] T"];
    for marker in markers {
        if let Some(pos) = line.find(marker) {
            let after = &line[pos + marker.len()..];
            let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() {
                return Some(digits);
            }
        }
    }
    None
}

/// Extract the numeric ID from a pending task line `- [ ] T001 …`.
fn task_id_from_pending(line: &str) -> Option<String> {
    let s = line.trim_start();
    let rest = s.strip_prefix("- [ ] T")?;
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() { None } else { Some(digits) }
}

/// Find the most recent `03-execute.md` under `apex_output_dir/*/`.
/// Returns `None` if no completed execute log exists.
pub fn find_latest_execute_log(apex_output_dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(apex_output_dir).ok()?;
    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| {
            let log = e.path().join("03-execute.md");
            log.exists().then(|| {
                let mtime = std::fs::metadata(&log)
                    .and_then(|m| m.modified())
                    .ok();
                (log, mtime)
            })
        })
        .max_by_key(|(_, mtime)| *mtime)
        .map(|(path, _)| path)
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // ── extract_skill ──────────────────────────────────────────────────────────

    #[test]
    fn extract_skill_creates_all_directories() {
        let dir = TempDir::new().unwrap();
        let target = dir.path().join("apex");
        extract_skill(&target).unwrap();

        assert!(target.join("SKILL.md").exists());
        assert!(target.join("steps").is_dir());
        assert!(target.join("templates").is_dir());
        assert!(target.join("scripts").is_dir());
    }

    #[test]
    fn extract_skill_writes_all_step_files() {
        let dir = TempDir::new().unwrap();
        let target = dir.path().join("apex");
        extract_skill(&target).unwrap();

        let steps = [
            "step-00-init.md",
            "step-00b-branch.md",
            "step-00b-economy.md",
            "step-00b-interactive.md",
            "step-01-analyze.md",
            "step-02-plan.md",
            "step-03-execute.md",
            "step-04-validate.md",
            "step-05-examine.md",
            "step-06-resolve.md",
            "step-07-tests.md",
            "step-08-run-tests.md",
            "step-09-finish.md",
        ];
        for step in steps {
            assert!(
                target.join("steps").join(step).exists(),
                "missing step file: {step}"
            );
        }
    }

    #[test]
    fn extract_skill_writes_all_template_files() {
        let dir = TempDir::new().unwrap();
        let target = dir.path().join("apex");
        extract_skill(&target).unwrap();

        let templates = [
            "00-context.md", "01-analyze.md", "02-plan.md", "03-execute.md",
            "04-validate.md", "05-examine.md", "06-resolve.md", "07-tests.md",
            "08-run-tests.md", "09-finish.md", "step-complete.md",
            "solidspec-context.md",
        ];
        for tmpl in templates {
            assert!(
                target.join("templates").join(tmpl).exists(),
                "missing template file: {tmpl}"
            );
        }
    }

    #[test]
    fn extract_skill_writes_scripts() {
        let dir = TempDir::new().unwrap();
        let target = dir.path().join("apex");
        extract_skill(&target).unwrap();

        assert!(target.join("scripts").join("setup-templates.sh").exists());
        assert!(target.join("scripts").join("update-progress.sh").exists());
    }

    #[test]
    fn extract_skill_is_idempotent() {
        let dir = TempDir::new().unwrap();
        let target = dir.path().join("apex");
        extract_skill(&target).unwrap();
        // Second call must not error (overwrites existing files)
        extract_skill(&target).unwrap();
        assert!(target.join("SKILL.md").exists());
    }

    #[test]
    fn skill_md_content_is_nonempty() {
        assert!(!skill_files::SKILL_MD.is_empty());
        assert!(skill_files::SKILL_MD.contains("apex"));
    }

    // ── build_solidspec_context ────────────────────────────────────────────────

    fn write_spec(dir: &Path) {
        std::fs::write(
            dir.join("spec.md"),
            "# Feature\n\n\
             ## Functional Requirements\n\n\
             - FR-001: Users can register\n\
             - FR-002: Users can log in\n\n\
             ## User Scenarios\n\n\
             **US1:** Given a new user, when they register, they get an account.\n\n\
             ## Other\n\nsome other content\n",
        )
        .unwrap();
    }

    fn write_plan(dir: &Path, lines: usize) {
        let content: String = (1..=lines)
            .map(|i| format!("Plan line {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(dir.join("plan.md"), content).unwrap();
    }

    fn write_tasks(dir: &Path) {
        std::fs::write(
            dir.join("tasks.md"),
            "- [ ] T001 Setup\n\
             - [x] T002 Done already\n\
             - [ ] T003 Implement login\n",
        )
        .unwrap();
    }

    #[test]
    fn context_includes_fr_lines() {
        let dir = TempDir::new().unwrap();
        write_spec(dir.path());
        write_plan(dir.path(), 5);
        write_tasks(dir.path());
        let ctx = build_solidspec_context(dir.path(), "001-auth").unwrap();
        assert!(ctx.contains("FR-001"), "context must include FR-001");
        assert!(ctx.contains("FR-002"), "context must include FR-002");
    }

    #[test]
    fn context_includes_user_scenarios() {
        let dir = TempDir::new().unwrap();
        write_spec(dir.path());
        write_plan(dir.path(), 5);
        write_tasks(dir.path());
        let ctx = build_solidspec_context(dir.path(), "001-auth").unwrap();
        assert!(ctx.contains("US1"), "context must include user scenario");
    }

    #[test]
    fn context_includes_pending_tasks_only() {
        let dir = TempDir::new().unwrap();
        write_spec(dir.path());
        write_plan(dir.path(), 5);
        write_tasks(dir.path());
        let ctx = build_solidspec_context(dir.path(), "001-auth").unwrap();
        assert!(ctx.contains("T001"), "pending T001 must appear");
        assert!(ctx.contains("T003"), "pending T003 must appear");
        // Completed tasks are not listed in the pending section
        assert!(!ctx.contains("T002"), "completed T002 must not appear in pending list");
    }

    #[test]
    fn context_task_counts_are_correct() {
        let dir = TempDir::new().unwrap();
        write_spec(dir.path());
        write_plan(dir.path(), 5);
        write_tasks(dir.path());
        let ctx = build_solidspec_context(dir.path(), "001-auth").unwrap();
        assert!(ctx.contains("2 pending"), "pending count must be 2");
        assert!(ctx.contains("1 done"), "done count must be 1");
    }

    #[test]
    fn context_plan_truncated_at_60_lines() {
        let dir = TempDir::new().unwrap();
        write_spec(dir.path());
        write_plan(dir.path(), 80); // 80 lines — exceeds limit
        write_tasks(dir.path());
        let ctx = build_solidspec_context(dir.path(), "001-auth").unwrap();
        assert!(ctx.contains("truncated"), "plan >60 lines must show truncation notice");
        assert!(ctx.contains("Plan line 60"), "line 60 must be present");
        assert!(!ctx.contains("Plan line 61"), "line 61 must be truncated");
    }

    #[test]
    fn context_plan_not_truncated_when_under_limit() {
        let dir = TempDir::new().unwrap();
        write_spec(dir.path());
        write_plan(dir.path(), 30); // under limit
        write_tasks(dir.path());
        let ctx = build_solidspec_context(dir.path(), "001-auth").unwrap();
        assert!(!ctx.contains("truncated"));
        assert!(ctx.contains("Plan line 30"));
    }

    #[test]
    fn context_missing_spec_produces_placeholder() {
        let dir = TempDir::new().unwrap();
        // No spec.md
        write_plan(dir.path(), 5);
        write_tasks(dir.path());
        let ctx = build_solidspec_context(dir.path(), "001-auth").unwrap();
        assert!(ctx.contains("not yet generated"));
    }

    #[test]
    fn context_missing_all_files_produces_placeholders() {
        let dir = TempDir::new().unwrap();
        let ctx = build_solidspec_context(dir.path(), "001-auth").unwrap();
        // All three sections must have placeholders, not panic
        assert!(ctx.contains("not yet generated"));
        assert!(ctx.contains("001-auth"));
    }

    #[test]
    fn context_under_16kb_for_typical_feature() {
        let dir = TempDir::new().unwrap();
        write_spec(dir.path());
        write_plan(dir.path(), 60);
        // Simulate 20 pending tasks
        let tasks: String = (1..=20)
            .map(|i| format!("- [ ] T{i:03} Task number {i}\n"))
            .collect();
        std::fs::write(dir.path().join("tasks.md"), tasks).unwrap();
        let ctx = build_solidspec_context(dir.path(), "001-auth").unwrap();
        assert!(
            ctx.len() <= 16 * 1024,
            "context must stay under 16 KB (was {} bytes)",
            ctx.len()
        );
    }

    // ── sync_tasks_from_apex_log ───────────────────────────────────────────────

    fn make_tasks_md(dir: &Path, lines: &[&str]) {
        std::fs::write(dir.join("tasks.md"), lines.join("\n") + "\n").unwrap();
    }

    fn make_execute_log(dir: &Path, content: &str) {
        std::fs::write(dir.join("03-execute.md"), content).unwrap();
    }

    fn read_tasks(dir: &Path) -> String {
        std::fs::read_to_string(dir.join("tasks.md")).unwrap()
    }

    #[test]
    fn sync_marks_checkmark_pattern() {
        let dir = TempDir::new().unwrap();
        make_tasks_md(dir.path(), &[
            "- [ ] T001 Setup project",
            "- [ ] T002 Add handler",
            "- [ ] T003 Write tests",
        ]);
        make_execute_log(dir.path(), "Completed ✓ T001 Setup project\nWorking on T002...\n✓ T003 done");
        let report = sync_tasks_from_apex_log(
            &dir.path().join("03-execute.md"),
            &dir.path().join("tasks.md"),
        ).unwrap();
        let tasks = read_tasks(dir.path());
        assert!(tasks.contains("- [x] T001"), "T001 must be marked done");
        assert!(tasks.contains("- [ ] T002"), "T002 must remain pending");
        assert!(tasks.contains("- [x] T003"), "T003 must be marked done");
        assert_eq!(report.tasks_found, 2);
        assert_eq!(report.tasks_marked_done, 2);
    }

    #[test]
    fn sync_marks_checkbox_pattern() {
        let dir = TempDir::new().unwrap();
        make_tasks_md(dir.path(), &["- [ ] T005 Implement auth"]);
        make_execute_log(dir.path(), "- [x] T005 Implement auth — complete");
        let report = sync_tasks_from_apex_log(
            &dir.path().join("03-execute.md"),
            &dir.path().join("tasks.md"),
        ).unwrap();
        assert!(read_tasks(dir.path()).contains("- [x] T005"));
        assert_eq!(report.tasks_marked_done, 1);
    }

    #[test]
    fn sync_leaves_unlisted_tasks_unchanged() {
        let dir = TempDir::new().unwrap();
        make_tasks_md(dir.path(), &[
            "- [ ] T001 Task A",
            "- [ ] T002 Task B",
        ]);
        make_execute_log(dir.path(), "✓ T001 done");
        sync_tasks_from_apex_log(
            &dir.path().join("03-execute.md"),
            &dir.path().join("tasks.md"),
        ).unwrap();
        let tasks = read_tasks(dir.path());
        assert!(tasks.contains("- [x] T001"));
        assert!(tasks.contains("- [ ] T002"), "T002 must be unchanged");
    }

    #[test]
    fn sync_is_idempotent() {
        let dir = TempDir::new().unwrap();
        make_tasks_md(dir.path(), &["- [ ] T001 Setup"]);
        make_execute_log(dir.path(), "✓ T001");
        let log = dir.path().join("03-execute.md");
        let tasks = dir.path().join("tasks.md");
        sync_tasks_from_apex_log(&log, &tasks).unwrap();
        let after_first = read_tasks(dir.path());
        sync_tasks_from_apex_log(&log, &tasks).unwrap();
        let after_second = read_tasks(dir.path());
        assert_eq!(after_first, after_second, "sync must be idempotent");
    }

    #[test]
    fn sync_preserves_trailing_newline() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("tasks.md"), "- [ ] T001 Task\n").unwrap();
        make_execute_log(dir.path(), "✓ T001");
        sync_tasks_from_apex_log(
            &dir.path().join("03-execute.md"),
            &dir.path().join("tasks.md"),
        ).unwrap();
        let content = std::fs::read_to_string(dir.path().join("tasks.md")).unwrap();
        assert!(content.ends_with('\n'), "trailing newline must be preserved");
    }

    #[test]
    fn sync_empty_log_returns_zero_counts() {
        let dir = TempDir::new().unwrap();
        make_tasks_md(dir.path(), &["- [ ] T001 Task"]);
        make_execute_log(dir.path(), "no completion markers here");
        let report = sync_tasks_from_apex_log(
            &dir.path().join("03-execute.md"),
            &dir.path().join("tasks.md"),
        ).unwrap();
        assert_eq!(report.tasks_found, 0);
        assert_eq!(report.tasks_marked_done, 0);
        assert!(read_tasks(dir.path()).contains("- [ ] T001"), "task must remain pending");
    }

    // ── find_latest_execute_log ────────────────────────────────────────────────

    #[test]
    fn find_latest_execute_log_returns_none_when_empty() {
        let dir = TempDir::new().unwrap();
        assert!(find_latest_execute_log(dir.path()).is_none());
    }

    #[test]
    fn find_latest_execute_log_finds_log_in_subdir() {
        let dir = TempDir::new().unwrap();
        let run = dir.path().join("auth-system");
        std::fs::create_dir_all(&run).unwrap();
        std::fs::write(run.join("03-execute.md"), "log content").unwrap();
        let found = find_latest_execute_log(dir.path());
        assert!(found.is_some());
        assert_eq!(found.unwrap(), run.join("03-execute.md"));
    }
}
