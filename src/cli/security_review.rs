use anyhow::{Context, Result};

use crate::config;
use crate::core::{feature, security_review};

pub fn run(feature_id: Option<&str>, dry_run: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let resolved = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&resolved);

    if !feature_dir.exists() {
        anyhow::bail!(
            "Feature directory not found: {}. Run 'solidspec specify' first.",
            feature_dir.display()
        );
    }

    let report_path = feature_dir.join("security-review.md");
    if report_path.exists() && !dry_run {
        println!("security-review.md already exists for {resolved}. Delete it to regenerate.");
        return Ok(());
    }

    let report = security_review::run_security_review(&feature_dir)?;
    let content = security_review::format_security_review(&report);

    println!(
        "Security review: {resolved} — {} finding(s)",
        report.findings.len()
    );

    if dry_run {
        println!("{content}");
        return Ok(());
    }

    std::fs::write(&report_path, &content)?;
    println!("  Created security-review.md");
    if report.findings.is_empty() {
        println!("  Run 'solidspec tasks {resolved}' next.");
    } else {
        println!(
            "  Every finding must have a mitigation task before running \
             'solidspec tasks {resolved}'."
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn init(dir: &std::path::Path) {
        std::fs::create_dir_all(dir.join(".solidspec")).unwrap();
        std::fs::write(dir.join("solidspec.toml"), "[project]\nname = \"t\"\n").unwrap();
    }

    #[test]
    fn fails_outside_project() {
        let dir = TempDir::new().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();
        let err = run(Some("001"), false).unwrap_err();
        assert!(err.to_string().contains("Not inside a SolidSpec project"));
    }

    #[test]
    fn fails_when_feature_dir_missing() {
        // An existing but non-matching specs/ dir reaches the explicit
        // `feature_dir.exists()` check in `run` (an empty specs/ dir, as
        // opposed to a missing one, makes `feature::resolve_feature` return
        // the requested id unresolved rather than erroring earlier).
        let dir = TempDir::new().unwrap();
        init(dir.path());
        std::fs::create_dir_all(dir.path().join("specs/001-other")).unwrap();
        std::env::set_current_dir(dir.path()).unwrap();
        let err = run(Some("999-missing"), false).unwrap_err();
        assert!(err.to_string().contains("not found") || err.to_string().contains("No feature"));
    }

    #[test]
    fn writes_report_when_plan_exists() {
        let dir = TempDir::new().unwrap();
        init(dir.path());
        let feature_dir = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature_dir).unwrap();
        std::fs::write(feature_dir.join("spec.md"), "# Spec\nStatic page.\n").unwrap();
        std::fs::write(feature_dir.join("plan.md"), "# Plan\nServe static HTML.\n").unwrap();

        std::env::set_current_dir(dir.path()).unwrap();
        run(Some("001-test"), false).unwrap();

        assert!(feature_dir.join("security-review.md").exists());
    }

    #[test]
    fn dry_run_does_not_write_file() {
        let dir = TempDir::new().unwrap();
        init(dir.path());
        let feature_dir = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature_dir).unwrap();
        std::fs::write(feature_dir.join("spec.md"), "# Spec\nStatic page.\n").unwrap();
        std::fs::write(feature_dir.join("plan.md"), "# Plan\nServe static HTML.\n").unwrap();

        std::env::set_current_dir(dir.path()).unwrap();
        run(Some("001-test"), true).unwrap();

        assert!(!feature_dir.join("security-review.md").exists());
    }
}
