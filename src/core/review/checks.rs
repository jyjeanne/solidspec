//! Individual preflight review checks. Each check inspects one artifact (or
//! a pair of artifacts for cross-referencing) and returns the findings it
//! detected — self-contained, no shared mutable state. `super::preflight_review`
//! runs them in sequence and aggregates the results.

use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use super::{Dimension, ReviewFinding, Severity};
use crate::core::{intent_parser, spec_parser};

static US_STORY_LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[US(\d+)\]").expect("invalid us story link regex"));
static AUTH_TERMS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(auth(entication|orization|enticate|orize)?|login|password|user\s+account|session\s+manag)\b")
        .expect("invalid auth terms regex")
});

static PLACEHOLDER_PATTERNS: LazyLock<Vec<(Regex, &str, Severity)>> = LazyLock::new(|| {
    vec![
        (
            Regex::new(r"(?i)\[TODO[:\s]*[^\]]*\]").unwrap(),
            "TODO marker",
            Severity::Medium,
        ),
        (
            Regex::new(r"(?i)\[TBD[:\s]*[^\]]*\]").unwrap(),
            "TBD marker",
            Severity::Medium,
        ),
        (
            Regex::new(r"(?i)\[To be filled[^\]]*\]").unwrap(),
            "'To be filled' placeholder",
            Severity::Medium,
        ),
        (
            Regex::new(r"(?i)\[PLACEHOLDER[^\]]*\]").unwrap(),
            "PLACEHOLDER marker",
            Severity::Medium,
        ),
        (
            Regex::new(r"(?i)\[Brief Title\]").unwrap(),
            "'Brief Title' placeholder",
            Severity::Medium,
        ),
        (
            Regex::new(r"(?i)\[NEEDS CLARIFICATION[^\]]*\]").unwrap(),
            "Unresolved clarification",
            Severity::High,
        ),
        (
            Regex::new(r"(?i)\[Insert [^\]]+\]").unwrap(),
            "'Insert ...' placeholder",
            Severity::Medium,
        ),
    ]
});

/// Detect placeholder text across any artifact.
pub(crate) fn check_placeholders(content: &str, file_name: &str) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();

    for (re, label, severity) in &*PLACEHOLDER_PATTERNS {
        for mat in re.find_iter(content) {
            findings.push(ReviewFinding {
                dimension: Dimension::Completeness,
                severity: severity.clone(),
                message: format!("{label} found in {file_name}: \"{}\"", mat.as_str()),
                remediation: format!(
                    "Replace the placeholder in {file_name} with concrete content."
                ),
                location: Some(file_name.to_string()),
            });
        }
    }

    findings
}

/// Check that spec.md has the expected top-level sections.
pub(crate) fn check_section_completeness(content: &str) -> Vec<ReviewFinding> {
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
pub(crate) fn check_ambiguous_language(content: &str) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();
    let weak_terms = [
        (
            "should",
            "Use 'MUST' or 'SHALL' for requirements, 'should' is non-binding",
        ),
        ("might", "Replace 'might' with a definite statement"),
        ("possibly", "Replace 'possibly' with a concrete decision"),
        (
            "approximately",
            "Replace 'approximately' with a measurable threshold",
        ),
        ("etc.", "Replace 'etc.' with an explicit list"),
        ("and/or", "Choose 'and' or 'or' — 'and/or' is ambiguous"),
        (
            "as needed",
            "Define explicit conditions instead of 'as needed'",
        ),
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
            if let Ok(re) = Regex::new(&pattern)
                && re.is_match(&lower_line)
            {
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

    // De-duplicate: keep only unique messages
    findings.sort_by(|a, b| a.message.cmp(&b.message));
    findings.dedup_by(|a, b| a.message == b.message);

    findings
}

/// Check that each requirement has measurable/testable language.
pub(crate) fn check_requirement_quality(spec: &spec_parser::ParsedSpec) -> Vec<ReviewFinding> {
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
                remediation: format!(
                    "Rewrite {} with a clear action verb: 'System MUST...'",
                    req.id
                ),
                location: Some("spec.md".into()),
            });
        }
    }

    findings
}

/// Ensure each user story has at least one acceptance scenario.
pub(crate) fn check_scenario_coverage(spec: &spec_parser::ParsedSpec) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();

    if spec.user_stories.is_empty() {
        findings.push(ReviewFinding {
            dimension: Dimension::Completeness,
            severity: Severity::High,
            message: "No user stories found in spec.md".into(),
            remediation: "Add user stories with ### User Story N - Title (Priority: P1) format."
                .into(),
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
pub(crate) fn check_cross_references(
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
pub(crate) fn check_task_story_links(
    spec: &spec_parser::ParsedSpec,
    tasks_content: &str,
) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();

    if spec.user_stories.is_empty() {
        return findings;
    }

    let referenced_stories: Vec<usize> = US_STORY_LINK_RE
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
pub(crate) fn check_test_coverage(
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
                remediation: "Run 'solidspec tests' to generate test scaffolds.".into(),
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
            remediation: "Run 'solidspec tests' to generate test scaffolds.".into(),
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
pub(crate) fn check_security_hints(plan_content: &str, spec_content: &str) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();

    let combined = format!("{spec_content}\n{plan_content}").to_lowercase();

    // If the spec mentions auth/users/passwords but plan has no security section
    // Use word-boundary patterns to avoid false positives ("author" matching "auth")
    let auth_related = AUTH_TERMS_RE.is_match(&combined);

    if auth_related {
        let plan_lower = plan_content.to_lowercase();
        if !plan_lower.contains("security") && !plan_lower.contains("authentication") {
            findings.push(ReviewFinding {
                dimension: Dimension::Security,
                severity: Severity::Medium,
                message: "Spec references auth/user features but plan lacks a security section"
                    .into(),
                remediation:
                    "Add a security section to plan.md covering authentication and authorization."
                        .into(),
                location: Some("plan.md".into()),
            });
        }
    }

    findings
}

/// Check intent alignment for the IDSD workflow.
///
/// Returns `(findings, score_out_of_10)`.
/// When `intent.md` is absent the score is `0.0` (0/10) with no findings —
/// the caller adds the `DimensionScore` directly so the table always shows the row.
pub(crate) fn review_intent_alignment(
    feature_dir: &Path,
    spec: &spec_parser::ParsedSpec,
) -> (Vec<ReviewFinding>, f64) {
    let intent_path = feature_dir.join("intent.md");
    if !intent_path.exists() {
        return (vec![], 0.0);
    }

    let intent = match intent_parser::parse_intent(&intent_path) {
        Ok(i) => i,
        Err(e) => {
            return (
                vec![ReviewFinding {
                    dimension: Dimension::IntentAlignment,
                    severity: Severity::High,
                    message: format!("intent.md could not be parsed: {e}"),
                    remediation: "Check intent.md format. Run 'solidspec intent' to recreate it."
                        .into(),
                    location: Some("intent.md".into()),
                }],
                0.0,
            );
        }
    };

    let mut findings = Vec::new();
    let mut penalty: f64 = 0.0;

    // Intent must not be in 'draft' status before implementation
    if intent.status == intent_parser::IntentStatus::Draft {
        findings.push(ReviewFinding {
            dimension: Dimension::IntentAlignment,
            severity: Severity::High,
            message: "Intent is in 'draft' status — must be 'active' before implementation".into(),
            remediation: "Update the **Status** field in intent.md to 'active'.".into(),
            location: Some("intent.md".into()),
        });
        penalty += 3.0;
    }

    // Every FR-XXX must trace to at least one evidence criterion (keyword overlap).
    // Prefer long keywords (≥5 chars); fall back to shorter ones (≥3 chars) for terse
    // requirements so that "Must log on" still matches "log" in any evidence criterion
    // rather than requiring an impossible verbatim phrase match.
    for req in &spec.requirements {
        let req_keywords: Vec<String> = {
            let long: Vec<String> = req
                .text
                .split_whitespace()
                .map(|w| {
                    w.trim_matches(|c: char| !c.is_alphanumeric())
                        .to_lowercase()
                })
                .filter(|w| w.len() >= 5)
                .collect();
            if long.is_empty() {
                req.text
                    .split_whitespace()
                    .map(|w| {
                        w.trim_matches(|c: char| !c.is_alphanumeric())
                            .to_lowercase()
                    })
                    .filter(|w| w.len() >= 3)
                    .collect()
            } else {
                long
            }
        };

        let covered = if req_keywords.is_empty() {
            // All tokens are 1-2 chars (e.g., "FR-001: Do it") — skip tracing check.
            true
        } else {
            intent.evidence.iter().any(|ev| {
                let ev_lower = ev.to_lowercase();
                req_keywords.iter().any(|kw| ev_lower.contains(kw.as_str()))
            })
        };

        if !covered {
            findings.push(ReviewFinding {
                dimension: Dimension::IntentAlignment,
                severity: Severity::Medium,
                message: format!(
                    "{} cannot be traced to any evidence criterion in intent.md",
                    req.id
                ),
                remediation: format!(
                    "Add an evidence criterion in intent.md that covers the intent of {}.",
                    req.id
                ),
                location: Some("intent.md".into()),
            });
            penalty += 1.5;
        }
    }

    let score = (10.0 - penalty).max(0.0);
    (findings, score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

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

    fn setup_intent(dir: &Path, content: &str) {
        std::fs::write(dir.join("intent.md"), content).unwrap();
    }

    fn minimal_spec_with_reqs() -> spec_parser::ParsedSpec {
        spec_parser::parse_spec_content(
            "## Requirements\n\
             - **FR-001**: System MUST authenticate users\n\
             - **FR-002**: System MUST allow password resets\n",
        )
        .unwrap()
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
    fn placeholder_detection() {
        let findings = check_placeholders("[TODO: fill this in] and [TBD]", "spec.md");
        assert_eq!(findings.len(), 2);
        assert!(
            findings
                .iter()
                .all(|f| f.dimension == Dimension::Completeness)
        );
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

    // ── review_intent_alignment() tests ─────────────────────────────────────

    #[test]
    fn intent_alignment_zero_score_when_no_intent_md() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();
        let spec = minimal_spec_with_reqs();

        let (findings, score) = review_intent_alignment(&feature, &spec);
        assert_eq!(score, 0.0, "Score must be 0.0 when intent.md is absent");
        assert!(findings.is_empty(), "No findings when intent.md absent");
    }

    #[test]
    fn intent_alignment_high_finding_when_draft_status() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();
        setup_intent(&feature, SAMPLE_INTENT_DRAFT);
        let spec = minimal_spec_with_reqs();

        let (findings, score) = review_intent_alignment(&feature, &spec);
        assert!(score < 10.0, "Draft status should penalise score");
        assert!(
            findings
                .iter()
                .any(|f| f.severity == Severity::High && f.message.contains("draft")),
            "Expected HIGH finding about draft status"
        );
    }

    #[test]
    fn intent_alignment_medium_finding_for_untraced_requirement() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();

        // Evidence only mentions authentication, not password reset
        let intent = "# Intent: Auth\n\n\
                      **Intent ID**: INT-001\n**Feature**: 001-auth\n\
                      **Created**: 2026-01-01\n**Status**: active\n\n\
                      ## Goal\nAuth.\n\n\
                      ## Evidence\n- Authentication succeeds with valid credentials\n";
        setup_intent(&feature, intent);

        let spec = minimal_spec_with_reqs(); // has FR-001 (authenticate) and FR-002 (password resets)

        let (findings, score) = review_intent_alignment(&feature, &spec);
        assert!(
            findings.iter().any(|f| f.severity == Severity::Medium
                && f.message.contains("FR-002")
                && f.message.contains("evidence criterion")),
            "FR-002 not covered by evidence — expected MEDIUM finding"
        );
        assert!(score < 10.0);
    }

    #[test]
    fn intent_alignment_perfect_score_when_all_requirements_covered() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();
        setup_intent(&feature, SAMPLE_INTENT_ACTIVE); // covers authenticate AND password reset
        let spec = minimal_spec_with_reqs();

        let (findings, score) = review_intent_alignment(&feature, &spec);
        assert_eq!(score, 10.0, "All requirements covered → 10/10");
        assert!(
            !findings
                .iter()
                .any(|f| f.severity == Severity::Medium || f.severity == Severity::High),
            "No Medium/High findings when all FRs are traced"
        );
    }

    // ── Regression tests for confirmed bugs ─────────────────────────────────

    #[test]
    fn parse_error_shows_high_finding_not_not_found_message() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();
        // Write a malformed intent.md (not valid ICE content but parseable as a file)
        // parse_intent may still succeed for minimal content, so write truly broken UTF-8-like
        // content that forces a structural parse issue — in practice the parser is lenient,
        // so write an intent.md that triggers Err via a bad file read by making it a directory.
        // Instead: directly test review_intent_alignment with a bad file path indirection.
        // The simplest way: create a directory named intent.md so open() fails.
        let intent_path = feature.join("intent.md");
        std::fs::create_dir_all(&intent_path).unwrap(); // directory, not a file → read_to_string fails

        let spec = minimal_spec_with_reqs();
        let (findings, score) = review_intent_alignment(&feature, &spec);

        // Must return a High finding describing the parse failure, NOT return ([], 0.0)
        assert_eq!(score, 0.0);
        assert!(
            !findings.is_empty(),
            "Parse failure must produce a finding, not silent ([], 0.0)"
        );
        assert!(
            findings
                .iter()
                .any(|f| f.severity == Severity::High && f.message.contains("could not be parsed")),
            "Finding must say 'could not be parsed', got: {:?}",
            findings.iter().map(|f| &f.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn terse_requirements_are_not_false_medium() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();

        // Evidence explicitly covers "log on"
        let intent = "# Intent: Login\n\n\
                      **Intent ID**: INT-001\n**Feature**: 001-auth\n\
                      **Created**: 2026-01-01\n**Status**: active\n\n\
                      ## Goal\nUsers can log on.\n\n\
                      ## Evidence\n- Users can log on to the system\n";
        setup_intent(&feature, intent);

        // Requirement whose keywords are all < 5 chars: "log" (3), "on" (2), "must" (4)
        let spec = spec_parser::parse_spec_content("## Requirements\n- **FR-001**: Must log on\n")
            .unwrap();

        let (findings, score) = review_intent_alignment(&feature, &spec);
        assert!(
            !findings
                .iter()
                .any(|f| f.severity == Severity::Medium && f.message.contains("FR-001")),
            "FR-001 with short keywords should NOT get a false MEDIUM finding (score={score})"
        );
    }
}
