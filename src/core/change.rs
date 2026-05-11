//! Change-based workflow — delta specs for brownfield modifications.
//!
//! Instead of creating a new feature from scratch for every change,
//! users create lightweight change folders with delta specs that describe
//! what was ADDED, MODIFIED, or REMOVED relative to the main spec.
//! The archive command merges deltas back into the main spec.

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use anyhow::{Context, Result, bail};
use regex::Regex;
use serde::{Deserialize, Serialize};

static FR_REQUIREMENT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*(FR-\d{3})\*\*:\s*(.+)").expect("invalid FR requirement regex")
});
static FR_LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*-\s*\*\*(FR-\d{3})\*\*:").expect("invalid FR line regex"));
static FR_ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(FR-\d{3})").expect("invalid FR id regex"));

/// A parsed delta spec — describes changes to a feature's main spec.
#[derive(Debug, Clone)]
pub struct DeltaSpec {
    /// New requirements being added
    pub added: Vec<DeltaRequirement>,
    /// Existing requirements being modified (with old + new text)
    pub modified: Vec<DeltaModification>,
    /// Requirement IDs being removed
    pub removed: Vec<String>,
    /// Raw content of the delta spec file
    #[allow(dead_code)]
    pub raw: String,
}

/// A single requirement being added or modified.
#[derive(Debug, Clone)]
pub struct DeltaRequirement {
    pub id: String,   // FR-042, etc.
    pub text: String, // full requirement text
}

/// A requirement modification — tracks what changed.
#[derive(Debug, Clone)]
pub struct DeltaModification {
    pub id: String,
    pub new_text: String,
    #[allow(dead_code)]
    pub previous_text: Option<String>,
}

/// Metadata about a change, stored in `.change.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMetadata {
    pub slug: String,
    pub title: String,
    pub status: ChangeStatus,
    pub created_at: String,
    pub schema: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChangeStatus {
    #[serde(rename = "proposed")]
    Proposed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "archived")]
    Archived,
}

impl ChangeMetadata {
    pub fn new(slug: &str, title: &str) -> Self {
        Self {
            slug: slug.to_string(),
            title: title.to_string(),
            status: ChangeStatus::Proposed,
            created_at: chrono::Utc::now().to_rfc3339(),
            schema: "spec-driven".into(),
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let meta: Self = serde_yaml::from_str(&content)?;
        Ok(meta)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Parse a delta spec file into structured sections.
/// Format:
///   ## Added Requirements
///   - **FR-042**: text
///   ## Modified Requirements
///   - **FR-012**: new text (was: old text)
///   ## Removed Requirements
///   - FR-008
pub fn parse_delta_spec(content: &str) -> DeltaSpec {
    let added = extract_added(content);
    let modified = extract_modified(content);
    let removed = extract_removed(content);

    DeltaSpec {
        added,
        modified,
        removed,
        raw: content.to_string(),
    }
}

fn extract_added(content: &str) -> Vec<DeltaRequirement> {
    let section = extract_section(content, "## Added");
    if section.is_empty() {
        return vec![];
    }

    FR_REQUIREMENT_RE
        .captures_iter(&section)
        .map(|caps| DeltaRequirement {
            id: caps[1].to_string(),
            text: caps[2].trim().to_string(),
        })
        .collect()
}

fn extract_modified(content: &str) -> Vec<DeltaModification> {
    let section = extract_section(content, "## Modified");
    if section.is_empty() {
        return vec![];
    }

    FR_REQUIREMENT_RE
        .captures_iter(&section)
        .map(|caps| {
            let id = caps[1].to_string();
            let full_text = caps[2].trim().to_string();
            // Split on " (was:" (case-insensitive) to get new + previous text
            let lower = full_text.to_lowercase();
            if let Some(was_pos) = lower.find(" (was:") {
                let new_text = full_text[..was_pos].trim().to_string();
                let prev = full_text[was_pos + 6..] // skip " (was:"
                    .trim_end_matches(')')
                    .trim()
                    .to_string();
                DeltaModification {
                    id,
                    new_text,
                    previous_text: if prev.is_empty() { None } else { Some(prev) },
                }
            } else {
                DeltaModification {
                    id,
                    new_text: full_text,
                    previous_text: None,
                }
            }
        })
        .collect()
}

fn extract_removed(content: &str) -> Vec<String> {
    let section = extract_section(content, "## Removed");
    if section.is_empty() {
        return vec![];
    }

    FR_ID_RE
        .captures_iter(&section)
        .map(|caps| caps[1].to_string())
        .collect()
}

fn extract_section(content: &str, marker: &str) -> String {
    if let Some(start) = content.find(marker) {
        let rest = &content[start..];
        // End at next ## heading or end of content
        let end = rest[marker.len()..]
            .find("\n## ")
            .map(|i| i + marker.len())
            .unwrap_or(rest.len());
        rest[..end].to_string()
    } else {
        String::new()
    }
}

/// Apply delta changes to a main spec content string.
/// Returns the merged spec.
pub fn merge_deltas(main_spec: &str, delta: &DeltaSpec) -> Result<String> {
    let mut lines: Vec<String> = main_spec.lines().map(String::from).collect();

    // 1. Remove requirements in delta.removed
    for removed_id in &delta.removed {
        lines.retain(|line| {
            if let Some(caps) = FR_LINE_RE.captures(line) {
                caps[1] != *removed_id
            } else {
                true
            }
        });
    }

    // 2. Update modified requirements in-place
    for mod_req in &delta.modified {
        for line in &mut lines {
            if let Some(caps) = FR_LINE_RE.captures(line)
                && caps[1] == mod_req.id
            {
                *line = format!("- **{}**: {}", mod_req.id, mod_req.new_text);
            }
        }
    }

    // 3. Append added requirements
    if !delta.added.is_empty() {
        let mut added_lines = Vec::new();
        added_lines.push(String::new());
        for req in &delta.added {
            added_lines.push(format!("- **{}**: {}", req.id, req.text));
        }
        lines.extend(added_lines);
    }

    Ok(lines.join("\n"))
}

/// Discover all active changes for a feature directory.
pub fn list_changes(feature_dir: &Path) -> Result<Vec<ChangeInfo>> {
    let changes_dir = feature_dir.join("changes");
    if !changes_dir.exists() {
        return Ok(vec![]);
    }

    let mut changes = Vec::new();
    for entry in std::fs::read_dir(&changes_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let slug = entry.file_name().to_string_lossy().to_string();
            // Skip the archive directory itself
            if slug == "archive" {
                continue;
            }
            let meta_path = entry.path().join(".change.yaml");
            let (title, status) = if meta_path.exists() {
                match ChangeMetadata::load(&meta_path) {
                    Ok(meta) => (meta.title, meta.status),
                    Err(_) => (slug.clone(), ChangeStatus::Proposed),
                }
            } else {
                (slug.clone(), ChangeStatus::Proposed)
            };

            changes.push(ChangeInfo {
                slug,
                title,
                status,
                dir: entry.path(),
            });
        }
    }

    // Sort: proposed first, then in-progress
    changes.sort_by(|a, b| {
        let a_weight = match a.status {
            ChangeStatus::Proposed => 0,
            ChangeStatus::InProgress => 1,
            ChangeStatus::Archived => 2,
        };
        let b_weight = match b.status {
            ChangeStatus::Proposed => 0,
            ChangeStatus::InProgress => 1,
            ChangeStatus::Archived => 2,
        };
        a_weight.cmp(&b_weight).then_with(|| a.slug.cmp(&b.slug))
    });

    Ok(changes)
}

/// Summary info about a change, for CLI display.
#[derive(Debug, Clone)]
pub struct ChangeInfo {
    pub slug: String,
    pub title: String,
    pub status: ChangeStatus,
    #[allow(dead_code)]
    pub dir: PathBuf,
}

/// Generate a slug from a description (lowercase, hyphens).
/// Returns a fallback if the title produces an empty slug.
pub fn slugify(title: &str) -> String {
    let slug = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    if slug.is_empty() {
        "change".to_string()
    } else {
        slug
    }
}

/// Create a change directory with proposal and delta-spec scaffold.
pub fn create_change(
    feature_dir: &Path,
    title: &str,
) -> Result<(String, PathBuf)> {
    let slug = slugify(title);
    if slug.is_empty() {
        bail!("Could not generate a valid change slug from title.");
    }

    let change_dir = feature_dir.join("changes").join(&slug);
    if change_dir.exists() {
        bail!(
            "Change '{}' already exists. Choose a different title or archive the existing change.",
            slug
        );
    }

    std::fs::create_dir_all(&change_dir)?;

    // Write proposal.md
    let proposal = format!(
        "# Proposal: {title}\n\n## Why\n\n<!-- Explain the motivation for this change. -->\n\n## What Changes\n\n<!-- Describe what will change at a high level. -->\n\n## Impact\n\n<!-- Which parts of the system are affected? -->\n\n## Non-Goals\n\n<!-- What is explicitly NOT included. -->\n"
    );
    std::fs::write(change_dir.join("proposal.md"), proposal)?;

    // Write delta-spec.md
    let delta_spec = format!(
        "# Delta Spec: {title}\n\n## Added Requirements\n\n<!-- - **FR-###**: New requirement description -->\n\n## Modified Requirements\n\n<!-- - **FR-###**: Updated text (was: original text) -->\n\n## Removed Requirements\n\n<!-- - FR-### -->\n"
    );
    std::fs::write(change_dir.join("delta-spec.md"), delta_spec)?;

    // Write metadata
    let meta = ChangeMetadata::new(&slug, title);
    meta.save(&change_dir.join(".change.yaml"))?;

    Ok((slug, change_dir))
}

/// Archive a change: merge deltas into main spec, move to archive/.
pub fn archive_change(feature_dir: &Path, slug: &str) -> Result<()> {
    let change_dir = feature_dir.join("changes").join(slug);
    if !change_dir.exists() {
        bail!("Change '{}' not found.", slug);
    }

    let delta_path = change_dir.join("delta-spec.md");
    if !delta_path.exists() {
        bail!("Change '{}' has no delta-spec.md to archive.", slug);
    }

    let delta_content = std::fs::read_to_string(&delta_path)?;
    let delta = parse_delta_spec(&delta_content);

    let main_spec_path = feature_dir.join("spec.md");
    if main_spec_path.exists() {
        let main_content = std::fs::read_to_string(&main_spec_path)?;
        let merged = merge_deltas(&main_content, &delta)?;
        std::fs::write(&main_spec_path, merged)?;
    } else if !delta.added.is_empty() || !delta.modified.is_empty() {
        // No main spec — create one from the delta
        let mut spec = format!("# Feature Specification: {}\n\n", slug);
        spec.push_str("## Requirements\n\n### Functional Requirements\n\n");
        for req in &delta.added {
            spec.push_str(&format!("- **{}**: {}\n", req.id, req.text));
        }
        for mod_req in &delta.modified {
            spec.push_str(&format!("- **{}**: {}\n", mod_req.id, mod_req.new_text));
        }
        std::fs::write(&main_spec_path, spec)?;
    }

    // Move to archive
    let archive_dir = feature_dir.join("changes/archive");
    std::fs::create_dir_all(&archive_dir)?;
    let dest = archive_dir.join(slug);
    let mut backup_path: Option<PathBuf> = None;
    if dest.exists() {
        // Rename existing to backup before overwriting
        let backup = archive_dir.join(format!("{}.bak", slug));
        if backup.exists() {
            std::fs::remove_dir_all(&backup).ok();
        }
        std::fs::rename(&dest, &backup)
            .with_context(|| format!("Failed to backup existing archive '{}'", slug))?;
        backup_path = Some(backup);
    }

    match std::fs::rename(&change_dir, &dest) {
        Ok(()) => {
            // Success — clean up backup
            if let Some(bk) = backup_path {
                std::fs::remove_dir_all(&bk).ok();
            }
        }
        Err(e) => {
            // Restore original if rename failed
            if let Some(bk) = backup_path {
                std::fs::rename(&bk, &dest).ok();
            }
            return Err(anyhow::anyhow!(
                "Failed to move change '{}' to archive: {e}",
                slug
            ));
        }
    }

    // Update metadata
    let meta_path = dest.join(".change.yaml");
    if let Ok(mut meta) = ChangeMetadata::load(&meta_path) {
        meta.status = ChangeStatus::Archived;
        meta.save(&meta_path).ok();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    const SAMPLE_DELTA: &str = r#"# Delta Spec: Add social login

## Added Requirements

- **FR-042**: System MUST support OAuth2 login via Google
- **FR-043**: System MUST support OAuth2 login via GitHub

## Modified Requirements

- **FR-012**: User profile MUST include OAuth provider (was: email only)

## Removed Requirements

- FR-008
"#;

    #[test]
    fn parse_added_requirements() {
        let delta = parse_delta_spec(SAMPLE_DELTA);
        assert_eq!(delta.added.len(), 2);
        assert_eq!(delta.added[0].id, "FR-042");
        assert!(delta.added[0].text.contains("OAuth2"));
        assert_eq!(delta.added[1].id, "FR-043");
    }

    #[test]
    fn parse_modified_requirements() {
        let delta = parse_delta_spec(SAMPLE_DELTA);
        assert_eq!(delta.modified.len(), 1);
        assert_eq!(delta.modified[0].id, "FR-012");
        assert!(delta.modified[0].new_text.contains("OAuth provider"));
        assert_eq!(
            delta.modified[0].previous_text.as_deref(),
            Some("email only")
        );
    }

    #[test]
    fn parse_removed_requirements() {
        let delta = parse_delta_spec(SAMPLE_DELTA);
        assert_eq!(delta.removed, vec!["FR-008"]);
    }

    #[test]
    fn merge_deltas_adds_new_requirements() {
        let main = "- **FR-001**: System MUST do something\n- **FR-002**: System MUST do another thing\n";
        let delta = DeltaSpec {
            added: vec![DeltaRequirement {
                id: "FR-042".into(),
                text: "OAuth login".into(),
            }],
            modified: vec![],
            removed: vec![],
            raw: String::new(),
        };
        let merged = merge_deltas(main, &delta).unwrap();
        assert!(merged.contains("FR-042"));
        assert!(merged.contains("OAuth login"));
        // Original requirements preserved
        assert!(merged.contains("FR-001"));
        assert!(merged.contains("FR-002"));
    }

    #[test]
    fn merge_deltas_removes_requirements() {
        let main = "- **FR-001**: keep\n- **FR-008**: remove me\n- **FR-002**: keep too\n";
        let delta = DeltaSpec {
            added: vec![],
            modified: vec![],
            removed: vec!["FR-008".into()],
            raw: String::new(),
        };
        let merged = merge_deltas(main, &delta).unwrap();
        assert!(!merged.contains("FR-008"));
        assert!(merged.contains("FR-001"));
        assert!(merged.contains("FR-002"));
    }

    #[test]
    fn merge_deltas_modifies_existing() {
        let main = "- **FR-012**: email only\n- **FR-001**: something\n";
        let delta = DeltaSpec {
            added: vec![],
            modified: vec![DeltaModification {
                id: "FR-012".into(),
                new_text: "email and OAuth provider".into(),
                previous_text: Some("email only".into()),
            }],
            removed: vec![],
            raw: String::new(),
        };
        let merged = merge_deltas(main, &delta).unwrap();
        assert!(merged.contains("OAuth provider"));
        assert!(!merged.contains("email only"));
    }

    #[test]
    fn empty_delta_parsed() {
        let delta = parse_delta_spec("# Empty\n\nNo sections here.\n");
        assert!(delta.added.is_empty());
        assert!(delta.modified.is_empty());
        assert!(delta.removed.is_empty());
    }

    #[test]
    fn slugify_produces_valid_slugs() {
        assert_eq!(slugify("Add Social Login"), "add-social-login");
        assert_eq!(slugify("Fix: OAuth 2.0 bugs!"), "fix-oauth-2-0-bugs");
        assert!(!slugify("!!!").is_empty());
    }

    #[test]
    fn create_and_archive_change_roundtrip() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();
        std::fs::write(
            feature.join("spec.md"),
            "- **FR-001**: keep me\n- **FR-008**: delete me\n",
        )
        .unwrap();

        // Create a change
        let (slug, change_dir) = create_change(&feature, "Delete FR-008").unwrap();
        assert!(slug.contains("delete-fr-008"));
        assert!(change_dir.join("proposal.md").exists());
        assert!(change_dir.join("delta-spec.md").exists());
        assert!(change_dir.join(".change.yaml").exists());

        // Write a delta spec
        std::fs::write(
            change_dir.join("delta-spec.md"),
            "## Removed Requirements\n\n- FR-008\n",
        )
        .unwrap();

        // Archive it
        archive_change(&feature, &slug).unwrap();

        // Verify main spec was updated
        let updated_spec = std::fs::read_to_string(feature.join("spec.md")).unwrap();
        assert!(!updated_spec.contains("FR-008"));
        assert!(updated_spec.contains("FR-001"));

        // Verify change was moved to archive
        assert!(!change_dir.exists());
        assert!(feature.join("changes/archive").join(&slug).exists());
    }

    #[test]
    fn list_changes_finds_all() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();

        create_change(&feature, "First change").unwrap();
        create_change(&feature, "Second change").unwrap();

        let changes = list_changes(&feature).unwrap();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].status, ChangeStatus::Proposed);
    }

    #[test]
    fn list_changes_skips_archive_directory() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();

        create_change(&feature, "Test change").unwrap();
        // Simulate an archive directory existing
        std::fs::create_dir_all(feature.join("changes/archive/old-change")).unwrap();

        let changes = list_changes(&feature).unwrap();
        assert_eq!(changes.len(), 1, "archive dir should not appear in changes list");
        assert_eq!(changes[0].title, "Test change");
    }

    #[test]
    fn archive_nonexistent_change_errors() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();
        assert!(archive_change(&feature, "nonexistent").is_err());
    }
}
