use anyhow::{Context, Result};

use crate::config;
use crate::core::{evidence, feature};

pub fn run(feature_id: Option<&str>, update: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);

    // intent.md is required for evidence collection
    if !feature_dir.join("intent.md").exists() {
        anyhow::bail!(
            "intent.md not found in {feature_dir_name}. \
             Run 'solidspec intent' to capture intent first."
        );
    }

    println!("Collecting evidence: {feature_dir_name}");

    let report = evidence::collect_evidence(&feature_dir)?;

    // Print per-criterion table to stdout
    println!(
        "\nSatisfaction: {}/{} criteria ({:.0}%)\n",
        report.satisfied_count, report.total_count, report.satisfaction_rate
    );

    for (i, criterion) in report.criteria.iter().enumerate() {
        let status = if !report.has_implemented_tests {
            "⏳"
        } else if criterion.satisfied {
            "✓"
        } else {
            "✗"
        };
        println!("  {} {}. {}", status, i + 1, criterion.text);
    }

    if !report.has_implemented_tests {
        println!(
            "\nBaseline — no test scaffolds marked STATUS: IMPLEMENTED yet.\n\
             Implement tests, then re-run 'solidspec evidence' to measure satisfaction."
        );
    }

    // Write evidence-report.md
    let report_content = evidence::format_evidence_report(&report);
    let report_path = feature_dir.join("evidence-report.md");
    std::fs::write(&report_path, &report_content)?;
    println!("\nReport written to specs/{feature_dir_name}/evidence-report.md");

    // Auto-update intent.md Status field when --update is passed
    if update && report.has_implemented_tests {
        let intent_path = feature_dir.join("intent.md");
        evidence::update_intent_status(&intent_path, &report.new_status)?;
        println!("Updated intent.md Status → {}", report.new_status.as_str());
    } else if update {
        println!("--update skipped: no implemented tests yet (status unchanged).");
    }

    Ok(())
}
