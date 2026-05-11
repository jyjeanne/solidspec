use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;

use super::errors::SolidSpecError;

static FEATURE_DIR_PREFIX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d{3})-").expect("invalid feature dir regex"));
static FEATURE_DIR_FULL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d{3})-.+$").expect("invalid feature dir full regex"));
static FEATURE_BRANCH_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\d{3}-.+$").expect("invalid feature branch regex"));

/// Scan `specs/` and return the next global feature number (highest + 1).
pub fn next_feature_number(specs_dir: &Path) -> Result<u32> {
    if !specs_dir.exists() {
        return Ok(1);
    }

    let mut max_num: u32 = 0;

    let entries = std::fs::read_dir(specs_dir).map_err(|e| SolidSpecError::Feature {
        message: format!("Cannot read specs directory: {e}"),
        fix: "Ensure 'specs/' directory exists and is readable.".into(),
    })?;

    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if let Some(caps) = FEATURE_DIR_PREFIX_RE.captures(&name_str)
                && let Ok(num) = caps[1].parse::<u32>()
            {
                max_num = max_num.max(num);
            }
        }
    }

    if max_num >= 999 {
        return Err(SolidSpecError::Feature {
            message: "Feature number overflow: all 999 IDs used".into(),
            fix: "Archive old features to free up numbers.".into(),
        }
        .into());
    }
    Ok(max_num + 1)
}

/// Format a feature number as zero-padded 3 digits.
pub fn format_feature_id(num: u32) -> String {
    format!("{num:03}")
}

/// Generate a short branch name from a description (2-4 words, action-noun).
pub fn generate_branch_name(description: &str) -> Result<String> {
    let description = description.trim();
    if description.is_empty() {
        return Err(SolidSpecError::Validation {
            message: "Feature description must not be empty.".into(),
        }
        .into());
    }

    let words: Vec<&str> = description
        .split_whitespace()
        .filter(|w| w.len() > 1 || w.chars().all(|c| c.is_alphabetic()))
        .take(5)
        .collect();

    let slug = if words.is_empty() {
        // Fallback: use the raw description, sanitized
        sanitize(description)
    } else {
        words
            .iter()
            .map(|w| sanitize(w))
            .collect::<Vec<_>>()
            .join("-")
    };

    if slug.is_empty() {
        return Err(SolidSpecError::Validation {
            message: "Could not generate a valid branch name from description.".into(),
        }
        .into());
    }

    Ok(slug)
}

fn sanitize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .to_lowercase()
}

/// Validate that a branch name matches the `\d{3}-.*` pattern.
pub fn is_valid_feature_branch(name: &str) -> bool {
    FEATURE_BRANCH_RE.is_match(name)
}

/// 4-level feature resolution:
/// 1. Explicit feature-id argument
/// 2. SOLIDSPEC_FEATURE env var
/// 3. Current Git branch (if matches \d{3}-.* pattern)
/// 4. Latest feature directory in specs/ (by numeric prefix)
pub fn resolve_feature(explicit_id: Option<&str>, project_root: &Path) -> Result<String> {
    // Level 1: explicit argument
    if let Some(id) = explicit_id {
        let id = id.trim();
        if !id.is_empty() {
            log::debug!("Feature resolved via explicit argument: '{id}'");
            return find_feature_dir_by_prefix(&project_root.join("specs"), id);
        }
    }

    // Level 2: env var
    if let Ok(env_feature) = std::env::var("SOLIDSPEC_FEATURE") {
        let env_feature = env_feature.trim().to_string();
        if !env_feature.is_empty() {
            log::debug!("Feature resolved via SOLIDSPEC_FEATURE env var: '{env_feature}'");
            return find_feature_dir_by_prefix(&project_root.join("specs"), &env_feature);
        }
    }

    // Level 3: git branch
    if let Some(branch) = super::git::current_branch(project_root)
        && is_valid_feature_branch(&branch)
    {
        let prefix = &branch[..3]; // extract numeric prefix
        log::debug!("Feature resolved via git branch: '{branch}' → prefix '{prefix}'");
        return find_feature_dir_by_prefix(&project_root.join("specs"), prefix);
    }

    // Level 4: latest specs/ directory
    log::debug!("Feature resolved via latest specs/ directory (fallback)");
    latest_feature_dir(&project_root.join("specs"))
}

/// Find a feature directory matching a prefix or exact name.
pub fn find_feature_dir_by_prefix(specs_dir: &Path, prefix: &str) -> Result<String> {
    if !specs_dir.exists() {
        return Err(SolidSpecError::Feature {
            message: "No specs/ directory found".into(),
            fix: "Run 'solidspec init' and 'solidspec specify' first.".into(),
        }
        .into());
    }

    // Try exact directory name match first
    if specs_dir.join(prefix).is_dir() {
        return Ok(prefix.to_string());
    }

    let mut matches = Vec::new();

    for entry in std::fs::read_dir(specs_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(caps) = FEATURE_DIR_FULL_RE.captures(&name) {
                // Match only on the numeric prefix (3 digits), not free-form starts_with
                if caps[1] == *prefix {
                    matches.push(name);
                }
            }
        }
    }

    match matches.len() {
        0 => Err(SolidSpecError::Feature {
            message: format!("No feature matching '{prefix}' found in specs/"),
            fix: "Check feature ID with 'ls specs/' or run 'solidspec specify'.".into(),
        }
        .into()),
        1 => Ok(matches.into_iter().next().unwrap()),
        _ => {
            matches.sort();
            // If all matches share the same 3-digit prefix, pick the latest
            let first_prefix = &matches[0][..3.min(matches[0].len())];
            let all_same_prefix = matches.iter().all(|m| m.starts_with(first_prefix));
            if all_same_prefix {
                Ok(matches.last().unwrap().clone())
            } else {
                Err(SolidSpecError::Feature {
                    message: format!(
                        "Ambiguous prefix '{}'. Matches: {}",
                        prefix,
                        matches.join(", ")
                    ),
                    fix: "Use the full feature directory name or numeric prefix (e.g., '001')."
                        .into(),
                }
                .into())
            }
        }
    }
}

/// Get the latest (highest numbered) feature directory.
fn latest_feature_dir(specs_dir: &Path) -> Result<String> {
    if !specs_dir.exists() {
        return Err(SolidSpecError::Feature {
            message: "No specs/ directory found".into(),
            fix: "Run 'solidspec init' and 'solidspec specify' first.".into(),
        }
        .into());
    }

    let mut best: Option<(u32, String)> = None;

    for entry in std::fs::read_dir(specs_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(caps) = FEATURE_DIR_FULL_RE.captures(&name)
                && let Ok(num) = caps[1].parse::<u32>()
                && best.as_ref().is_none_or(|(n, _)| num > *n)
            {
                best = Some((num, name));
            }
        }
    }

    best.map(|(_, name)| name).ok_or_else(|| {
        SolidSpecError::Feature {
            message: "No feature directories found in specs/".into(),
            fix: "Run 'solidspec specify' to create a feature.".into(),
        }
        .into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn empty_specs_returns_001() {
        let dir = TempDir::new().unwrap();
        let specs = dir.path().join("specs");
        std::fs::create_dir_all(&specs).unwrap();
        assert_eq!(next_feature_number(&specs).unwrap(), 1);
    }

    #[test]
    fn nonexistent_specs_returns_001() {
        let dir = TempDir::new().unwrap();
        let specs = dir.path().join("specs");
        assert_eq!(next_feature_number(&specs).unwrap(), 1);
    }

    #[test]
    fn existing_001_002_returns_003() {
        let dir = TempDir::new().unwrap();
        let specs = dir.path().join("specs");
        std::fs::create_dir_all(specs.join("001-auth")).unwrap();
        std::fs::create_dir_all(specs.join("002-chat")).unwrap();
        assert_eq!(next_feature_number(&specs).unwrap(), 3);
    }

    #[test]
    fn non_sequential_gaps_use_global_max_plus_one() {
        let dir = TempDir::new().unwrap();
        let specs = dir.path().join("specs");
        std::fs::create_dir_all(specs.join("001-auth")).unwrap();
        std::fs::create_dir_all(specs.join("003-payments")).unwrap();
        // Should be 004, not 002
        assert_eq!(next_feature_number(&specs).unwrap(), 4);
    }

    #[test]
    fn ignores_non_matching_dirs() {
        let dir = TempDir::new().unwrap();
        let specs = dir.path().join("specs");
        std::fs::create_dir_all(specs.join("001-auth")).unwrap();
        std::fs::create_dir_all(specs.join("not-a-feature")).unwrap();
        std::fs::create_dir_all(specs.join("abc")).unwrap();
        assert_eq!(next_feature_number(&specs).unwrap(), 2);
    }

    #[test]
    fn format_feature_id_zero_pads() {
        assert_eq!(format_feature_id(1), "001");
        assert_eq!(format_feature_id(42), "042");
        assert_eq!(format_feature_id(100), "100");
    }

    #[test]
    fn generate_branch_name_from_description() {
        let name = generate_branch_name("Real-time chat with message history").unwrap();
        assert!(!name.is_empty());
        assert!(!name.contains(' '));
        // Should be lowercase with hyphens
        assert!(name.chars().all(|c| c.is_alphanumeric() || c == '-'));
    }

    #[test]
    fn empty_description_returns_error() {
        assert!(generate_branch_name("").is_err());
        assert!(generate_branch_name("   ").is_err());
    }

    #[test]
    fn is_valid_feature_branch_matches_pattern() {
        assert!(is_valid_feature_branch("001-auth"));
        assert!(is_valid_feature_branch("042-chat-system"));
        assert!(!is_valid_feature_branch("main"));
        assert!(!is_valid_feature_branch("01-short"));
        assert!(!is_valid_feature_branch("001"));
    }

    // P2-16: Feature branch detection tests
    #[test]
    fn resolve_explicit_arg_wins() {
        let dir = TempDir::new().unwrap();
        let specs = dir.path().join("specs");
        std::fs::create_dir_all(specs.join("001-auth")).unwrap();
        std::fs::create_dir_all(specs.join("002-chat")).unwrap();

        let result = resolve_feature(Some("001"), dir.path()).unwrap();
        assert!(result.starts_with("001"));
    }

    // NOTE: env var tests run sequentially to avoid race conditions.
    // We use a single test that covers both env var and fallback scenarios.
    #[test]
    fn resolve_env_var_and_latest_fallback() {
        // Clean state
        unsafe { std::env::remove_var("SOLIDSPEC_FEATURE") };

        // Part 1: env var wins
        let dir = TempDir::new().unwrap();
        let specs = dir.path().join("specs");
        std::fs::create_dir_all(specs.join("001-auth")).unwrap();
        std::fs::create_dir_all(specs.join("002-chat")).unwrap();

        unsafe { std::env::set_var("SOLIDSPEC_FEATURE", "001") };
        let result = resolve_feature(None, dir.path()).unwrap();
        assert!(result.starts_with("001"), "Env var should win: {result}");
        unsafe { std::env::remove_var("SOLIDSPEC_FEATURE") };

        // Part 2: fallback to latest (env var removed)
        let dir2 = TempDir::new().unwrap();
        let specs2 = dir2.path().join("specs");
        std::fs::create_dir_all(specs2.join("001-auth")).unwrap();
        std::fs::create_dir_all(specs2.join("003-payments")).unwrap();

        let result = resolve_feature(None, dir2.path()).unwrap();
        assert!(result.starts_with("003"), "Should pick latest: {result}");
    }

    #[test]
    fn resolve_empty_specs_returns_error() {
        let dir = TempDir::new().unwrap();
        let specs = dir.path().join("specs");
        std::fs::create_dir_all(&specs).unwrap();

        assert!(resolve_feature(None, dir.path()).is_err());
    }

    #[test]
    fn find_feature_dir_single_match() {
        let dir = TempDir::new().unwrap();
        let specs = dir.path().join("specs");
        std::fs::create_dir_all(specs.join("001-auth")).unwrap();

        let result = find_feature_dir_by_prefix(&specs, "001").unwrap();
        assert_eq!(result, "001-auth");
    }

    #[test]
    fn find_feature_dir_no_match() {
        let dir = TempDir::new().unwrap();
        let specs = dir.path().join("specs");
        std::fs::create_dir_all(specs.join("001-auth")).unwrap();

        assert!(find_feature_dir_by_prefix(&specs, "099").is_err());
    }

    #[test]
    fn find_feature_dir_multiple_picks_latest() {
        let dir = TempDir::new().unwrap();
        let specs = dir.path().join("specs");
        std::fs::create_dir_all(specs.join("001-auth-v1")).unwrap();
        std::fs::create_dir_all(specs.join("001-auth-v2")).unwrap();

        let result = find_feature_dir_by_prefix(&specs, "001").unwrap();
        assert_eq!(result, "001-auth-v2");
    }
}
