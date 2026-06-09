use anyhow::{Context, Result};

use crate::config;
use crate::core::{feature, tdd};

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

    let report_path = feature_dir.join("tdd-red-report.md");
    if report_path.exists() {
        println!(
            "tdd-red-report.md already exists for {resolved}. \
             Delete it to regenerate."
        );
        return Ok(());
    }

    let content = tdd::scaffold_red_report(&feature_dir, &resolved)?;

    if dry_run {
        println!("{content}");
        return Ok(());
    }

    let tests_dir = feature_dir.join("tests");
    if !tests_dir.exists() {
        std::fs::create_dir_all(&tests_dir)?;
    }

    std::fs::write(&report_path, &content)?;
    println!("Generated tdd-red-report.md for {resolved}");
    println!(
        "  → Open it and complete the RED phase with your agent: \
         /solidspec-tdd-tests {resolved}"
    );

    Ok(())
}
