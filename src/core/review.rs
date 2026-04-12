use std::path::Path;

use anyhow::Result;
use regex::Regex;

use super::errors::RustySpecError;
use super::spec_parser;

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
        return Err(RustySpecError::Spec {
            feature_id: feature_id.clone(),
            message: "spec.md not found".into(),
            fix: "Run 'rustyspec specify' first.".into(),
        }
        .into());
    }

    let spec_content = std::fs::read_to_string(&spec_path)?;
    let spec = spec_parser::parse_spec(&spec_path)?;

    let mut findings = Vec::new();

    // 1. Placeholder detection
    findings.extend(check_placeholders(&spec_content, "spec.md"));

    // 2. Section completeness
    findings.extend(check_section_completeness(&spec_content));

    // 3. Ambiguous language
    findings.extend(check_ambiguous_language(&spec_content));

    // 4. Requirement quality
    findings.extend(check_requirement_quality(&spec));

    // 5. Acceptance scenario coverage
    findings.extend(check_scenario_coverage(&spec));

    // 6. Cross-artifact checks
    let plan_path = feature_dir.join("plan.md");
    let tasks_path = feature_dir.join("tasks.md");
    let plan_content = if plan_path.exists() {
        let content = std::fs::read_to_string(&plan_path)?;
        findings.extend(check_placeholders(&content, "plan.md"));
        findings.extend(check_cross_references(&spec, &content, "plan.md"));
        Some(content)
    } else {
        findings.push(ReviewFinding {
            dimension: Dimension::Completeness,
            severity: Severity::High,
            message: "plan.md missing — no architecture plan".into(),
            remediation: "Run 'rustyspec plan' to generate the plan.".into(),
            location: None,
        });
        None
    };

    if tasks_path.exists() {
        let tasks_content = std::fs::read_to_string(&tasks_path)?;
        findings.extend(check_placeholders(&tasks_content, "tasks.md"));
        findings.extend(check_task_story_links(&spec, &tasks_content));
    }

    // 7. Test coverage check
    let tests_dir = feature_dir.join("tests");
    findings.extend(check_test_coverage(&spec, &tests_dir));

    // 8. Security heuristics
    if let Some(ref plan) = plan_content {
        findings.extend(check_security_hints(plan, &spec_content));
    }

    // Cap findings
    let overflow_count = if findings.len() > MAX_FINDINGS {
        let overflow = findings.len() - MAX_FINDINGS;
        findings.truncate(MAX_FINDINGS);
        overflow
    } else {
        0
    };

    // Score dimensions
    let dimension_scores = score_dimensions(&findings);
    let overall_score = if dimension_scores.is_empty() {
        100.0
    } else {
        let total: f64 = dimension_scores.iter().map(|d| d.score).sum();
        let max: f64 = dimension_scores.iter().map(|d| d.max_score).sum();
        if max > 0.0 { total / max * 100.0 } else { 100.0 }
    };

    Ok(ReviewReport {
        feature_id,
        findings,
        dimension_scores,
        overall_score,
        overflow_count,
    })
}

/// Detect placeholder text across any artifact.
fn check_placeholders(content: &str, file_name: &str) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();
    let placeholder_patterns = [
        (r"(?i)\[TODO[:\s]*[^\]]*\]", "TODO marker"),
        (r"(?i)\[TBD[:\s]*[^\]]*\]", "TBD marker"),
        (r"(?i)\[To be filled[^\]]*\]", "'To be filled' placeholder"),
        (r"(?i)\[PLACEHOLDER[^\]]*\]", "PLACEHOLDER marker"),
        (r"(?i)\[Brief Title\]", "'Brief Title' placeholder"),
        (r"(?i)\[NEEDS CLARIFICATION[^\]]*\]", "Unresolved clarification"),
        (r"(?i)\[Insert [^\]]+\]", "'Insert ...' placeholder"),
    ];

    for (pattern, label) in &placeholder_patterns {
        let re = Regex::new(pattern).unwrap();
        for mat in re.find_iter(content) {
            findings.push(ReviewFinding {
                dimension: Dimension::Completeness,
                severity: if *label == "Unresolved clarification" {
                    Severity::High
                } else {
                    Severity::Medium
                },
                message: format!("{label} found in {file_name}: \"{}\"", mat.as_str()),
                remediation: format!("Replace the placeholder in {file_name} with concrete content."),
                location: Some(file_name.to_string()),
            });
        }
    }

    findings
}

/// Check that spec.md has the expected top-level sections.
fn check_section_completeness(content: &str) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();
    let required_sections = [
        ("User Scenarios", "## User Scenarios"),
        ("Requirements", "## Requirements"),
        ("Success Criteria", "## Success Criteria"),
    ];

    for (name, marker) in &required_sections {
        if !content.contains(marker) {
            findings.push(ReviewFinding {
                dimension: Dimension::Completeness,
                severity: Severity::High,
                message: format!("Missing '{name}' section in spec.md"),
                remediation: format!("Add a '{marker}' section with concrete content."),
                location: Some("spec.md".into()),
            });
        }
    }

    // Key Entities section
    if !content.contains("### Key Entities") && !content.contains("## Key Entities") {
        findings.push(ReviewFinding {
            dimension: Dimension::Completeness,
            severity: Severity::Medium,
            message: "Missing 'Key Entities' section in spec.md".into(),
            remediation: "Add a Key Entities section to define domain objects.".into(),
            location: Some("spec.md".into()),
        });
    }

    findings
}

/// Detect weak/ambiguous language patterns.
fn check_ambiguous_language(content: &str) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();
    let weak_terms = [
        ("should", "Use 'MUST' or 'SHALL' for requirements, 'should' is non-binding"),
        ("might", "Replace 'might' with a definite statement"),
        ("possibly", "Replace 'possibly' with a concrete decision"),
        ("approximately", "Replace 'approximately' with a measurable threshold"),
        ("etc.", "Replace 'etc.' with an explicit list"),
        ("and/or", "Choose 'and' or 'or' — 'and/or' is ambiguous"),
        ("as needed", "Define explicit conditions instead of 'as needed'"),
        ("if applicable", "Specify when it applies or remove"),
    ];

    // Only flag terms in requirement-like lines (lines with FR-, MUST, SHALL, or bullet points)
    for line in content.lines() {
        let lower_line = line.to_lowercase();
        let is_requirement_line = lower_line.contains("fr-")
            || lower_line.contains("must")
            || lower_line.contains("shall")
            || line.trim_start().starts_with("- ");

        if !is_requirement_line {
            continue;
        }

        for (term, advice) in &weak_terms {
            // Use word-boundary matching to avoid false positives (e.g. "shoulders" for "should")
            let pattern = format!(r"(?i)\b{}\b", regex::escape(term));
            if let Ok(re) = Regex::new(&pattern) {
                if re.is_match(&lower_line) {
                    findings.push(ReviewFinding {
                        dimension: Dimension::Clarity,
                        severity: Severity::Low,
                        message: format!("Ambiguous term '{term}' in requirement context"),
                        remediation: advice.to_string(),
                        location: Some("spec.md".into()),
                    });
                }
            }
        }
    }

    // De-duplicate: keep only unique messages
    findings.sort_by(|a, b| a.message.cmp(&b.message));
    findings.dedup_by(|a, b| a.message == b.message);

    findings
}

/// Check that each requirement has measurable/testable language.
fn check_requirement_quality(spec: &spec_parser::ParsedSpec) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();

    if spec.requirements.is_empty() {
        findings.push(ReviewFinding {
            dimension: Dimension::Completeness,
            severity: Severity::Critical,
            message: "No functional requirements (FR-###) found in spec.md".into(),
            remediation: "Add functional requirements using the **FR-001**: format.".into(),
            location: Some("spec.md".into()),
        });
        return findings;
    }

    for req in &spec.requirements {
        let lower = req.text.to_lowercase();
        // Check for measurability: should reference a verb like "must", or contain measurable criteria
        let has_verb = lower.contains("must")
            || lower.contains("shall")
            || lower.contains("can")
            || lower.contains("allow")
            || lower.contains("support")
            || lower.contains("provide")
            || lower.contains("enable");

        if !has_verb {
            findings.push(ReviewFinding {
                dimension: Dimension::Testability,
                severity: Severity::Medium,
                message: format!("{}: lacks action verb (MUST/SHALL/CAN)", req.id),
                remediation: format!("Rewrite {} with a clear action verb: 'System MUST...'", req.id),
                location: Some("spec.md".into()),
            });
        }
    }

    findings
}

/// Ensure each user story has at least one acceptance scenario.
fn check_scenario_coverage(spec: &spec_parser::ParsedSpec) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();

    if spec.user_stories.is_empty() {
        findings.push(ReviewFinding {
            dimension: Dimension::Completeness,
            severity: Severity::High,
            message: "No user stories found in spec.md".into(),
            remediation: "Add user stories with ### User Story N - Title (Priority: P1) format.".into(),
            location: Some("spec.md".into()),
        });
        return findings;
    }

    for (i, story) in spec.user_stories.iter().enumerate() {
        if story.acceptance_scenarios.is_empty() {
            findings.push(ReviewFinding {
                dimension: Dimension::Testability,
                severity: Severity::High,
                message: format!(
                    "User Story {} ('{}') has no Given/When/Then acceptance scenarios",
                    i + 1,
                    story.title
                ),
                remediation: "Add at least one **Given**/**When**/**Then** scenario.".into(),
                location: Some("spec.md".into()),
            });
        }
    }

    findings
}

/// Check that spec requirements appear in plan.md.
fn check_cross_references(
    spec: &spec_parser::ParsedSpec,
    plan_content: &str,
    file_name: &str,
) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();

    for req in &spec.requirements {
        if !plan_content.contains(&req.id) {
            findings.push(ReviewFinding {
                dimension: Dimension::Consistency,
                severity: Severity::Medium,
                message: format!("{} not referenced in {file_name}", req.id),
                remediation: format!("Ensure {} is addressed in {file_name}.", req.id),
                location: Some(file_name.to_string()),
            });
        }
    }

    // Check entities are referenced
    for entity in &spec.entities {
        if !plan_content.contains(entity) {
            findings.push(ReviewFinding {
                dimension: Dimension::Consistency,
                severity: Severity::Low,
                message: format!("Entity '{entity}' not mentioned in {file_name}"),
                remediation: format!("Reference '{entity}' in {file_name} for traceability."),
                location: Some(file_name.to_string()),
            });
        }
    }

    findings
}

/// Check that tasks reference user stories ([US1], [US2], etc.).
fn check_task_story_links(
    spec: &spec_parser::ParsedSpec,
    tasks_content: &str,
) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();

    if spec.user_stories.is_empty() {
        return findings;
    }

    let us_re = Regex::new(r"\[US(\d+)\]").unwrap();
    let referenced_stories: Vec<usize> = us_re
        .captures_iter(tasks_content)
        .filter_map(|c| c[1].parse().ok())
        .collect();

    for (i, story) in spec.user_stories.iter().enumerate() {
        let story_num = i + 1;
        if !referenced_stories.contains(&story_num) {
            findings.push(ReviewFinding {
                dimension: Dimension::Consistency,
                severity: Severity::Medium,
                message: format!(
                    "User Story {} ('{}') not linked in tasks.md",
                    story_num, story.title
                ),
                remediation: format!("Add [US{story_num}] tags to relevant tasks."),
                location: Some("tasks.md".into()),
            });
        }
    }

    findings
}

/// Check that tests directory covers acceptance scenarios.
fn check_test_coverage(
    spec: &spec_parser::ParsedSpec,
    tests_dir: &Path,
) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();

    if !tests_dir.exists() {
        if !spec.user_stories.is_empty() {
            findings.push(ReviewFinding {
                dimension: Dimension::Testability,
                severity: Severity::High,
                message: "No tests/ directory found".into(),
                remediation: "Run 'rustyspec tests' to generate test scaffolds.".into(),
                location: None,
            });
        }
        return findings;
    }

    // Count test files (exclude plain .md/.txt but keep .test.txt scaffolds)
    let test_files: Vec<_> = std::fs::read_dir(tests_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| {
            let path = e.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            // Accept .test.txt (generated scaffolds) and any non-md/non-txt extension
            if name.contains(".test.") {
                return true;
            }
            path.extension()
                .is_some_and(|ext| ext != "md" && ext != "txt")
        })
        .collect();

    if test_files.is_empty() && !spec.user_stories.is_empty() {
        findings.push(ReviewFinding {
            dimension: Dimension::Testability,
            severity: Severity::High,
            message: "tests/ directory exists but contains no test files".into(),
            remediation: "Run 'rustyspec tests' to generate test scaffolds.".into(),
            location: None,
        });
    }

    // Count total acceptance scenarios
    let total_scenarios: usize = spec
        .user_stories
        .iter()
        .map(|s| s.acceptance_scenarios.len())
        .sum();

    if total_scenarios > 0 && test_files.len() < spec.user_stories.len() {
        findings.push(ReviewFinding {
            dimension: Dimension::Testability,
            severity: Severity::Medium,
            message: format!(
                "Only {} test file(s) for {} user stories with {} scenarios",
                test_files.len(),
                spec.user_stories.len(),
                total_scenarios,
            ),
            remediation: "Ensure each user story has corresponding test coverage.".into(),
            location: Some("tests/".into()),
        });
    }

    findings
}

/// Basic security-related heuristic checks on plan and spec content.
fn check_security_hints(plan_content: &str, spec_content: &str) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();

    let combined = format!("{spec_content}\n{plan_content}").to_lowercase();

    // If the spec mentions auth/users/passwords but plan has no security section
    // Use word-boundary patterns to avoid false positives ("author" matching "auth")
    let auth_re = Regex::new(r"(?i)\b(auth(entication|orization|enticate|orize)?|login|password|user\s+account|session\s+manag)\b").unwrap();
    let auth_related = auth_re.is_match(&combined);

    if auth_related {
        let plan_lower = plan_content.to_lowercase();
        if !plan_lower.contains("security") && !plan_lower.contains("authentication") {
            findings.push(ReviewFinding {
                dimension: Dimension::Security,
                severity: Severity::Medium,
                message: "Spec references auth/user features but plan lacks a security section".into(),
                remediation: "Add a security section to plan.md covering authentication and authorization.".into(),
                location: Some("plan.md".into()),
            });
        }
    }

    findings
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

    // Findings by severity
    out.push_str(&format!("## Findings ({})", report.findings.len()));
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
            .filter(|f| &f.severity == sev)
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
        let rustyspec = project_root.join(".rustyspec");
        std::fs::create_dir_all(&rustyspec).unwrap();
        std::fs::write(
            rustyspec.join("constitution.md"),
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
    fn placeholder_detection() {
        let findings = check_placeholders("[TODO: fill this in] and [TBD]", "spec.md");
        assert_eq!(findings.len(), 2);
        assert!(findings.iter().all(|f| f.dimension == Dimension::Completeness));
    }

    #[test]
    fn missing_sections_detected() {
        let findings = check_section_completeness("# Just a title\n");
        assert!(findings.len() >= 3); // Missing scenarios, requirements, success criteria
    }

    #[test]
    fn ambiguous_language_flagged() {
        let content = "- **FR-001**: System should possibly handle etc. requests";
        let findings = check_ambiguous_language(content);
        assert!(!findings.is_empty());
    }

    #[test]
    fn empty_spec_means_no_requirements() {
        let spec = spec_parser::parse_spec_content("# Empty\n").unwrap();
        let findings = check_requirement_quality(&spec);
        assert!(findings.iter().any(|f| f.severity == Severity::Critical));
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
    fn cross_reference_gaps_found() {
        let spec = spec_parser::parse_spec_content(GOOD_SPEC).unwrap();
        let plan = "# Plan\nFR-001 covered.\n"; // FR-002 missing
        let findings = check_cross_references(&spec, plan, "plan.md");
        assert!(findings.iter().any(|f| f.message.contains("FR-002")));
    }

    #[test]
    fn task_story_link_gaps() {
        let spec = spec_parser::parse_spec_content(GOOD_SPEC).unwrap();
        let tasks = "- [ ] T001 Setup [US1]\n"; // US2 missing
        let findings = check_task_story_links(&spec, tasks);
        assert!(findings.iter().any(|f| f.message.contains("User Story 2")));
    }

    #[test]
    fn missing_spec_returns_error() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-missing");
        std::fs::create_dir_all(&feature).unwrap();
        assert!(preflight_review(&feature, dir.path()).is_err());
    }

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
        let completeness = scores.iter().find(|s| s.dimension == Dimension::Completeness).unwrap();
        assert_eq!(completeness.score, 0.0); // 10 - 5 - 5 = 0 (clamped)
    }
}
