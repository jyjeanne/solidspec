//! Ship report rendering — turns a `ShipReport` into the Markdown file
//! written to `ship-report.md`.

use super::{FanOutFinding, LaneStatus, ShipDecision, ShipReport};

/// Render a `ShipReport` as Markdown with a machine-readable `<!-- ship: bool -->` header.
pub fn format_ship_report(report: &ShipReport) -> String {
    let is_ship = report.decision == ShipDecision::Ship;
    let decision_str = if is_ship { "SHIP" } else { "HOLD" };
    let generated = chrono::Utc::now().to_rfc3339();

    let mut out = format!(
        "# Ship Report: {feature}\n\n\
         <!-- ship: {is_ship} -->\n\
         <!-- generated: {generated} -->\n\n\
         **Decision**: {decision_str}\n\n\
         ## Lane Scores\n\n\
         | Lane | Agent | Score | Threshold | Status |\n\
         |------|-------|-------|-----------|--------|\n",
        feature = report.feature_id,
    );

    for result in &report.lanes {
        let status_cell = match &result.status {
            LaneStatus::Done if result.score >= result.threshold => "✓ Pass",
            LaneStatus::Done => "✗ Fail",
            LaneStatus::TimedOut => "⏱ Timed Out",
            LaneStatus::Failed(_) => "✗ Failed",
        };
        out.push_str(&format!(
            "| {} | {} | {}/100 | {} | {} |\n",
            result.lane_label, result.agent_id, result.score, result.threshold, status_cell
        ));
    }

    if !report.blocking_findings.is_empty() {
        out.push_str("\n## Blocking Findings\n");

        // Group findings by lane for readable output.
        let mut by_lane: std::collections::BTreeMap<&str, Vec<&FanOutFinding>> =
            std::collections::BTreeMap::new();
        for f in &report.blocking_findings {
            by_lane.entry(f.lane).or_default().push(f);
        }
        for (lane_id, findings) in &by_lane {
            let label = report
                .lanes
                .iter()
                .find(|r| r.lane_id == *lane_id)
                .map(|r| r.lane_label)
                .unwrap_or(lane_id);
            out.push_str(&format!("\n### {label}\n\n"));
            for f in findings {
                out.push_str(&format!(
                    "- **[{}]** {}\n  *Fix*: {}\n\n",
                    f.severity, f.message, f.remediation
                ));
            }
        }
    }

    out.push_str(&format!(
        "\n## Re-run\n\n```bash\nsolidspec ship {}\n```\n",
        report.feature_id
    ));

    out
}
