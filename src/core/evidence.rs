use std::path::Path;

use anyhow::Result;

use super::intent_parser::{self, IntentStatus};

/// Satisfaction status of a single evidence criterion.
#[derive(Debug, Clone)]
pub struct EvidenceCriterionResult {
    pub text: String,
    pub satisfied: bool,
}

/// Full evidence satisfaction report for a feature.
#[derive(Debug)]
pub struct EvidenceReport {
    pub feature_id: String,
    pub criteria: Vec<EvidenceCriterionResult>,
    pub satisfied_count: usize,
    pub total_count: usize,
    /// 0.0–100.0 percentage of criteria satisfied by implemented tests.
    pub satisfaction_rate: f64,
    /// Derived status based on satisfaction_rate (only meaningful when
    /// `has_implemented_tests` is true).
    pub new_status: IntentStatus,
    /// False at baseline — no test scaffolds have been marked IMPLEMENTED yet.
    pub has_implemented_tests: bool,
}

/// Collect evidence satisfaction for a feature.
///
/// Reads `intent.md` for evidence criteria, scans `tests/` for scaffolds
/// marked `STATUS: IMPLEMENTED`, and cross-references via keyword overlap
/// (identical algorithm to `analyzer::compute_drift`).
pub fn collect_evidence(feature_dir: &Path) -> Result<EvidenceReport> {
    let feature_id = feature_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let intent_path = feature_dir.join("intent.md");
    let intent = intent_parser::parse_intent(&intent_path)?;

    // Collect all test scaffold files (same extensions as compute_drift)
    let tests_dir = feature_dir.join("tests");
    let scaffold_files: Vec<String> = std::fs::read_dir(&tests_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| {
            matches!(
                e.path().extension().and_then(|x| x.to_str()),
                Some("md" | "ts" | "py" | "rs" | "go")
            )
        })
        .filter_map(|e| std::fs::read_to_string(e.path()).ok())
        .collect();

    let any_implemented = scaffold_files
        .iter()
        .any(|f| f.contains("STATUS: IMPLEMENTED") && !f.contains("STATUS: NOT IMPLEMENTED"));

    // Build the lowercased body of implemented tests only
    let implemented_body: String = scaffold_files
        .iter()
        .filter(|f| f.contains("STATUS: IMPLEMENTED") && !f.contains("STATUS: NOT IMPLEMENTED"))
        .cloned()
        .collect::<Vec<_>>()
        .join("\n")
        .to_lowercase();

    let implemented_words: std::collections::HashSet<&str> = implemented_body
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect();

    let mut criteria = Vec::new();

    for criterion in &intent.evidence {
        if criterion.trim().is_empty() {
            continue;
        }

        let satisfied = if !any_implemented {
            // Baseline — no tests implemented yet; nothing is satisfied
            false
        } else {
            let keywords: Vec<String> = criterion
                .split_whitespace()
                .map(|w| {
                    w.trim_matches(|c: char| !c.is_alphanumeric())
                        .to_lowercase()
                })
                .filter(|w| w.len() >= 5)
                .collect();

            if keywords.is_empty() {
                // Short criterion: exact phrase match
                let phrase = criterion.trim().to_lowercase();
                implemented_body.contains(&phrase)
            } else {
                keywords
                    .iter()
                    .any(|kw| implemented_words.contains(kw.as_str()))
            }
        };

        criteria.push(EvidenceCriterionResult {
            text: criterion.clone(),
            satisfied,
        });
    }

    let total_count = criteria.len();
    let satisfied_count = criteria.iter().filter(|c| c.satisfied).count();
    let satisfaction_rate = if total_count > 0 {
        satisfied_count as f64 / total_count as f64 * 100.0
    } else {
        100.0
    };

    let new_status = if !any_implemented {
        intent.status
    } else if satisfaction_rate >= 100.0 {
        IntentStatus::Satisfied
    } else if satisfaction_rate < 70.0 {
        IntentStatus::Drifted
    } else {
        IntentStatus::Active
    };

    Ok(EvidenceReport {
        feature_id,
        criteria,
        satisfied_count,
        total_count,
        satisfaction_rate,
        new_status,
        has_implemented_tests: any_implemented,
    })
}

/// Rewrite the `**Status**: <value>` line in `intent.md` in place.
pub fn update_intent_status(intent_path: &Path, new_status: &IntentStatus) -> Result<()> {
    let content = std::fs::read_to_string(intent_path)?;
    let updated = content
        .lines()
        .map(|line| {
            if line.contains("**Status**:") {
                format!("**Status**: {}", new_status.as_str())
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let updated = if content.ends_with('\n') {
        format!("{updated}\n")
    } else {
        updated
    };
    std::fs::write(intent_path, updated)?;
    Ok(())
}

/// Format the evidence report as Markdown (written to `evidence-report.md`).
pub fn format_evidence_report(report: &EvidenceReport) -> String {
    let mut out = format!(
        "# Evidence Report: {}\n\n**Date**: {}\n**Satisfaction Rate**: {:.0}%\n",
        report.feature_id,
        chrono::Local::now().format("%Y-%m-%d"),
        report.satisfaction_rate,
    );

    if report.has_implemented_tests {
        out.push_str(&format!("**Status**: {}\n\n", report.new_status.as_str()));
    } else {
        out.push_str("**Status**: baseline (no tests implemented yet)\n\n");
    }

    out.push_str("## Criteria\n\n");
    out.push_str("| # | Criterion | Status |\n");
    out.push_str("|---|-----------|--------|\n");
    for (i, c) in report.criteria.iter().enumerate() {
        let status = if !report.has_implemented_tests {
            "⏳ baseline"
        } else if c.satisfied {
            "✓ Satisfied"
        } else {
            "✗ Not satisfied"
        };
        out.push_str(&format!("| {} | {} | {} |\n", i + 1, c.text, status));
    }

    if report.criteria.is_empty() {
        out.push_str("*No evidence criteria defined in intent.md.*\n");
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    const SAMPLE_INTENT: &str = r#"# Intent: Auth System

**Intent ID**: INT-001
**Feature**: 001-auth
**Created**: 2026-06-01
**Status**: active

## Goal
Allow users to authenticate securely.

## Evidence
- Users can authenticate with valid credentials
- Password reset email is delivered
- Session is created after login
"#;

    fn write_intent(dir: &Path, content: &str) {
        std::fs::write(dir.join("intent.md"), content).unwrap();
    }

    fn write_test_file(dir: &Path, name: &str, status: &str, body: &str) {
        let tests = dir.join("tests");
        std::fs::create_dir_all(&tests).unwrap();
        let content = format!("{body}\nSTATUS: {status}\n");
        std::fs::write(tests.join(name), content).unwrap();
    }

    #[test]
    fn baseline_all_not_implemented() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();
        write_intent(&feature, SAMPLE_INTENT);
        write_test_file(
            &feature,
            "test1.md",
            "NOT IMPLEMENTED",
            "GIVEN: user\nWHEN: logs in\nTHEN: session created",
        );

        let report = collect_evidence(&feature).unwrap();
        assert!(
            !report.has_implemented_tests,
            "Baseline: no IMPLEMENTED tests"
        );
        assert_eq!(report.satisfied_count, 0);
        assert_eq!(report.total_count, 3);
        assert!(report.criteria.iter().all(|c| !c.satisfied));
    }

    #[test]
    fn satisfied_criterion_detected() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();
        write_intent(&feature, SAMPLE_INTENT);
        // Test body mentions "authenticate" and "credentials" — covers criterion 1
        write_test_file(
            &feature,
            "test1.md",
            "IMPLEMENTED",
            "GIVEN: valid credentials\nWHEN: authenticate\nTHEN: success",
        );

        let report = collect_evidence(&feature).unwrap();
        assert!(report.has_implemented_tests);
        assert!(
            report.criteria[0].satisfied,
            "Criterion 1 should be satisfied"
        );
    }

    #[test]
    fn satisfaction_rate_100_gives_satisfied_status() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();
        write_intent(&feature, SAMPLE_INTENT);
        // Single test that covers all three criteria keywords
        write_test_file(
            &feature,
            "test1.md",
            "IMPLEMENTED",
            "authenticate credentials\npassword reset email delivered\nsession created login",
        );

        let report = collect_evidence(&feature).unwrap();
        assert_eq!(report.new_status, IntentStatus::Satisfied);
        assert_eq!(report.satisfaction_rate, 100.0);
    }

    #[test]
    fn low_satisfaction_gives_drifted_status() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();

        let intent = "# Intent: Many Criteria\n\n\
                      **Intent ID**: INT-001\n**Feature**: 001-auth\n\
                      **Created**: 2026-01-01\n**Status**: active\n\n\
                      ## Goal\nTest.\n\n\
                      ## Evidence\n\
                      - Alpha criterion one\n\
                      - Bravo criterion two\n\
                      - Charlie criterion three\n\
                      - Delta criterion four\n\
                      - Echo criterion five\n";
        write_intent(&feature, intent);
        // Implemented test only covers "alpha" — 1/5 = 20% < 70%
        write_test_file(&feature, "test1.md", "IMPLEMENTED", "alpha testing done");

        let report = collect_evidence(&feature).unwrap();
        assert_eq!(report.new_status, IntentStatus::Drifted);
        assert!(report.satisfaction_rate < 70.0);
    }

    #[test]
    fn no_tests_dir_returns_baseline() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();
        write_intent(&feature, SAMPLE_INTENT);
        // No tests/ directory at all

        let report = collect_evidence(&feature).unwrap();
        assert!(!report.has_implemented_tests);
        assert_eq!(report.satisfied_count, 0);
    }

    #[test]
    fn update_intent_status_rewrites_status_line() {
        let dir = TempDir::new().unwrap();
        let intent_path = dir.path().join("intent.md");
        std::fs::write(&intent_path, SAMPLE_INTENT).unwrap();

        update_intent_status(&intent_path, &IntentStatus::Satisfied).unwrap();

        let updated = std::fs::read_to_string(&intent_path).unwrap();
        assert!(updated.contains("**Status**: satisfied"));
        assert!(!updated.contains("**Status**: active"));
        // Other content must be preserved
        assert!(updated.contains("INT-001"));
        assert!(updated.contains("Users can authenticate"));
    }

    #[test]
    fn update_intent_status_preserves_trailing_newline() {
        let dir = TempDir::new().unwrap();
        let intent_path = dir.path().join("intent.md");
        std::fs::write(&intent_path, SAMPLE_INTENT).unwrap();
        assert!(SAMPLE_INTENT.ends_with('\n'));

        update_intent_status(&intent_path, &IntentStatus::Drifted).unwrap();

        let updated = std::fs::read_to_string(&intent_path).unwrap();
        assert!(
            updated.ends_with('\n'),
            "Trailing newline must be preserved"
        );
    }

    #[test]
    fn format_report_contains_table_and_header() {
        let report = EvidenceReport {
            feature_id: "001-auth".into(),
            criteria: vec![
                EvidenceCriterionResult {
                    text: "Users can authenticate".into(),
                    satisfied: true,
                },
                EvidenceCriterionResult {
                    text: "Password reset works".into(),
                    satisfied: false,
                },
            ],
            satisfied_count: 1,
            total_count: 2,
            satisfaction_rate: 50.0,
            new_status: IntentStatus::Drifted,
            has_implemented_tests: true,
        };

        let md = format_evidence_report(&report);
        assert!(md.contains("# Evidence Report: 001-auth"));
        assert!(md.contains("50%"));
        assert!(md.contains("✓ Satisfied"));
        assert!(md.contains("✗ Not satisfied"));
        assert!(md.contains("## Criteria"));
    }
}
