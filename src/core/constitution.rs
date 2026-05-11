#![allow(dead_code)]
use std::path::Path;

use anyhow::Result;

use super::errors::SolidSpecError;

#[derive(Debug, Clone)]
pub struct Constitution {
    pub raw: String,
    pub gates: Vec<Gate>,
}

#[derive(Debug, Clone)]
pub struct Gate {
    pub name: String,
    pub article: String,
    pub checks: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GateResult {
    pub gate_name: String,
    pub passed: bool,
    pub violations: Vec<String>,
}

pub fn load_constitution(path: &Path) -> Result<Constitution> {
    let content = std::fs::read_to_string(path).map_err(|_| SolidSpecError::Config {
        path: path.to_path_buf(),
        message: "Constitution file not found".into(),
        fix: format!(
            "Ensure {} exists. Run 'solidspec init' to create it.",
            path.display()
        ),
    })?;

    parse_constitution(&content)
}

pub fn parse_constitution(content: &str) -> Result<Constitution> {
    let mut gates = Vec::new();

    // Extract gates from articles
    if content.contains("Article VII") || content.contains("Simplicity") {
        gates.push(Gate {
            name: "Simplicity Gate".into(),
            article: "Article VII".into(),
            checks: vec![
                "Maximum 3 projects for initial implementation".into(),
                "No speculative or might-need features".into(),
            ],
        });
    }

    if content.contains("Article VIII") || content.contains("Anti-Abstraction") {
        gates.push(Gate {
            name: "Anti-Abstraction Gate".into(),
            article: "Article VIII".into(),
            checks: vec![
                "Use framework features directly".into(),
                "Single model representation".into(),
            ],
        });
    }

    if content.contains("Article IX") || content.contains("Integration-First") {
        gates.push(Gate {
            name: "Integration-First Gate".into(),
            article: "Article IX".into(),
            checks: vec![
                "Contract tests mandatory before implementation".into(),
                "Prefer real services over mocks".into(),
            ],
        });
    }

    Ok(Constitution {
        raw: content.to_string(),
        gates,
    })
}

/// Remove the "Constitution Check" section from plan content to avoid
/// false positives from the gate checklist text itself.
fn strip_constitution_section(content: &str) -> String {
    let mut result = String::new();
    let mut in_constitution_section = false;

    for line in content.lines() {
        if line.starts_with("## Constitution Check") {
            in_constitution_section = true;
            continue;
        }
        if in_constitution_section && line.starts_with("## ") {
            in_constitution_section = false;
        }
        if !in_constitution_section {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Check if a plan violates any constitution gates.
pub fn check_plan_compliance(constitution: &Constitution, plan_content: &str) -> Vec<GateResult> {
    let mut results = Vec::new();

    // Strip the "Constitution Check" section from analysis to avoid false positives
    // from the gate checklist text itself
    let plan_for_analysis = strip_constitution_section(plan_content);
    let plan_lower = plan_for_analysis.to_lowercase();

    for gate in &constitution.gates {
        let mut violations = Vec::new();

        match gate.name.as_str() {
            "Simplicity Gate" => {
                // Check for too many projects
                if plan_lower.contains("project 4") || plan_lower.contains("4th project") {
                    violations.push("More than 3 projects detected".into());
                }
                if plan_lower.contains("future-proof") || plan_lower.contains("might need") {
                    violations.push("Future-proofing language detected".into());
                }
            }
            "Anti-Abstraction Gate"
                if plan_lower.contains("wrapper") || plan_lower.contains("abstraction layer") =>
            {
                violations.push("Wrapper/abstraction layer detected".into());
            }
            "Integration-First Gate"
                if plan_lower.contains("mock") && !plan_lower.contains("contract") =>
            {
                violations.push("Mocks used without contract tests".into());
            }
            _ => {}
        }

        results.push(GateResult {
            gate_name: gate.name.clone(),
            passed: violations.is_empty(),
            violations,
        });
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    const SAMPLE_CONSTITUTION: &str = r#"# Project Constitution

## Core Principles

### Article VII: Simplicity
- Maximum 3 projects for initial implementation
- No speculative features

### Article VIII: Anti-Abstraction
- Use framework features directly
- Single model representation

### Article IX: Integration-First
- Contract tests mandatory before implementation
- Prefer real services over mocks
"#;

    #[test]
    fn load_valid_constitution_extracts_all_gates() {
        let constitution = parse_constitution(SAMPLE_CONSTITUTION).unwrap();
        assert_eq!(constitution.gates.len(), 3);
        assert_eq!(constitution.gates[0].name, "Simplicity Gate");
        assert_eq!(constitution.gates[1].name, "Anti-Abstraction Gate");
        assert_eq!(constitution.gates[2].name, "Integration-First Gate");
    }

    #[test]
    fn missing_constitution_returns_error_with_path() {
        let result = load_constitution(Path::new("/nonexistent/constitution.md"));
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Constitution file not found"));
    }

    #[test]
    fn gate_evaluation_passes_when_no_violations() {
        let constitution = parse_constitution(SAMPLE_CONSTITUTION).unwrap();
        let plan = "Simple plan using framework directly with contract tests.";
        let results = check_plan_compliance(&constitution, plan);
        assert!(results.iter().all(|r| r.passed));
    }

    #[test]
    fn gate_evaluation_fails_with_violation_details() {
        let constitution = parse_constitution(SAMPLE_CONSTITUTION).unwrap();
        let plan = "We should future-proof by adding a wrapper abstraction layer.";
        let results = check_plan_compliance(&constitution, plan);
        let failed: Vec<_> = results.iter().filter(|r| !r.passed).collect();
        assert!(!failed.is_empty());
        assert!(failed.iter().any(|r| !r.violations.is_empty()));
    }

    #[test]
    fn custom_constitution_with_only_simplicity() {
        let content = "### Article VII: Simplicity\n- Max 3 projects\n";
        let constitution = parse_constitution(content).unwrap();
        assert_eq!(constitution.gates.len(), 1);
        assert_eq!(constitution.gates[0].name, "Simplicity Gate");
    }

    #[test]
    fn load_from_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("constitution.md");
        std::fs::write(&path, SAMPLE_CONSTITUTION).unwrap();
        let constitution = load_constitution(&path).unwrap();
        assert_eq!(constitution.gates.len(), 3);
    }
}
