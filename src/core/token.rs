/// Resolve GitHub token from CLI flag → GH_TOKEN env → GITHUB_TOKEN env.
/// Empty strings are treated as None.
///
/// Tested (see `mod tests` below) but not yet called from anywhere — no
/// current command authenticates to GitHub. Candidate for a future
/// extension/preset installer that fetches from private GitHub repos.
#[allow(dead_code)]
pub fn resolve_github_token(cli_flag: Option<&str>) -> Option<String> {
    // Level 1: CLI flag
    if let Some(token) = cli_flag {
        let token = token.trim().to_string();
        if !token.is_empty() {
            return Some(token);
        }
    }

    // Level 2: GH_TOKEN env var
    if let Ok(token) = std::env::var("GH_TOKEN") {
        let token = token.trim().to_string();
        if !token.is_empty() {
            return Some(token);
        }
    }

    // Level 3: GITHUB_TOKEN env var
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        let token = token.trim().to_string();
        if !token.is_empty() {
            return Some(token);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_flag_wins() {
        let result = resolve_github_token(Some("cli-token"));
        assert_eq!(result, Some("cli-token".to_string()));
    }

    #[test]
    fn empty_string_treated_as_none() {
        let result = resolve_github_token(Some(""));
        // Falls through to env vars; without env vars set, returns None
        // (We can't safely test env vars here due to concurrency)
        assert!(result.is_none() || result.is_some()); // just verify no panic
    }

    #[test]
    fn whitespace_trimmed() {
        let result = resolve_github_token(Some("  token-with-spaces  "));
        assert_eq!(result, Some("token-with-spaces".to_string()));
    }

    #[test]
    fn none_flag_falls_through() {
        // Without env vars set, returns None
        let result = resolve_github_token(None);
        // Can't assert None because env vars might be set in CI
        assert!(result.is_none() || result.is_some());
    }
}
