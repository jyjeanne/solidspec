use anyhow::{Context, Result};

use crate::config;
use crate::core::{apex, feature};

pub fn run(feature_id: Option<&str>, sync: bool, context_only: bool, dry_run: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);

    let tasks_path = feature_dir.join("tasks.md");
    if !tasks_path.exists() {
        anyhow::bail!(
            "tasks.md not found. Run 'solidspec tasks {}' first.",
            feature_dir_name
        );
    }

    // --sync: read the latest apex execute log and mark completed tasks in tasks.md
    if sync {
        let apex_output_dir = feature_dir.join("apex");
        match apex::find_latest_execute_log(&apex_output_dir) {
            Some(log_path) => {
                let report = apex::sync_tasks_from_apex_log(&log_path, &tasks_path)?;
                println!(
                    "Synced {}/{} task(s) marked done.",
                    report.tasks_marked_done, report.tasks_found
                );
            }
            None => {
                println!("No apex execute log found. Nothing to sync.");
            }
        }
        return Ok(());
    }

    // Build the SolidSpec context document
    let context = apex::build_solidspec_context(&feature_dir, &feature_dir_name)?;
    let context_path = project_root.join(".solidspec/apex-context.md");

    if dry_run {
        println!(
            "Would write context ({} bytes) to {}",
            context.len(),
            context_path.display()
        );
        println!("Feature: {feature_dir_name}");
        return Ok(());
    }

    std::fs::create_dir_all(context_path.parent().expect("context path has parent dir"))?;
    std::fs::write(&context_path, &context)?;

    if context_only {
        println!("Context written: {}", context_path.display());
        return Ok(());
    }

    // Task summary
    let tasks_content = std::fs::read_to_string(&tasks_path)?;
    let pending = tasks_content.matches("- [ ] T").count();
    let done = tasks_content.matches("- [x] T").count() + tasks_content.matches("- [X] T").count();

    println!("APEX: {feature_dir_name}");
    println!("  Tasks: {pending} pending, {done} done");
    println!("  Context: {}", context_path.display());
    println!();
    println!("  Open your AI agent and run:");
    println!("  /solidspec-apex {feature_dir_name}");
    println!();
    println!("  Or if the /apex skill is installed:");
    println!(
        "  /apex -a -s implement feature: {}",
        feature_slug(&feature_dir_name)
    );

    Ok(())
}

/// Strip the leading NNN- numeric prefix from a feature dir name.
/// "001-auth-system" → "auth-system"; returns the original if no prefix found.
fn feature_slug(feature_dir_name: &str) -> &str {
    if let Some(pos) = feature_dir_name.find('-') {
        let prefix = &feature_dir_name[..pos];
        if prefix.chars().all(|c| c.is_ascii_digit()) {
            return &feature_dir_name[pos + 1..];
        }
    }
    feature_dir_name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_slug_strips_numeric_prefix() {
        assert_eq!(feature_slug("001-auth-system"), "auth-system");
        assert_eq!(feature_slug("042-user-profile"), "user-profile");
    }

    #[test]
    fn feature_slug_leaves_non_numeric_prefix_intact() {
        assert_eq!(feature_slug("my-feature"), "my-feature");
        assert_eq!(feature_slug("no-number"), "no-number");
    }

    #[test]
    fn feature_slug_leaves_plain_name_intact() {
        assert_eq!(feature_slug("feature"), "feature");
    }

    #[test]
    fn run_fails_without_project_root() {
        let result = run(Some("001-test"), false, false, false);
        // Either no project root found or feature not resolved — both are errors
        assert!(result.is_err());
    }
}
