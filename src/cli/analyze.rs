use anyhow::{Context, Result};

use crate::config;
use crate::core::{analyzer, feature};

pub fn run(feature_id: Option<&str>) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);

    println!("Analyzing: {feature_dir_name}");

    let report = analyzer::analyze_feature(&feature_dir, &project_root)?;
    let output = analyzer::format_report(&report);

    println!("{output}");

    if report.findings.is_empty() {
        println!(
            "All clear — traceability score: {:.0}%",
            report.traceability_score
        );
    } else {
        let critical = report
            .findings
            .iter()
            .filter(|f| f.severity == analyzer::Severity::Critical)
            .count();
        let high = report
            .findings
            .iter()
            .filter(|f| f.severity == analyzer::Severity::High)
            .count();
        println!(
            "Found {} issues ({} critical, {} high) — traceability: {:.0}%",
            report.findings.len(),
            critical,
            high,
            report.traceability_score
        );
    }

    Ok(())
}
