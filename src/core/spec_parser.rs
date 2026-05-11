#![allow(dead_code)]
use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;

use super::errors::SolidSpecError;

static USER_STORY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"###\s+User Story \d+\s*-\s*(.+?)\s*\(Priority:\s*(P\d+)\)")
        .expect("invalid user story regex")
});
static SCENARIO_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Given\*\*\s+(.+)").expect("invalid scenario regex"));
static REQUIREMENT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*(FR-\d{3})\*\*:\s*(.+)").expect("invalid requirement regex")
});
static CLARIFICATION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[NEEDS CLARIFICATION[:\s]*([^\]]*)\]").expect("invalid clarification regex")
});
static ENTITY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*\[([^\]]+)\]\*\*:\s*(.*)").expect("invalid entity regex")
});

/// Parsed representation of a spec.md file.
#[derive(Debug, Clone)]
pub struct ParsedSpec {
    pub user_stories: Vec<UserStory>,
    pub requirements: Vec<Requirement>,
    pub clarification_markers: Vec<ClarificationMarker>,
    pub entities: Vec<String>,
    pub raw: String,
}

#[derive(Debug, Clone)]
pub struct UserStory {
    pub title: String,
    pub priority: String, // P1, P2, P3...
    pub acceptance_scenarios: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Requirement {
    pub id: String, // FR-001, FR-002...
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ClarificationMarker {
    pub text: String,
    pub line_number: usize,
}

pub fn parse_spec(path: &Path) -> Result<ParsedSpec> {
    let content = std::fs::read_to_string(path).map_err(|e| SolidSpecError::Spec {
        feature_id: path.display().to_string(),
        message: format!("Cannot read spec: {e}"),
        fix: "Ensure spec.md exists. Run 'solidspec specify' first.".into(),
    })?;

    parse_spec_content(&content)
}

pub fn parse_spec_content(content: &str) -> Result<ParsedSpec> {
    let user_stories = extract_user_stories(content);
    let requirements = extract_requirements(content);
    let clarification_markers = extract_clarification_markers(content);
    let entities = extract_entities(content);

    Ok(ParsedSpec {
        user_stories,
        requirements,
        clarification_markers,
        entities,
        raw: content.to_string(),
    })
}

fn extract_user_stories(content: &str) -> Vec<UserStory> {
    let mut stories = Vec::new();

    for caps in USER_STORY_RE.captures_iter(content) {
        let title = caps[1].trim().to_string();
        let priority = caps[2].to_string();

        // Find acceptance scenarios after this story header
        let start = caps.get(0).unwrap().end();
        let rest = &content[start..];
        // Scenarios end at next ### or end of content
        let end = rest.find("\n### ").unwrap_or(rest.len());
        let section = &rest[..end];

        let scenarios: Vec<String> = SCENARIO_RE
            .captures_iter(section)
            .map(|c| c[1].trim().to_string())
            .collect();

        stories.push(UserStory {
            title,
            priority,
            acceptance_scenarios: scenarios,
        });
    }

    stories
}

fn extract_requirements(content: &str) -> Vec<Requirement> {
    REQUIREMENT_RE
        .captures_iter(content)
        .map(|caps| Requirement {
            id: caps[1].to_string(),
            text: caps[2].trim().to_string(),
        })
        .collect()
}

fn extract_clarification_markers(content: &str) -> Vec<ClarificationMarker> {
    content
        .lines()
        .enumerate()
        .flat_map(|(line_num, line)| {
            CLARIFICATION_RE.captures_iter(line).map(move |caps| ClarificationMarker {
                text: caps
                    .get(1)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default(),
                line_number: line_num + 1,
            })
        })
        .collect()
}

fn extract_entities(content: &str) -> Vec<String> {
    extract_entities_with_descriptions(content)
        .into_iter()
        .map(|(name, _)| name)
        .collect()
}

/// Extract entities with their descriptions from the Key Entities section.
pub fn extract_entities_with_descriptions(content: &str) -> Vec<(String, String)> {
    let section_start = content.find("### Key Entities");
    if let Some(start) = section_start {
        let rest = &content[start..];
        let end = rest[1..].find("\n## ").map(|i| i + 1).unwrap_or(rest.len());
        let section = &rest[..end];

        ENTITY_RE
            .captures_iter(section)
            .map(|caps| (caps[1].to_string(), caps[2].trim().to_string()))
            .collect()
    } else {
        Vec::new()
    }
}

/// Validate spec quality. Returns list of issues.
pub fn validate_spec_quality(content: &str) -> Vec<String> {
    let mut issues = Vec::new();

    // Check for implementation details
    let impl_patterns = [
        ("REST API", "Spec mentions REST API (implementation detail)"),
        ("database", "Spec mentions database (implementation detail)"),
        ("SQL", "Spec mentions SQL (implementation detail)"),
        (
            "HTTP endpoint",
            "Spec mentions HTTP endpoint (implementation detail)",
        ),
        (
            "microservice",
            "Spec mentions microservice (implementation detail)",
        ),
    ];

    let lower = content.to_lowercase();
    for (pattern, msg) in &impl_patterns {
        if lower.contains(&pattern.to_lowercase()) {
            issues.push(msg.to_string());
        }
    }

    // Count markers
    let marker_count = content.matches("[NEEDS CLARIFICATION").count();
    if marker_count > 3 {
        issues.push(format!(
            "Too many [NEEDS CLARIFICATION] markers ({marker_count}, max 3)"
        ));
    }

    issues
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SPEC: &str = r#"# Feature Specification: Chat System

## User Scenarios & Testing

### User Story 1 - Real-time messaging (Priority: P1)

Send messages in real-time.

**Acceptance Scenarios**:

1. **Given** user is logged in, **When** sends a message, **Then** recipient sees it instantly

---

### User Story 2 - Message history (Priority: P2)

View past messages.

**Acceptance Scenarios**:

1. **Given** user opens chat, **When** scrolls up, **Then** sees older messages

---

### User Story 3 - User presence (Priority: P3)

See who is online.

## Requirements

### Functional Requirements

- **FR-001**: System MUST deliver messages in real-time
- **FR-002**: System MUST persist message history
- **FR-003**: System MUST track [NEEDS CLARIFICATION: retention period not specified]

### Key Entities

- **[Message]**: A text payload with sender, timestamp, and content
- **[User]**: A participant in the chat system

## Success Criteria

- **SC-001**: Users can send and receive messages
"#;

    #[test]
    fn parse_three_stories_with_correct_priorities() {
        let parsed = parse_spec_content(SAMPLE_SPEC).unwrap();
        assert_eq!(parsed.user_stories.len(), 3);
        assert_eq!(parsed.user_stories[0].priority, "P1");
        assert_eq!(parsed.user_stories[1].priority, "P2");
        assert_eq!(parsed.user_stories[2].priority, "P3");
        assert_eq!(parsed.user_stories[0].title, "Real-time messaging");
    }

    #[test]
    fn extract_acceptance_scenarios() {
        let parsed = parse_spec_content(SAMPLE_SPEC).unwrap();
        assert!(!parsed.user_stories[0].acceptance_scenarios.is_empty());
    }

    #[test]
    fn extract_requirements_numbered() {
        let parsed = parse_spec_content(SAMPLE_SPEC).unwrap();
        assert_eq!(parsed.requirements.len(), 3);
        assert_eq!(parsed.requirements[0].id, "FR-001");
        assert_eq!(parsed.requirements[2].id, "FR-003");
    }

    #[test]
    fn identify_clarification_markers_with_count() {
        let parsed = parse_spec_content(SAMPLE_SPEC).unwrap();
        assert_eq!(parsed.clarification_markers.len(), 1);
        assert!(
            parsed.clarification_markers[0]
                .text
                .contains("retention period")
        );
    }

    #[test]
    fn multiple_markers_counted() {
        let content =
            "- [NEEDS CLARIFICATION: a]\n- [NEEDS CLARIFICATION: b]\n- [NEEDS CLARIFICATION: c]";
        let parsed = parse_spec_content(content).unwrap();
        assert_eq!(parsed.clarification_markers.len(), 3);
    }

    #[test]
    fn extract_entities() {
        let parsed = parse_spec_content(SAMPLE_SPEC).unwrap();
        assert_eq!(parsed.entities.len(), 2);
        assert!(parsed.entities.contains(&"Message".to_string()));
        assert!(parsed.entities.contains(&"User".to_string()));
    }

    #[test]
    fn empty_spec_handled() {
        let parsed = parse_spec_content("").unwrap();
        assert!(parsed.user_stories.is_empty());
        assert!(parsed.requirements.is_empty());
        assert!(parsed.clarification_markers.is_empty());
    }

    #[test]
    fn validate_spec_detects_impl_details() {
        let content = "System MUST expose a REST API for user management";
        let issues = validate_spec_quality(content);
        assert!(!issues.is_empty());
    }

    #[test]
    fn validate_spec_clean_passes() {
        let content = "Users MUST be able to send messages to other users in real-time";
        let issues = validate_spec_quality(content);
        assert!(issues.is_empty());
    }

    #[test]
    fn validate_spec_too_many_markers() {
        let content = "[NEEDS CLARIFICATION: a] [NEEDS CLARIFICATION: b] [NEEDS CLARIFICATION: c] [NEEDS CLARIFICATION: d]";
        let issues = validate_spec_quality(content);
        assert!(issues.iter().any(|i| i.contains("Too many")));
    }

    #[test]
    fn extract_entities_with_descriptions() {
        let entities = super::extract_entities_with_descriptions(SAMPLE_SPEC);
        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].0, "Message");
        assert!(
            entities[0].1.contains("text payload"),
            "Expected description, got: {}",
            entities[0].1
        );
        assert_eq!(entities[1].0, "User");
    }

    #[test]
    fn extract_entities_with_empty_description() {
        let content = "### Key Entities\n\n- **[Foo]**: \n\n## Next Section\n";
        let entities = super::extract_entities_with_descriptions(content);
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].0, "Foo");
        assert!(entities[0].1.is_empty());
    }
}
