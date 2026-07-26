//! Renders a `ReviewReport` as the Markdown written to `review-report.md`.

use super::{Dimension, ReviewReport, Severity};

/// Format a review report as Markdown.
pub fn format_review_report(report: &ReviewReport) -> String {
    let mut out = format!("# Review Report: {}\n\n", report.feature_id);

    // Overall score
    let rounded = report.overall_score.round() as u32;
    let grade = match rounded {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    out.push_str(&format!(
        "**Overall Score**: {:.0}% (Grade: {grade})\n\n",
        report.overall_score
    ));

    // Dimension table
    out.push_str("## Dimension Scores\n\n");
    out.push_str("| Dimension | Score | Findings |\n");
    out.push_str("|-----------|-------|----------|\n");
    for ds in &report.dimension_scores {
        out.push_str(&format!(
            "| {} | {:.0}/{:.0} | {} |\n",
            ds.dimension, ds.score, ds.max_score, ds.finding_count
        ));
    }
    out.push('\n');

    // Intent Alignment section
    out.push_str("## Intent Alignment\n\n");
    if let Some(ia) = report
        .dimension_scores
        .iter()
        .find(|ds| ds.dimension == Dimension::IntentAlignment)
    {
        out.push_str(&format!(
            "**Score**: {:.0}/{:.0}\n\n",
            ia.score, ia.max_score
        ));
        if ia.finding_count == 0 && ia.score == 0.0 {
            out.push_str(
                "`intent.md` not found — IDSD is not active for this feature.\n\
                 Run `solidspec intent \"<title>\"` to capture intent and enable the IDSD workflow.\n",
            );
        } else if ia.finding_count == 0 {
            out.push_str(
                "Intent status is valid and all functional requirements \
                 are traced to evidence criteria.\n",
            );
        } else {
            let ia_findings: Vec<_> = report
                .findings
                .iter()
                .filter(|f| f.dimension == Dimension::IntentAlignment)
                .collect();
            for finding in ia_findings {
                let loc = finding
                    .location
                    .as_deref()
                    .map(|l| format!(" ({l})"))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "- **[{}]**{loc} {}\n  *Fix*: {}\n\n",
                    finding.severity, finding.message, finding.remediation
                ));
            }
        }
    }
    out.push('\n');

    // Findings by severity (excludes IntentAlignment — shown in its own section above)
    let non_ia_count = report
        .findings
        .iter()
        .filter(|f| f.dimension != Dimension::IntentAlignment)
        .count();
    out.push_str(&format!("## Findings ({})", non_ia_count));
    if report.overflow_count > 0 {
        out.push_str(&format!(" (+{} not shown)", report.overflow_count));
    }
    out.push_str("\n\n");

    let severity_order = [
        Severity::Critical,
        Severity::High,
        Severity::Medium,
        Severity::Low,
        Severity::Info,
    ];

    for sev in &severity_order {
        let sev_findings: Vec<_> = report
            .findings
            .iter()
            .filter(|f| &f.severity == sev && f.dimension != Dimension::IntentAlignment)
            .collect();
        if sev_findings.is_empty() {
            continue;
        }

        out.push_str(&format!("### {sev}\n\n"));
        for finding in sev_findings {
            let loc = finding
                .location
                .as_deref()
                .map(|l| format!(" ({l})"))
                .unwrap_or_default();
            out.push_str(&format!(
                "- **[{}]**{loc} {}\n",
                finding.dimension, finding.message
            ));
            out.push_str(&format!("  *Fix*: {}\n\n", finding.remediation));
        }
    }

    if report.findings.is_empty() {
        out.push_str("No issues found. Spec quality looks good!\n");
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::review::{DimensionScore, ReviewFinding};

    #[test]
    fn format_report_renders_markdown() {
        let report = ReviewReport {
            feature_id: "001-test".into(),
            findings: vec![ReviewFinding {
                dimension: Dimension::Completeness,
                severity: Severity::High,
                message: "Missing plan".into(),
                remediation: "Run plan command".into(),
                location: None,
            }],
            dimension_scores: vec![DimensionScore {
                dimension: Dimension::Completeness,
                score: 7.0,
                max_score: 10.0,
                finding_count: 1,
            }],
            overall_score: 70.0,
            overflow_count: 0,
        };

        let md = format_review_report(&report);
        assert!(md.contains("# Review Report"));
        assert!(md.contains("70%"));
        assert!(md.contains("Missing plan"));
    }
}
