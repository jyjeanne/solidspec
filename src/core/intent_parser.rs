#![allow(dead_code)]
use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;

use super::errors::SolidSpecError;

static TITLE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#\s+Intent:\s+(.+)").expect("invalid intent title regex"));
static INTENT_ID_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*Intent ID\*\*:\s*(INT-\d+)").expect("invalid intent ID regex")
});
static FEATURE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Feature\*\*:\s*(.+)").expect("invalid feature regex"));
static CREATED_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Created\*\*:\s*(.+)").expect("invalid created date regex"));
static STATUS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Status\*\*:\s*(\w+)").expect("invalid status regex"));

/// Lifecycle status of an intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntentStatus {
    Draft,
    Active,
    Satisfied,
    Drifted,
}

/// A single evidence criterion from `intent.md`, annotated with satisfaction status.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceCriterion {
    /// Raw text of the criterion (as written in `intent.md`).
    pub text: String,
    /// `true` when at least one implemented test scaffold covers this criterion.
    pub satisfied: bool,
}

/// Result of cross-referencing evidence criteria against implemented test scaffolds.
#[derive(Debug, Clone)]
pub struct IntentDrift {
    /// Percentage of evidence criteria that are NOT yet satisfied.
    /// `0.0` at baseline (no tests implemented yet).
    pub score: f64,
    /// Texts of unsatisfied criteria, in original order.
    pub unsatisfied: Vec<String>,
}

impl IntentStatus {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "active" => Self::Active,
            "satisfied" => Self::Satisfied,
            "drifted" => Self::Drifted,
            _ => Self::Draft,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Satisfied => "satisfied",
            Self::Drifted => "drifted",
        }
    }
}

/// Parsed representation of an `intent.md` file (ICE model).
#[derive(Debug, Clone)]
pub struct IntentSpec {
    pub title: String,
    /// e.g. "INT-001"
    pub intent_id: String,
    /// e.g. "001-feature-name"
    pub feature: String,
    pub created: String,
    pub status: IntentStatus,
    /// One-sentence goal — why this capability exists
    pub goal: String,
    /// Boundaries that must remain true regardless of implementation
    pub constraints: Vec<String>,
    /// Measurable success criteria mapping to test scenarios
    pub evidence: Vec<String>,
    pub risks: Vec<String>,
    pub open_questions: Vec<String>,
    pub raw: String,
}

/// Parse `intent.md` from disk.
pub fn parse_intent(path: &Path) -> Result<IntentSpec> {
    let content = std::fs::read_to_string(path).map_err(|e| SolidSpecError::Spec {
        feature_id: path.display().to_string(),
        message: format!("Cannot read intent.md: {e}"),
        fix: "Ensure intent.md exists. Run 'solidspec intent' first.".into(),
    })?;
    parse_intent_content(&content)
}

/// Parse `intent.md` content from a string (testable without file I/O).
pub fn parse_intent_content(content: &str) -> Result<IntentSpec> {
    Ok(IntentSpec {
        title: extract_title(content),
        intent_id: extract_inline_field(content, &INTENT_ID_RE),
        feature: extract_inline_field(content, &FEATURE_RE),
        created: extract_inline_field(content, &CREATED_RE),
        status: extract_status(content),
        goal: extract_section_body(content, "Goal"),
        constraints: extract_list_items(content, "Constraints"),
        evidence: extract_list_items(content, "Evidence"),
        risks: extract_list_items(content, "Risks"),
        open_questions: extract_list_items(content, "Open Questions"),
        raw: content.to_string(),
    })
}

fn extract_title(content: &str) -> String {
    content
        .lines()
        .find_map(|line| TITLE_RE.captures(line).map(|c| c[1].trim().to_string()))
        .unwrap_or_default()
}

fn extract_inline_field(content: &str, re: &Regex) -> String {
    content
        .lines()
        .find_map(|line| re.captures(line).map(|c| c[1].trim().to_string()))
        .unwrap_or_default()
}

fn extract_status(content: &str) -> IntentStatus {
    content
        .lines()
        .find_map(|line| {
            STATUS_RE
                .captures(line)
                .map(|c| IntentStatus::from_str(&c[1]))
        })
        .unwrap_or(IntentStatus::Draft)
}

/// Extract prose body of a `## Section` heading (non-list lines, joined).
fn extract_section_body(content: &str, section: &str) -> String {
    let heading = format!("## {section}");
    let lines: Vec<&str> = content.lines().collect();

    let Some(start) = lines.iter().position(|l| l.trim() == heading.as_str()) else {
        return String::new();
    };

    let rest = &lines[start + 1..];
    let end = rest
        .iter()
        .position(|l| l.starts_with("## "))
        .unwrap_or(rest.len());

    rest[..end]
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with("- "))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Extract bullet list items (`- item`) from a `## Section`.
fn extract_list_items(content: &str, section: &str) -> Vec<String> {
    let heading = format!("## {section}");
    let lines: Vec<&str> = content.lines().collect();

    let Some(start) = lines.iter().position(|l| l.trim() == heading.as_str()) else {
        return Vec::new();
    };

    let rest = &lines[start + 1..];
    let end = rest
        .iter()
        .position(|l| l.starts_with("## "))
        .unwrap_or(rest.len());

    rest[..end]
        .iter()
        .filter_map(|l| {
            let trimmed = l.trim();
            trimmed
                .strip_prefix("- ")
                .map(|item| item.trim().to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INTENT: &str = r#"# Intent: Allow users to export PDF reports

**Intent ID**: INT-001
**Feature**: 001-export-pdf-reports
**Created**: 2026-06-01
**Status**: active

## Goal

Allow users to export any report as a PDF file without leaving the application.

## Constraints

- Must support Markdown output
- Must be CI-compatible (no interactive prompts)
- Must complete in under 10 seconds

## Evidence

- Export command exits 0 when a valid report is provided
- Generated PDF contains all report sections
- All Given/When/Then scenarios pass

## Risks

- PDF rendering library may have licensing restrictions

## Open Questions

- Should we support password-protected PDFs?
"#;

    #[test]
    fn parse_title() {
        let intent = parse_intent_content(SAMPLE_INTENT).unwrap();
        assert_eq!(intent.title, "Allow users to export PDF reports");
    }

    #[test]
    fn parse_intent_id() {
        let intent = parse_intent_content(SAMPLE_INTENT).unwrap();
        assert_eq!(intent.intent_id, "INT-001");
    }

    #[test]
    fn parse_feature() {
        let intent = parse_intent_content(SAMPLE_INTENT).unwrap();
        assert_eq!(intent.feature, "001-export-pdf-reports");
    }

    #[test]
    fn parse_status_active() {
        let intent = parse_intent_content(SAMPLE_INTENT).unwrap();
        assert_eq!(intent.status, IntentStatus::Active);
    }

    #[test]
    fn parse_goal() {
        let intent = parse_intent_content(SAMPLE_INTENT).unwrap();
        assert!(
            intent.goal.contains("export"),
            "Goal should mention export: {}",
            intent.goal
        );
    }

    #[test]
    fn parse_constraints_count() {
        let intent = parse_intent_content(SAMPLE_INTENT).unwrap();
        assert_eq!(intent.constraints.len(), 3);
        assert!(intent.constraints[0].contains("Markdown"));
    }

    #[test]
    fn parse_evidence_count() {
        let intent = parse_intent_content(SAMPLE_INTENT).unwrap();
        assert_eq!(intent.evidence.len(), 3);
        assert!(intent.evidence[0].contains("exits 0"));
    }

    #[test]
    fn parse_risks() {
        let intent = parse_intent_content(SAMPLE_INTENT).unwrap();
        assert_eq!(intent.risks.len(), 1);
        assert!(intent.risks[0].contains("PDF rendering"));
    }

    #[test]
    fn parse_open_questions() {
        let intent = parse_intent_content(SAMPLE_INTENT).unwrap();
        assert_eq!(intent.open_questions.len(), 1);
        assert!(intent.open_questions[0].contains("password"));
    }

    #[test]
    fn parse_empty_content_returns_defaults() {
        let intent = parse_intent_content("").unwrap();
        assert!(intent.title.is_empty());
        assert!(intent.intent_id.is_empty());
        assert_eq!(intent.status, IntentStatus::Draft);
        assert!(intent.constraints.is_empty());
        assert!(intent.evidence.is_empty());
    }

    #[test]
    fn status_from_str_variants() {
        assert_eq!(IntentStatus::from_str("draft"), IntentStatus::Draft);
        assert_eq!(IntentStatus::from_str("active"), IntentStatus::Active);
        assert_eq!(IntentStatus::from_str("satisfied"), IntentStatus::Satisfied);
        assert_eq!(IntentStatus::from_str("drifted"), IntentStatus::Drifted);
        assert_eq!(IntentStatus::from_str("ACTIVE"), IntentStatus::Active);
        assert_eq!(IntentStatus::from_str("unknown"), IntentStatus::Draft);
    }

    #[test]
    fn status_as_str_roundtrip() {
        for status in [
            IntentStatus::Draft,
            IntentStatus::Active,
            IntentStatus::Satisfied,
            IntentStatus::Drifted,
        ] {
            let s = status.as_str();
            assert_eq!(IntentStatus::from_str(s), status);
        }
    }

    #[test]
    fn raw_field_preserved() {
        let intent = parse_intent_content(SAMPLE_INTENT).unwrap();
        assert_eq!(intent.raw, SAMPLE_INTENT);
    }
}
