use anyhow::{Context, Result};

use crate::config;
use crate::core::{feature, review};

pub fn run(feature_id: Option<&str>) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);

    println!("Reviewing: {feature_dir_name}");

    let report = review::preflight_review(&feature_dir, &project_root)?;
    let output = review::format_review_report(&report);

    // Write report to feature directory
    let report_path = feature_dir.join("review-report.md");
    std::fs::write(&report_path, &output)?;
    println!("Report written to specs/{feature_dir_name}/review-report.md\n");

    println!("{output}");

    let critical = report
        .findings
        .iter()
        .filter(|f| f.severity == review::Severity::Critical)
        .count();
    let high = report
        .findings
        .iter()
        .filter(|f| f.severity == review::Severity::High)
        .count();

    if report.findings.is_empty() {
        println!("All clear — overall score: {:.0}%", report.overall_score);
    } else {
        println!(
            "Found {} issues ({} critical, {} high) — score: {:.0}%",
            report.findings.len(),
            critical,
            high,
            report.overall_score
        );
    }

    Ok(())
}
