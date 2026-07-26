use std::path::Path;

use anyhow::Result;

use super::errors::SolidSpecError;
use super::spec_parser;

mod checks;
mod report;
pub use report::format_review_report;

const MAX_FINDINGS: usize = 100;

/// Review dimension categories (inspired by ai-spec-review-skill).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dimension {
    Completeness,
    Clarity,
    Testability,
    Consistency,
    Security,
    Performance,
    Maintainability,
    /// IDSD only — scores 0/10 when `intent.md` is absent.
    IntentAlignment,
}

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dimension::Completeness => write!(f, "Completeness"),
            Dimension::Clarity => write!(f, "Clarity"),
            Dimension::Testability => write!(f, "Testability"),
            Dimension::Consistency => write!(f, "Consistency"),
            Dimension::Security => write!(f, "Security"),
            Dimension::Performance => write!(f, "Performance"),
            Dimension::Maintainability => write!(f, "Maintainability"),
            Dimension::IntentAlignment => write!(f, "IntentAlignment"),
        }
    }
}

/// Severity level for review findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "CRITICAL"),
            Severity::High => write!(f, "HIGH"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::Low => write!(f, "LOW"),
            Severity::Info => write!(f, "INFO"),
        }
    }
}

/// A single review finding.
#[derive(Debug, Clone)]
pub struct ReviewFinding {
    pub dimension: Dimension,
    pub severity: Severity,
    pub message: String,
    pub remediation: String,
    pub location: Option<String>,
}

/// Score for a single dimension.
#[derive(Debug, Clone)]
pub struct DimensionScore {
    pub dimension: Dimension,
    pub score: f64,
    pub max_score: f64,
    pub finding_count: usize,
}

/// Full review report.
#[derive(Debug)]
pub struct ReviewReport {
    pub feature_id: String,
    pub findings: Vec<ReviewFinding>,
    pub dimension_scores: Vec<DimensionScore>,
    pub overall_score: f64,
    pub overflow_count: usize,
}

/// Run preflight review heuristics on a feature directory. Read-only — no file modifications.
pub fn preflight_review(feature_dir: &Path, _project_root: &Path) -> Result<ReviewReport> {
    let feature_id = feature_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let spec_path = feature_dir.join("spec.md");
    if !spec_path.exists() {
        return Err(SolidSpecError::Spec {
            feature_id,
            message: "spec.md not found".into(),
            fix: "Run 'solidspec specify' first.".into(),
        }
        .into());
    }

    let spec_content = std::fs::read_to_string(&spec_path)?;
    let spec = spec_parser::parse_spec(&spec_path)?;

    let mut findings = Vec::new();

    // 1. Placeholder detection
    findings.extend(checks::check_placeholders(&spec_content, "spec.md"));

    // 2. Section completeness
    findings.extend(checks::check_section_completeness(&spec_content));

    // 3. Ambiguous language
    findings.extend(checks::check_ambiguous_language(&spec_content));

    // 4. Requirement quality
    findings.extend(checks::check_requirement_quality(&spec));

    // 5. Acceptance scenario coverage
    findings.extend(checks::check_scenario_coverage(&spec));

    // 6. Cross-artifact checks
    let plan_path = feature_dir.join("plan.md");
    let tasks_path = feature_dir.join("tasks.md");
    let plan_content = if plan_path.exists() {
        let content = std::fs::read_to_string(&plan_path)?;
        findings.extend(checks::check_placeholders(&content, "plan.md"));
        findings.extend(checks::check_cross_references(&spec, &content, "plan.md"));
        Some(content)
    } else {
        findings.push(ReviewFinding {
            dimension: Dimension::Completeness,
            severity: Severity::High,
            message: "plan.md missing — no architecture plan".into(),
            remediation: "Run 'solidspec plan' to generate the plan.".into(),
            location: None,
        });
        None
    };

    if tasks_path.exists() {
        let tasks_content = std::fs::read_to_string(&tasks_path)?;
        findings.extend(checks::check_placeholders(&tasks_content, "tasks.md"));
        findings.extend(checks::check_task_story_links(&spec, &tasks_content));
    }

    // 7. Test coverage check
    let tests_dir = feature_dir.join("tests");
    findings.extend(checks::check_test_coverage(&spec, &tests_dir));

    // 8. Security heuristics
    if let Some(ref plan) = plan_content {
        findings.extend(checks::check_security_hints(plan, &spec_content));
    }

    // Cap the 8 base-dimension findings first, before appending IA findings.
    // This ensures overflow_count reflects only non-IA findings and IA findings
    // are never silently truncated (which would produce false "all traced" output).
    let overflow_count = if findings.len() > MAX_FINDINGS {
        let overflow = findings.len() - MAX_FINDINGS;
        findings.truncate(MAX_FINDINGS);
        overflow
    } else {
        0
    };

    // 9. Intent alignment (IDSD; 0/10 when intent.md absent) — appended after cap
    let (intent_findings, intent_score) = checks::review_intent_alignment(feature_dir, &spec);
    let intent_finding_count = intent_findings.len();
    findings.extend(intent_findings);

    // Score the 7 base dimensions via penalty-based helper
    let mut dimension_scores = score_dimensions(&findings);

    // IntentAlignment score is computed separately (not penalty-based via findings)
    dimension_scores.push(DimensionScore {
        dimension: Dimension::IntentAlignment,
        score: intent_score,
        max_score: 10.0,
        finding_count: intent_finding_count,
    });

    let overall_score = if dimension_scores.is_empty() {
        100.0
    } else {
        let total: f64 = dimension_scores.iter().map(|d| d.score).sum();
        let max: f64 = dimension_scores.iter().map(|d| d.max_score).sum();
        if max > 0.0 {
            total / max * 100.0
        } else {
            100.0
        }
    };

    Ok(ReviewReport {
        feature_id,
        findings,
        dimension_scores,
        overall_score,
        overflow_count,
    })
}

/// Score each dimension based on findings.
fn score_dimensions(findings: &[ReviewFinding]) -> Vec<DimensionScore> {
    let all_dims = [
        Dimension::Completeness,
        Dimension::Clarity,
        Dimension::Testability,
        Dimension::Consistency,
        Dimension::Security,
        Dimension::Performance,
        Dimension::Maintainability,
    ];

    all_dims
        .into_iter()
        .map(|dim| {
            let dim_findings: Vec<_> = findings.iter().filter(|f| f.dimension == dim).collect();
            let max_score = 10.0;
            let penalty: f64 = dim_findings
                .iter()
                .map(|f| match f.severity {
                    Severity::Critical => 5.0,
                    Severity::High => 3.0,
                    Severity::Medium => 1.5,
                    Severity::Low => 0.5,
                    Severity::Info => 0.0,
                })
                .sum();
            let score = (max_score - penalty).max(0.0);

            DimensionScore {
                dimension: dim,
                score,
                max_score,
                finding_count: dim_findings.len(),
            }
        })
        .collect()
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

    fn setup_project(project_root: &Path) {
        let solidspec = project_root.join(".solidspec");
        std::fs::create_dir_all(&solidspec).unwrap();
        std::fs::write(
            solidspec.join("constitution.md"),
            "### Article VII: Simplicity\n",
        )
        .unwrap();
    }

    const GOOD_SPEC: &str = r#"# Feature Specification: Auth System

## User Scenarios & Testing

### User Story 1 - User login (Priority: P1)

**Acceptance Scenarios**:

1. **Given** valid credentials, **When** user submits login form, **Then** session is created

---

### User Story 2 - Password reset (Priority: P2)

**Acceptance Scenarios**:

1. **Given** registered email, **When** user requests reset, **Then** email is sent

## Requirements

### Functional Requirements

- **FR-001**: System MUST authenticate users via email and password
- **FR-002**: System MUST allow password resets via email

### Key Entities

- **[User]**: A registered account holder
- **[Session]**: An active authentication session

## Success Criteria

- **SC-001**: Users can log in and out
"#;

    const MINIMAL_PLAN: &str = r#"# Architecture Plan

FR-001 handled by auth module.
FR-002 handled by email service.

User entity stored in users table.
Session managed via tokens.
"#;

    const MINIMAL_TASKS: &str = r#"# Task Breakdown

## Phase 1: Setup

- [ ] T001 Initialize project [US1]
- [ ] T002 Setup email service [US2]
"#;

    #[test]
    fn good_spec_scores_high() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(&feature, GOOD_SPEC, Some(MINIMAL_PLAN), Some(MINIMAL_TASKS));
        setup_project(dir.path());

        let report = preflight_review(&feature, dir.path()).unwrap();
        assert!(
            report.overall_score >= 80.0,
            "Expected high score, got {:.0}%",
            report.overall_score
        );
    }

    #[test]
    fn stories_without_scenarios_flagged() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        let spec = "# Spec\n\n## User Scenarios & Testing\n\n### User Story 1 - Test (Priority: P1)\n\nNo scenarios here.\n\n## Requirements\n\n- **FR-001**: System MUST do something\n\n## Success Criteria\n\n- **SC-001**: It works\n";
        setup_feature(&feature, spec, None, None);
        setup_project(dir.path());

        let report = preflight_review(&feature, dir.path()).unwrap();
        assert!(report.findings.iter().any(|f| {
            f.dimension == Dimension::Testability && f.message.contains("no Given/When/Then")
        }));
    }

    #[test]
    fn missing_spec_returns_error() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-missing");
        std::fs::create_dir_all(&feature).unwrap();
        assert!(preflight_review(&feature, dir.path()).is_err());
    }

    #[test]
    fn review_is_read_only() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(&feature, GOOD_SPEC, Some(MINIMAL_PLAN), Some(MINIMAL_TASKS));
        setup_project(dir.path());

        let spec_before = std::fs::read_to_string(feature.join("spec.md")).unwrap();
        let plan_before = std::fs::read_to_string(feature.join("plan.md")).unwrap();

        let _report = preflight_review(&feature, dir.path()).unwrap();

        assert_eq!(
            spec_before,
            std::fs::read_to_string(feature.join("spec.md")).unwrap(),
            "Review modified spec.md!"
        );
        assert_eq!(
            plan_before,
            std::fs::read_to_string(feature.join("plan.md")).unwrap(),
            "Review modified plan.md!"
        );
    }

    fn setup_intent(dir: &Path, content: &str) {
        std::fs::write(dir.join("intent.md"), content).unwrap();
    }

    const SAMPLE_INTENT_ACTIVE: &str = r#"# Intent: Auth System

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

    const SAMPLE_INTENT_DRAFT: &str = r#"# Intent: Auth System

**Intent ID**: INT-001
**Feature**: 001-auth
**Created**: 2026-06-01
**Status**: draft

## Goal
Allow users to authenticate securely.

## Evidence
- Authentication succeeds with valid credentials
"#;

    #[test]
    fn preflight_review_includes_intent_alignment_dimension() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(&feature, GOOD_SPEC, Some(MINIMAL_PLAN), Some(MINIMAL_TASKS));
        setup_project(dir.path());
        // No intent.md → IntentAlignment score must be 0/10

        let report = preflight_review(&feature, dir.path()).unwrap();
        let ia = report
            .dimension_scores
            .iter()
            .find(|ds| ds.dimension == Dimension::IntentAlignment)
            .expect("IntentAlignment dimension must be present");
        assert_eq!(ia.score, 0.0);
        assert_eq!(ia.max_score, 10.0);
    }

    #[test]
    fn format_report_contains_intent_alignment_section() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(&feature, GOOD_SPEC, Some(MINIMAL_PLAN), Some(MINIMAL_TASKS));
        setup_project(dir.path());

        let report = preflight_review(&feature, dir.path()).unwrap();
        let md = format_review_report(&report);
        assert!(
            md.contains("## Intent Alignment"),
            "Report must contain '## Intent Alignment' section"
        );
        assert!(
            md.contains("0/10"),
            "Missing intent.md should show 0/10 in Intent Alignment"
        );
    }

    #[test]
    fn intent_alignment_section_shows_traced_when_all_good() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(&feature, GOOD_SPEC, Some(MINIMAL_PLAN), Some(MINIMAL_TASKS));
        setup_project(dir.path());
        setup_intent(&feature, SAMPLE_INTENT_ACTIVE);

        let report = preflight_review(&feature, dir.path()).unwrap();
        let md = format_review_report(&report);

        assert!(md.contains("## Intent Alignment"));
        let ia = report
            .dimension_scores
            .iter()
            .find(|ds| ds.dimension == Dimension::IntentAlignment)
            .unwrap();
        assert!(
            ia.score > 0.0,
            "Active intent with covered FRs should score > 0"
        );
    }

    #[test]
    fn no_issues_message_suppressed_when_ia_has_findings() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        // Use GOOD_SPEC with MINIMAL_PLAN + MINIMAL_TASKS → all non-IA dimensions pass
        setup_feature(&feature, GOOD_SPEC, Some(MINIMAL_PLAN), Some(MINIMAL_TASKS));
        setup_project(dir.path());
        setup_intent(&feature, SAMPLE_INTENT_DRAFT); // draft → HIGH IA finding

        let report = preflight_review(&feature, dir.path()).unwrap();
        let md = format_review_report(&report);

        assert!(
            !md.contains("No issues found"),
            "Must NOT print 'No issues found' when IA has a HIGH finding"
        );
        assert!(
            md.contains("draft"),
            "Report must show the draft-status finding"
        );
    }

    #[test]
    fn scoring_penalizes_critical_findings() {
        let findings = vec![
            ReviewFinding {
                dimension: Dimension::Completeness,
                severity: Severity::Critical,
                message: "bad".into(),
                remediation: "fix".into(),
                location: None,
            },
            ReviewFinding {
                dimension: Dimension::Completeness,
                severity: Severity::Critical,
                message: "worse".into(),
                remediation: "fix".into(),
                location: None,
            },
        ];
        let scores = score_dimensions(&findings);
        let completeness = scores
            .iter()
            .find(|s| s.dimension == Dimension::Completeness)
            .unwrap();
        assert_eq!(completeness.score, 0.0); // 10 - 5 - 5 = 0 (clamped)
    }
}
