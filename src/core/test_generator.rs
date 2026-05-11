#![allow(dead_code)]
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use super::spec_parser::ParsedSpec;

static WHEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*When\*\*").expect("invalid when regex"));
static THEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Then\*\*").expect("invalid then regex"));
static UNDERSCORE_COLLAPSE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"_+").expect("invalid underscore regex"));

// ── Acceptance Scenario ──────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AcceptanceScenario {
    pub given: String,
    pub when: String,
    pub then: String,
    pub story_index: usize, // 1-based
    pub story_title: String,
    pub story_priority: String,
}

/// Extract Given/When/Then triples from all user stories in a parsed spec.
pub fn extract_scenarios(spec: &ParsedSpec) -> Vec<AcceptanceScenario> {
    let mut scenarios = Vec::new();

    for (idx, story) in spec.user_stories.iter().enumerate() {
        for raw in &story.acceptance_scenarios {
            if let Some(scenario) =
                parse_single_scenario(raw, idx + 1, &story.title, &story.priority)
            {
                scenarios.push(scenario);
            } else {
                log::warn!("Skipping unparseable scenario in US{}: {}", idx + 1, raw);
            }
        }
    }

    scenarios
}

/// Parse a single scenario line: split on **When** and **Then** markers.
fn parse_single_scenario(
    raw: &str,
    story_index: usize,
    story_title: &str,
    story_priority: &str,
) -> Option<AcceptanceScenario> {
    // Expected format: "the app is open, **When** user types..., **Then** task appears..."
    // The raw string already has **Given** stripped by the spec parser

    let when_pos = WHEN_RE.find(raw)?;
    let then_pos = THEN_RE.find(raw)?;

    if then_pos.start() <= when_pos.end() {
        return None; // Then before When — invalid
    }

    let given = raw[..when_pos.start()]
        .trim()
        .trim_end_matches(',')
        .trim()
        .to_string();
    let when = raw[when_pos.end()..then_pos.start()]
        .trim()
        .trim_end_matches(',')
        .trim()
        .to_string();
    let then = raw[then_pos.end()..]
        .trim()
        .trim_end_matches(',')
        .trim()
        .to_string();

    if given.is_empty() || when.is_empty() || then.is_empty() {
        return None;
    }

    Some(AcceptanceScenario {
        given,
        when,
        then,
        story_index,
        story_title: story_title.to_string(),
        story_priority: story_priority.to_string(),
    })
}

// ── Test Framework Detection ─────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameworkId {
    Jest,
    Vitest,
    Mocha,
    Pytest,
    CargoTest,
    GoTest,
    Generic,
}

#[derive(Debug, Clone)]
pub struct TestFramework {
    pub id: FrameworkId,
    pub name: String,
    pub language: String,
    pub file_extension: String,
    pub slug_style: SlugStyle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlugStyle {
    Underscores, // Rust, Python, Go: spaces → _
    Preserved,   // Jest/Vitest/Mocha: keep spaces in string
}

/// Detect the test framework from project files.
pub fn detect_framework(project_root: &Path) -> TestFramework {
    let has_tsconfig = project_root.join("tsconfig.json").exists();
    let pkg_json = project_root.join("package.json");

    if pkg_json.exists()
        && let Ok(content) = std::fs::read_to_string(&pkg_json)
    {
        let ext = if has_tsconfig { ".test.ts" } else { ".test.js" };
        let lang = if has_tsconfig {
            "TypeScript"
        } else {
            "JavaScript"
        };

        if content.contains("\"vitest\"") {
            return TestFramework {
                id: FrameworkId::Vitest,
                name: "Vitest".into(),
                language: lang.into(),
                file_extension: ext.into(),
                slug_style: SlugStyle::Preserved,
            };
        }
        if content.contains("\"jest\"") {
            return TestFramework {
                id: FrameworkId::Jest,
                name: "Jest".into(),
                language: lang.into(),
                file_extension: ext.into(),
                slug_style: SlugStyle::Preserved,
            };
        }
        if content.contains("\"mocha\"") {
            return TestFramework {
                id: FrameworkId::Mocha,
                name: "Mocha".into(),
                language: lang.into(),
                file_extension: ext.into(),
                slug_style: SlugStyle::Preserved,
            };
        }
        // Default JS framework
        return TestFramework {
            id: FrameworkId::Jest,
            name: "Jest".into(),
            language: lang.into(),
            file_extension: ext.into(),
            slug_style: SlugStyle::Preserved,
        };
    }

    if has_tsconfig {
        return TestFramework {
            id: FrameworkId::Jest,
            name: "Jest".into(),
            language: "TypeScript".into(),
            file_extension: ".test.ts".into(),
            slug_style: SlugStyle::Preserved,
        };
    }

    if project_root.join("Cargo.toml").exists() {
        return TestFramework {
            id: FrameworkId::CargoTest,
            name: "cargo test".into(),
            language: "Rust".into(),
            file_extension: ".rs".into(),
            slug_style: SlugStyle::Underscores,
        };
    }

    if project_root.join("pyproject.toml").exists()
        || project_root.join("requirements.txt").exists()
        || project_root.join("setup.py").exists()
    {
        return TestFramework {
            id: FrameworkId::Pytest,
            name: "pytest".into(),
            language: "Python".into(),
            file_extension: "_test.py".into(),
            slug_style: SlugStyle::Underscores,
        };
    }

    if project_root.join("go.mod").exists() {
        return TestFramework {
            id: FrameworkId::GoTest,
            name: "go test".into(),
            language: "Go".into(),
            file_extension: "_test.go".into(),
            slug_style: SlugStyle::Underscores,
        };
    }

    TestFramework {
        id: FrameworkId::Generic,
        name: "Generic".into(),
        language: "pseudocode".into(),
        file_extension: ".test.txt".into(),
        slug_style: SlugStyle::Underscores,
    }
}

/// Parse a framework name string (from --framework flag) into a TestFramework.
pub fn framework_from_name(name: &str) -> Option<TestFramework> {
    match name.to_lowercase().as_str() {
        "jest" => Some(TestFramework {
            id: FrameworkId::Jest,
            name: "Jest".into(),
            language: "JavaScript".into(),
            file_extension: ".test.js".into(),
            slug_style: SlugStyle::Preserved,
        }),
        "vitest" => Some(TestFramework {
            id: FrameworkId::Vitest,
            name: "Vitest".into(),
            language: "JavaScript".into(),
            file_extension: ".test.js".into(),
            slug_style: SlugStyle::Preserved,
        }),
        "mocha" => Some(TestFramework {
            id: FrameworkId::Mocha,
            name: "Mocha".into(),
            language: "JavaScript".into(),
            file_extension: ".test.js".into(),
            slug_style: SlugStyle::Preserved,
        }),
        "pytest" => Some(TestFramework {
            id: FrameworkId::Pytest,
            name: "pytest".into(),
            language: "Python".into(),
            file_extension: "_test.py".into(),
            slug_style: SlugStyle::Underscores,
        }),
        "cargo" => Some(TestFramework {
            id: FrameworkId::CargoTest,
            name: "cargo test".into(),
            language: "Rust".into(),
            file_extension: ".rs".into(),
            slug_style: SlugStyle::Underscores,
        }),
        "go" => Some(TestFramework {
            id: FrameworkId::GoTest,
            name: "go test".into(),
            language: "Go".into(),
            file_extension: "_test.go".into(),
            slug_style: SlugStyle::Underscores,
        }),
        "generic" => Some(TestFramework {
            id: FrameworkId::Generic,
            name: "Generic".into(),
            language: "pseudocode".into(),
            file_extension: ".test.txt".into(),
            slug_style: SlugStyle::Underscores,
        }),
        _ => None,
    }
}

// ── Slugification ────────────────────────────────────────────────────

pub fn slugify(text: &str, style: &SlugStyle) -> String {
    match style {
        SlugStyle::Preserved => text.trim().to_string(),
        SlugStyle::Underscores => {
            let slug: String = text
                .to_lowercase()
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect();
            let slug = slug.trim_matches('_').to_string();
            // Collapse consecutive underscores
            let slug = UNDERSCORE_COLLAPSE_RE.replace_all(&slug, "_").to_string();
            // Max 80 chars (safe truncation on char boundary)
            if slug.len() > 80 {
                let end = slug
                    .char_indices()
                    .take_while(|(i, _)| *i < 80)
                    .last()
                    .map(|(i, c)| i + c.len_utf8())
                    .unwrap_or(80);
                slug[..end].trim_end_matches('_').to_string()
            } else {
                slug
            }
        }
    }
}

// ── Test Scaffold Generation ─────────────────────────────────────────

/// Generate test file content for a list of scenarios belonging to one user story.
pub fn render_test_file(
    feature_name: &str,
    story_index: usize,
    story_title: &str,
    story_priority: &str,
    scenarios: &[AcceptanceScenario],
    framework: &TestFramework,
) -> String {
    match framework.id {
        FrameworkId::Jest | FrameworkId::Vitest | FrameworkId::Mocha => render_jest(
            feature_name,
            story_index,
            story_title,
            story_priority,
            scenarios,
        ),
        FrameworkId::Pytest => render_pytest(
            feature_name,
            story_index,
            story_title,
            story_priority,
            scenarios,
        ),
        FrameworkId::CargoTest => render_cargo(
            feature_name,
            story_index,
            story_title,
            story_priority,
            scenarios,
        ),
        FrameworkId::GoTest => render_go(
            feature_name,
            story_index,
            story_title,
            story_priority,
            scenarios,
        ),
        FrameworkId::Generic => render_generic(
            feature_name,
            story_index,
            story_title,
            story_priority,
            scenarios,
        ),
    }
}

fn render_jest(
    feature: &str,
    idx: usize,
    title: &str,
    priority: &str,
    scenarios: &[AcceptanceScenario],
) -> String {
    let mut out = format!(
        "// Generated by solidspec tests — DO NOT EDIT test structure (fill in bodies)\n\
         // Feature: {feature}\n\
         // User Story {idx}: {title} ({priority})\n\n\
         describe('US{idx}: {title}', () => {{\n"
    );
    for s in scenarios {
        let name = s.then.replace('\'', "\\'");
        out.push_str(&format!(
            "  test('{name}', () => {{\n\
             \x20\x20\x20\x20// Given: {given}\n\
             \x20\x20\x20\x20// When: {when}\n\
             \x20\x20\x20\x20// Then: {then}\n\
             \x20\x20\x20\x20throw new Error('TODO: implement this test');\n\
             \x20\x20}});\n\n",
            given = s.given,
            when = s.when,
            then = s.then,
        ));
    }
    out.push_str("});\n");
    out
}

fn render_pytest(
    feature: &str,
    idx: usize,
    title: &str,
    priority: &str,
    scenarios: &[AcceptanceScenario],
) -> String {
    let class_name = format!("TestUS{}{}", idx, slugify(title, &SlugStyle::Underscores));
    // Capitalize first letter of each word for class name
    let class_name: String = class_name
        .split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect();

    let mut out = format!(
        "# Generated by solidspec tests — DO NOT EDIT test structure (fill in bodies)\n\
         # Feature: {feature}\n\
         # User Story {idx}: {title} ({priority})\n\n\n\
         class {class_name}:\n"
    );
    for s in scenarios {
        let fn_name = slugify(&s.then, &SlugStyle::Underscores);
        out.push_str(&format!(
            "    def test_{fn_name}(self):\n\
             \x20\x20\x20\x20\x20\x20\x20\x20\"\"\"\n\
             \x20\x20\x20\x20\x20\x20\x20\x20Given: {given}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20When: {when}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20Then: {then}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20\"\"\"\n\
             \x20\x20\x20\x20\x20\x20\x20\x20raise NotImplementedError(\"TODO: implement this test\")\n\n",
            given = s.given,
            when = s.when,
            then = s.then,
        ));
    }
    out
}

fn render_cargo(
    feature: &str,
    idx: usize,
    title: &str,
    priority: &str,
    scenarios: &[AcceptanceScenario],
) -> String {
    let mod_name = slugify(&format!("us{idx}_{title}"), &SlugStyle::Underscores);
    let mut out = format!(
        "// Generated by solidspec tests — DO NOT EDIT test structure (fill in bodies)\n\
         // Feature: {feature}\n\
         // User Story {idx}: {title} ({priority})\n\n\
         #[cfg(test)]\n\
         mod {mod_name} {{\n"
    );
    for s in scenarios {
        let fn_name = slugify(&s.then, &SlugStyle::Underscores);
        out.push_str(&format!(
            "    #[test]\n\
             \x20\x20\x20\x20fn {fn_name}() {{\n\
             \x20\x20\x20\x20\x20\x20\x20\x20// Given: {given}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20// When: {when}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20// Then: {then}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20todo!(\"implement this test\");\n\
             \x20\x20\x20\x20}}\n\n",
            given = s.given,
            when = s.when,
            then = s.then,
        ));
    }
    out.push_str("}\n");
    out
}

fn render_go(
    feature: &str,
    idx: usize,
    title: &str,
    priority: &str,
    scenarios: &[AcceptanceScenario],
) -> String {
    let pkg_name = slugify(&format!("us{idx}_{title}"), &SlugStyle::Underscores);
    let mut out = format!(
        "// Generated by solidspec tests — DO NOT EDIT test structure (fill in bodies)\n\
         // Feature: {feature}\n\
         // User Story {idx}: {title} ({priority})\n\n\
         package {pkg_name}\n\n\
         import \"testing\"\n\n"
    );
    for s in scenarios {
        // Go test functions must be PascalCase
        let fn_name = slugify(&s.then, &SlugStyle::Underscores);
        let fn_name: String = fn_name
            .split('_')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect();
        out.push_str(&format!(
            "func Test{fn_name}(t *testing.T) {{\n\
             \t// Given: {given}\n\
             \t// When: {when}\n\
             \t// Then: {then}\n\
             \tt.Fatal(\"TODO: implement this test\")\n\
             }}\n\n",
            given = s.given,
            when = s.when,
            then = s.then,
        ));
    }
    out
}

fn render_generic(
    feature: &str,
    idx: usize,
    title: &str,
    priority: &str,
    scenarios: &[AcceptanceScenario],
) -> String {
    let mut out = format!(
        "# Generated by solidspec tests\n\
         # Feature: {feature}\n\
         # User Story {idx}: {title} ({priority})\n\n"
    );
    for s in scenarios {
        out.push_str(&format!(
            "TEST \"{escaped_then}\":\n\
             \x20\x20GIVEN: {given}\n\
             \x20\x20WHEN: {when}\n\
             \x20\x20THEN: {then}\n\
             \x20\x20STATUS: NOT IMPLEMENTED\n\n",
            escaped_then = s.then.replace('\"', "\\\""),
            given = s.given,
            when = s.when,
            then = s.then,
        ));
    }
    out
}

/// Generate the test file name for a story.
pub fn test_file_name(story_index: usize, story_title: &str, framework: &TestFramework) -> String {
    let slug = slugify(story_title, &SlugStyle::Underscores);
    let short = if slug.len() > 40 {
        slug[..40].trim_end_matches('_').to_string()
    } else {
        slug
    };
    format!("us{story_index}_{short}{}", framework.file_extension)
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::spec_parser::{ParsedSpec, UserStory};
    use tempfile::TempDir;

    fn sample_spec() -> ParsedSpec {
        ParsedSpec {
            user_stories: vec![
                UserStory {
                    title: "Add a new task".into(),
                    priority: "P1".into(),
                    acceptance_scenarios: vec![
                        "the app is open, **When** user types a task title and clicks \"Add\", **Then** the task appears in the list with status \"pending\"".into(),
                        "user tries to add empty task, **When** clicking \"Add\", **Then** an error message is shown".into(),
                    ],
                },
                UserStory {
                    title: "View all tasks".into(),
                    priority: "P1".into(),
                    acceptance_scenarios: vec![
                        "there are 5 tasks, **When** user opens the app, **Then** all 5 tasks are displayed".into(),
                    ],
                },
            ],
            requirements: vec![],
            clarification_markers: vec![],
            entities: vec![],
            raw: String::new(),
        }
    }

    // ── Scenario Parser Tests ──

    #[test]
    fn extract_scenarios_from_spec() {
        let spec = sample_spec();
        let scenarios = extract_scenarios(&spec);
        assert_eq!(scenarios.len(), 3);
        assert_eq!(scenarios[0].story_index, 1);
        assert_eq!(scenarios[0].story_title, "Add a new task");
        assert_eq!(scenarios[2].story_index, 2);
    }

    #[test]
    fn scenario_splits_given_when_then() {
        let spec = sample_spec();
        let scenarios = extract_scenarios(&spec);
        assert_eq!(scenarios[0].given, "the app is open");
        assert!(scenarios[0].when.contains("user types a task title"));
        assert!(scenarios[0].then.contains("task appears in the list"));
    }

    #[test]
    fn scenario_with_missing_when_skipped() {
        let spec = ParsedSpec {
            user_stories: vec![UserStory {
                title: "Bad".into(),
                priority: "P1".into(),
                acceptance_scenarios: vec!["no markers here at all".into()],
            }],
            requirements: vec![],
            clarification_markers: vec![],
            entities: vec![],
            raw: String::new(),
        };
        let scenarios = extract_scenarios(&spec);
        assert!(scenarios.is_empty());
    }

    #[test]
    fn empty_spec_returns_no_scenarios() {
        let spec = ParsedSpec {
            user_stories: vec![],
            requirements: vec![],
            clarification_markers: vec![],
            entities: vec![],
            raw: String::new(),
        };
        assert!(extract_scenarios(&spec).is_empty());
    }

    // ── Framework Detection Tests ──

    #[test]
    fn detect_jest_from_package_json() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("package.json"),
            r#"{"devDependencies":{"jest":"^29"}}"#,
        )
        .unwrap();
        let fw = detect_framework(dir.path());
        assert_eq!(fw.id, FrameworkId::Jest);
        assert_eq!(fw.file_extension, ".test.js");
    }

    #[test]
    fn detect_typescript_with_tsconfig() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("package.json"),
            r#"{"devDependencies":{"jest":"^29"}}"#,
        )
        .unwrap();
        std::fs::write(dir.path().join("tsconfig.json"), "{}").unwrap();
        let fw = detect_framework(dir.path());
        assert_eq!(fw.id, FrameworkId::Jest);
        assert_eq!(fw.language, "TypeScript");
        assert_eq!(fw.file_extension, ".test.ts");
    }

    #[test]
    fn detect_cargo_test() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
        let fw = detect_framework(dir.path());
        assert_eq!(fw.id, FrameworkId::CargoTest);
    }

    #[test]
    fn detect_pytest() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("pyproject.toml"), "").unwrap();
        let fw = detect_framework(dir.path());
        assert_eq!(fw.id, FrameworkId::Pytest);
    }

    #[test]
    fn detect_go_test() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("go.mod"), "module x").unwrap();
        let fw = detect_framework(dir.path());
        assert_eq!(fw.id, FrameworkId::GoTest);
    }

    #[test]
    fn detect_generic_when_no_files() {
        let dir = TempDir::new().unwrap();
        let fw = detect_framework(dir.path());
        assert_eq!(fw.id, FrameworkId::Generic);
    }

    #[test]
    fn framework_from_name_override() {
        assert_eq!(framework_from_name("jest").unwrap().id, FrameworkId::Jest);
        assert_eq!(
            framework_from_name("pytest").unwrap().id,
            FrameworkId::Pytest
        );
        assert_eq!(
            framework_from_name("cargo").unwrap().id,
            FrameworkId::CargoTest
        );
        assert_eq!(framework_from_name("go").unwrap().id, FrameworkId::GoTest);
        assert!(framework_from_name("unknown").is_none());
    }

    // ── Slugification Tests ──

    #[test]
    fn slugify_underscores() {
        assert_eq!(
            slugify("task appears in list", &SlugStyle::Underscores),
            "task_appears_in_list"
        );
    }

    #[test]
    fn slugify_strips_special_chars() {
        assert_eq!(
            slugify("it's a \"test\"!", &SlugStyle::Underscores),
            "it_s_a_test"
        );
    }

    #[test]
    fn slugify_preserved_keeps_spaces() {
        assert_eq!(
            slugify("task appears in list", &SlugStyle::Preserved),
            "task appears in list"
        );
    }

    #[test]
    fn slugify_max_80_chars() {
        let long = "a ".repeat(50);
        let slug = slugify(&long, &SlugStyle::Underscores);
        assert!(slug.len() <= 80);
    }

    // ── Template Rendering Tests ──

    #[test]
    fn jest_template_valid_syntax() {
        let spec = sample_spec();
        let scenarios = extract_scenarios(&spec);
        let us1: Vec<_> = scenarios.iter().filter(|s| s.story_index == 1).collect();
        let fw = framework_from_name("jest").unwrap();
        let output = render_test_file(
            "001-todo",
            1,
            "Add a new task",
            "P1",
            &us1.iter().map(|s| (*s).clone()).collect::<Vec<_>>(),
            &fw,
        );
        assert!(output.contains("describe('US1: Add a new task'"));
        assert!(output.contains("test('"));
        assert!(output.contains("// Given:"));
        assert!(output.contains("// When:"));
        assert!(output.contains("// Then:"));
        assert!(output.contains("throw new Error('TODO: implement this test')"));
    }

    #[test]
    fn pytest_template_valid_syntax() {
        let scenarios = vec![AcceptanceScenario {
            given: "app is open".into(),
            when: "user clicks add".into(),
            then: "task appears".into(),
            story_index: 1,
            story_title: "Add task".into(),
            story_priority: "P1".into(),
        }];
        let fw = framework_from_name("pytest").unwrap();
        let output = render_test_file("001-todo", 1, "Add task", "P1", &scenarios, &fw);
        assert!(output.contains("class Test"));
        assert!(output.contains("def test_"));
        assert!(output.contains("raise NotImplementedError"));
    }

    #[test]
    fn cargo_template_valid_syntax() {
        let scenarios = vec![AcceptanceScenario {
            given: "app is open".into(),
            when: "user clicks".into(),
            then: "task appears".into(),
            story_index: 1,
            story_title: "Add task".into(),
            story_priority: "P1".into(),
        }];
        let fw = framework_from_name("cargo").unwrap();
        let output = render_test_file("001-todo", 1, "Add task", "P1", &scenarios, &fw);
        assert!(output.contains("#[cfg(test)]"));
        assert!(output.contains("#[test]"));
        assert!(output.contains("fn task_appears"));
        assert!(output.contains("todo!("));
    }

    #[test]
    fn go_template_valid_syntax() {
        let scenarios = vec![AcceptanceScenario {
            given: "app is open".into(),
            when: "user clicks".into(),
            then: "task appears".into(),
            story_index: 1,
            story_title: "Add task".into(),
            story_priority: "P1".into(),
        }];
        let fw = framework_from_name("go").unwrap();
        let output = render_test_file("001-todo", 1, "Add task", "P1", &scenarios, &fw);
        assert!(output.contains("package us1_add_task"));
        assert!(output.contains("import \"testing\""));
        assert!(output.contains("func Test"));
        assert!(output.contains("t.Fatal("));
    }

    #[test]
    fn generic_template_valid() {
        let scenarios = vec![AcceptanceScenario {
            given: "app open".into(),
            when: "click".into(),
            then: "result".into(),
            story_index: 1,
            story_title: "Test".into(),
            story_priority: "P1".into(),
        }];
        let fw = framework_from_name("generic").unwrap();
        let output = render_test_file("001", 1, "Test", "P1", &scenarios, &fw);
        assert!(output.contains("TEST \"result\""));
        assert!(output.contains("GIVEN:"));
        assert!(output.contains("STATUS: NOT IMPLEMENTED"));
    }

    // ── File Name Tests ──

    #[test]
    fn test_file_name_format() {
        let fw = framework_from_name("jest").unwrap();
        let name = test_file_name(1, "Add a new task", &fw);
        assert_eq!(name, "us1_add_a_new_task.test.js");
    }

    #[test]
    fn test_file_name_truncated() {
        let fw = framework_from_name("pytest").unwrap();
        let long_title =
            "a very long story title that exceeds the maximum allowed length for file names";
        let name = test_file_name(1, long_title, &fw);
        assert!(name.len() < 60); // slug 40 + prefix + ext
    }
}
