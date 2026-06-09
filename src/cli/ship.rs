use anyhow::{Context, Result};

use crate::config;
use crate::core::fan_out::{self, ShipDecision};
use crate::core::feature;

const VALID_LANES: &[&str] = &["code", "security", "tests", "perf"];

#[allow(clippy::too_many_arguments)]
pub fn run(
    feature_id: Option<&str>,
    lane_filter: Vec<String>,
    fail_on_hold: bool,
    code_agent: Option<String>,
    tests_agent: Option<String>,
    security_agent: Option<String>,
    perf_agent: Option<String>,
    no_agent: bool,
    dry_run: bool,
    timeout: u64,
    ignore_timeout: bool,
) -> Result<()> {
    // Validate --lane values before doing any work.
    for lane in &lane_filter {
        if !VALID_LANES.contains(&lane.as_str()) {
            anyhow::bail!(
                "Unknown lane '{}'. Valid lanes: {}",
                lane,
                VALID_LANES.join(", ")
            );
        }
    }

    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);

    if !feature_dir.join("spec.md").exists() {
        anyhow::bail!(
            "No spec.md found for feature '{}'. Run 'solidspec specify' first.",
            feature_dir_name
        );
    }

    let mut cfg = match config::RootConfig::load(&project_root.join("solidspec.toml")) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Warning: could not load solidspec.toml ({e}), using defaults.");
            config::RootConfig::new(&feature_dir_name)
        }
    };

    // Apply CLI agent overrides on top of config.
    if let Some(a) = code_agent {
        cfg.fan_out.code_agent = Some(a);
    }
    if let Some(a) = security_agent {
        cfg.fan_out.security_agent = Some(a);
    }
    if let Some(a) = tests_agent {
        cfg.fan_out.tests_agent = Some(a);
    }
    if let Some(a) = perf_agent {
        cfg.fan_out.perf_agent = Some(a);
    }

    let default_agent = cfg.ai.default_agent.clone();
    let mut lanes = fan_out::build_lanes(&cfg.fan_out, &feature_dir, &default_agent);

    // Apply --lane filter.
    if !lane_filter.is_empty() {
        lanes.retain(|l| lane_filter.iter().any(|f| f == l.id));
    }

    if dry_run {
        println!("Ship Assessment (dry run): {feature_dir_name}\n");
        println!("{:<18} {:<12} Threshold", "Lane", "Agent");
        println!("{}", "-".repeat(44));
        for l in &lanes {
            println!("{:<18} {:<12} {}", l.label, l.agent_id, l.threshold);
        }
        println!("\nNo files created (dry run).");
        return Ok(());
    }

    println!("Ship Assessment: {feature_dir_name}\n");
    println!("Launching {} review lanes (concurrent)...", lanes.len());

    let results = fan_out::run_fan_out(lanes, feature_dir.clone(), project_root, no_agent, timeout);

    let report = fan_out::aggregate_results(
        results,
        &feature_dir_name,
        cfg.fan_out.block_on_critical,
        ignore_timeout,
    );

    // Print summary separator and decision.
    println!("\n{}", "━".repeat(40));
    let decision_str = if report.decision == ShipDecision::Ship {
        "  Ship Decision: SHIP ✓"
    } else {
        "  Ship Decision: HOLD ✗"
    };
    println!("{decision_str}");

    if !report.blocking_findings.is_empty() {
        println!(
            "\n  Blocking issues ({} findings):",
            report.blocking_findings.len()
        );
        for f in &report.blocking_findings {
            println!(
                "  [{}/{}]   {}",
                f.lane.to_uppercase(),
                f.severity,
                f.message
            );
        }
    }
    println!("{}", "━".repeat(40));

    // Write ship-report.md.
    let markdown = fan_out::format_ship_report(&report);
    let report_path = feature_dir.join("ship-report.md");
    std::fs::write(&report_path, &markdown)?;
    println!("\nReport: specs/{}/ship-report.md", feature_dir_name);

    if !lane_filter.is_empty() {
        let re_run_lanes = lane_filter.join(",");
        println!("Re-run: solidspec ship {feature_dir_name} --lane {re_run_lanes}");
    } else {
        println!("Re-run: solidspec ship {feature_dir_name}");
    }

    if fail_on_hold && report.decision == ShipDecision::Hold {
        std::process::exit(1);
    }

    Ok(())
}
