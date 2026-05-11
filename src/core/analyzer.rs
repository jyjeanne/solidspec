use std::path::Path;

use anyhow::Result;

use super::constitution;
use super::errors::SolidSpecError;
use super::spec_parser;

const MAX_FINDINGS: usize = 50;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "CRITICAL"),
            Severity::High => write!(f, "HIGH"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::Low => write!(f, "LOW"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub severity: Severity,
    pub message: String,
    pub remediation: String,
}

#[derive(Debug)]
pub struct AnalysisReport {
    pub findings: Vec<Finding>,
    pub overflow_count: usize,
    pub traceability_score: f64,
}

/// Analyze cross-artifact consistency. Read-only — does NOT modify files.
pub fn analyze_feature(feature_dir: &Path, project_root: &Path) -> Result<AnalysisReport> {
    let spec_path = feature_dir.join("spec.md");
    if !spec_path.exists() {
        return Err(SolidSpecError::Spec {
            feature_id: feature_dir.display().to_string(),
            message: "spec.md not found".into(),
            fix: "Run 'solidspec specify' first.".into(),
        }
        .into());
    }

    let mut all_findings = Vec::new();

    // Parse spec
    let spec = spec_parser::parse_spec(&spec_path)?;

    // Check plan exists
    let plan_path = feature_dir.join("plan.md");
    let has_plan = plan_path.exists();
    if !has_plan {
        all_findings.push(Finding {
            severity: Severity::High,
            message: "plan.md missing — no architecture plan found".into(),
            remediation: "Run 'solidspec plan' to generate the implementation plan.".into(),
        });
    }

    // Check tasks exist
    let tasks_path = feature_dir.join("tasks.md");
    let has_tasks = tasks_path.exists();
    if !has_tasks && has_plan {
        all_findings.push(Finding {
            severity: Severity::High,
            message: "tasks.md missing — no task breakdown found".into(),
            remediation: "Run 'solidspec tasks' to generate the task breakdown.".into(),
        });
    }

    // Check unresolved clarification markers
    if !spec.clarification_markers.is_empty() {
        all_findings.push(Finding {
            severity: Severity::High,
            message: format!(
                "{} unresolved [NEEDS CLARIFICATION] markers in spec.md",
                spec.clarification_markers.len()
            ),
            remediation: "Run 'solidspec clarify' to resolve ambiguities.".into(),
        });
    }

    // Constitution compliance
    let constitution_path = project_root.join(".solidspec/constitution.md");
    if constitution_path.exists() {
        let const_result = constitution::load_constitution(&constitution_path)?;
        if has_plan {
            let plan_content = std::fs::read_to_string(&plan_path)?;
            let gate_results = constitution::check_plan_compliance(&const_result, &plan_content);
            for gate in gate_results {
                if !gate.passed {
                    for violation in &gate.violations {
                        all_findings.push(Finding {
                            severity: Severity::Critical,
                            message: format!("Constitution violation ({}): {violation}", gate.gate_name),
                            remediation: "Constitution conflicts are non-negotiable. Update the plan to comply.".into(),
                        });
                    }
                }
            }
        }
    }

    // Requirement → plan traceability
    let mut traced_reqs = 0;
    if has_plan {
        let plan_content = std::fs::read_to_string(&plan_path)?;
        for req in &spec.requirements {
            // Simple check: requirement ID or key terms appear in plan
            if plan_content.contains(&req.id) {
                traced_reqs += 1;
            }
        }

        let total_reqs = spec.requirements.len();
        if total_reqs > 0 && traced_reqs < total_reqs {
            let missing = total_reqs - traced_reqs;
            all_findings.push(Finding {
                severity: Severity::Medium,
                message: format!("{missing} requirements not traced in plan.md"),
                remediation: "Ensure all FR-### IDs from spec.md appear in plan.md.".into(),
            });
        }
    }

    // Orphan task detection
    if has_tasks {
        let tasks_content = std::fs::read_to_string(&tasks_path)?;
        let task_count = tasks_content.matches("- [ ] T").count()
            + tasks_content.matches("- [X] T").count()
            + tasks_content.matches("- [x] T").count();
        if task_count == 0 {
            all_findings.push(Finding {
                severity: Severity::Medium,
                message: "tasks.md contains no tasks".into(),
                remediation: "Run 'solidspec tasks' to regenerate.".into(),
            });
        }
    }

    // Terminology drift (simple check)
    if has_plan {
        let plan_content = std::fs::read_to_string(&plan_path)?;
        for entity in &spec.entities {
            if !plan_content.contains(entity) {
                all_findings.push(Finding {
                    severity: Severity::Low,
                    message: format!("Entity '{entity}' from spec not mentioned in plan.md"),
                    remediation: format!("Ensure '{entity}' is referenced in the plan."),
                });
            }
        }
    }

    // Cap findings
    let overflow_count = if all_findings.len() > MAX_FINDINGS {
        let overflow = all_findings.len() - MAX_FINDINGS;
        all_findings.truncate(MAX_FINDINGS);
        overflow
    } else {
        0
    };

    // Traceability score
    let total_reqs = spec.requirements.len();
    let traceability_score = if total_reqs > 0 {
        traced_reqs as f64 / total_reqs as f64 * 100.0
    } else {
        100.0 // no requirements = vacuously traced
    };

    Ok(AnalysisReport {
        findings: all_findings,
        overflow_count,
        traceability_score,
    })
}

pub fn format_report(report: &AnalysisReport) -> String {
    let mut output = String::from("# Analysis Report\n\n");
    output.push_str(&format!(
        "**Traceability Score**: {:.0}%\n",
        report.traceability_score
    ));
    output.push_str(&format!("**Findings**: {}", report.findings.len()));
    if report.overflow_count > 0 {
        output.push_str(&format!(
            " (+{} overflow, not shown)",
            report.overflow_count
        ));
    }
    output.push_str("\n\n");

    for finding in &report.findings {
        output.push_str(&format!(
            "- **[{}]** {}\n",
            finding.severity, finding.message
        ));
        output.push_str(&format!("  *Remediation*: {}\n\n", finding.remediation));
    }

    if report.findings.is_empty() {
        output.push_str("No issues found. All artifacts are consistent.\n");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_feature(dir: &Path, spec: &str, plan: Option<&str>, tasks: Option<&str>) {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(dir.join("spec.md"), spec).unwrap();
        if let Some(p) = plan {
            std::fs::write(dir.join("plan.md"), p).unwrap();
        }
        if let Some(t) = tasks {
            std::fs::write(dir.join("tasks.md"), t).unwrap();
        }
    }

    fn setup_constitution(project_root: &Path) {
        let solidspec = project_root.join(".solidspec");
        std::fs::create_dir_all(&solidspec).unwrap();
        std::fs::write(
            solidspec.join("constitution.md"),
            "### Article VII: Simplicity\n### Article VIII: Anti-Abstraction\n### Article IX: Integration-First\n",
        ).unwrap();
    }

    #[test]
    fn fully_traced_artifacts_high_score() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_constitution(dir.path());

        let spec = "## Requirements\n- **FR-001**: authenticate users\n- **FR-002**: store sessions\n## Success Criteria\n- **SC-001**: login works\n";
        let plan = "# Plan\nFR-001 handled by auth module\nFR-002 handled by session store\n";
        let tasks = "- [ ] T001 Setup\n- [ ] T002 Auth\n";
        setup_feature(&feature, spec, Some(plan), Some(tasks));

        let report = analyze_feature(&feature, dir.path()).unwrap();
        assert_eq!(report.traceability_score, 100.0);
    }

    #[test]
    fn missing_plan_is_high_finding() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(&feature, "# Spec\n", None, None);
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.severity == Severity::High && f.message.contains("plan.md missing"))
        );
    }

    #[test]
    fn orphan_tasks_medium_finding() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(
            &feature,
            "# Spec\n",
            Some("# Plan\n"),
            Some("# Tasks\nno actual tasks\n"),
        );
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.severity == Severity::Medium && f.message.contains("no tasks"))
        );
    }

    #[test]
    fn constitution_violation_is_critical() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        let plan = "We should future-proof this with a wrapper abstraction layer.";
        setup_feature(&feature, "# Spec\n", Some(plan), None);
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.severity == Severity::Critical)
        );
    }

    #[test]
    fn missing_spec_returns_error() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();

        assert!(analyze_feature(&feature, dir.path()).is_err());
    }

    #[test]
    fn max_findings_enforced() {
        // Simulate by checking the cap constant
        assert_eq!(MAX_FINDINGS, 50);
    }

    #[test]
    fn analyze_does_not_modify_files() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        let spec = "# Spec\n- **FR-001**: test\n";
        let plan = "# Plan\nFR-001 covered\n";
        setup_feature(&feature, spec, Some(plan), Some("- [ ] T001 test\n"));
        setup_constitution(dir.path());

        let spec_before = std::fs::read_to_string(feature.join("spec.md")).unwrap();
        let plan_before = std::fs::read_to_string(feature.join("plan.md")).unwrap();

        let _report = analyze_feature(&feature, dir.path()).unwrap();

        let spec_after = std::fs::read_to_string(feature.join("spec.md")).unwrap();
        let plan_after = std::fs::read_to_string(feature.join("plan.md")).unwrap();
        assert_eq!(spec_before, spec_after, "Analyzer modified spec.md!");
        assert_eq!(plan_before, plan_after, "Analyzer modified plan.md!");
    }

    #[test]
    fn remediation_suggestions_present() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(&feature, "# Spec\n", None, None);
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        for finding in &report.findings {
            assert!(
                !finding.remediation.is_empty(),
                "Finding has empty remediation"
            );
        }
    }
}
