#![allow(dead_code)]
use std::path::Path;

use anyhow::{Context, Result};

use crate::config;
use crate::core::feature;
use crate::extensions;

pub fn run(feature_id: Option<&str>, pass: Option<u32>) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);

    println!("Implementing: {feature_dir_name}");

    // Check tasks.md exists
    let tasks_path = feature_dir.join("tasks.md");
    if !tasks_path.exists() {
        anyhow::bail!(
            "tasks.md not found. Run 'solidspec tasks {}' first.",
            feature_dir_name
        );
    }

    // Blocking checklist check
    let checklist_path = feature_dir.join("checklists/requirements.md");
    if checklist_path.exists() {
        let checklist = std::fs::read_to_string(&checklist_path)?;
        let unchecked = checklist.matches("- [ ]").count();
        if unchecked > 0 {
            println!("  Warning: {unchecked} unchecked items in requirements checklist.");
            println!("  Review checklists/requirements.md before proceeding.");
        }
    }

    // Fire before_implement hooks
    let ext_registry = extensions::manager::load_registry(&project_root).unwrap_or_default();
    extensions::hooks::fire_hooks("before_implement", &project_root, &ext_registry);

    // Parse tasks
    let tasks_content = std::fs::read_to_string(&tasks_path)?;
    let total = tasks_content.matches("- [ ] T").count();
    let done = tasks_content.matches("- [x] T").count() + tasks_content.matches("- [X] T").count();

    let pass_num = pass.unwrap_or(1);
    println!("  Pass {pass_num}: {total} pending tasks, {done} completed");

    if total == 0 {
        println!("  All tasks completed!");
    } else {
        // Print pending tasks for the AI agent to execute
        println!("\n  Pending tasks:");
        for line in tasks_content.lines() {
            if line.contains("- [ ] T") {
                println!("    {line}");
            }
        }
        println!("\n  Execute these tasks in order, respecting [P] parallel markers.");
        println!("  Mark each completed task as [X] in tasks.md.");
    }

    // Fire after_implement hooks
    extensions::hooks::fire_hooks("after_implement", &project_root, &ext_registry);

    Ok(())
}

/// Mark a task as complete in tasks.md
pub fn mark_task_done(tasks_path: &Path, task_id: &str) -> Result<()> {
    let content = std::fs::read_to_string(tasks_path)?;
    let updated = content.replace(&format!("- [ ] {task_id}"), &format!("- [X] {task_id}"));
    std::fs::write(tasks_path, updated)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn mark_task_done_updates_checkbox() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("tasks.md");
        std::fs::write(&path, "- [ ] T001 Setup project\n- [ ] T002 Add models\n").unwrap();

        mark_task_done(&path, "T001").unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("- [X] T001"));
        assert!(content.contains("- [ ] T002")); // unchanged
    }

    #[test]
    fn mark_nonexistent_task_is_noop() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("tasks.md");
        std::fs::write(&path, "- [ ] T001 Setup\n").unwrap();

        mark_task_done(&path, "T999").unwrap(); // no panic

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("- [ ] T001")); // unchanged
    }
}
