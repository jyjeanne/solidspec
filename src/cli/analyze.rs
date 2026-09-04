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

    // Persist the report so DAG-based completion detection
    // (`ArtifactGraph::compute_states`/`first_ready`, consulted by `solidspec
    // status`, the pipeline's "Next:" hint, and `ship`'s analyze/review gate)
    // can see this artifact as done — matches `analysis-report.md` declared
    // in every schema.yaml's `analyze` node, and mirrors `review.rs` writing
    // review-report.md. Without this, analyze never registers as complete
    // even immediately after running, and the "Next" hint loops forever on
    // "analyze" instead of advancing to `ship`.
    let report_path = feature_dir.join("analysis-report.md");
    std::fs::write(&report_path, &output)?;
    println!("Report written to specs/{feature_dir_name}/analysis-report.md\n");

    println!("{output}");

    let coverage_str = report
        .intent_coverage
        .map(|c| format!(" | intent coverage: {c:.0}%"))
        .unwrap_or_default();

    if report.findings.is_empty() {
        println!(
            "All clear — traceability score: {:.0}%{coverage_str}",
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
            "Found {} issues ({} critical, {} high) — traceability: {:.0}%{coverage_str}",
            report.findings.len(),
            critical,
            high,
            report.traceability_score
        );
    }

    Ok(())
}
