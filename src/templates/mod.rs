pub mod resolver;

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use tera::{Context, Tera};

use crate::core::errors::SolidSpecError;

/// Embedded default templates
pub mod embedded {
    pub const SPEC_TEMPLATE: &str = include_str!("../../templates/spec-template.md");
    pub const PLAN_TEMPLATE: &str = include_str!("../../templates/plan-template.md");
    pub const TASKS_TEMPLATE: &str = include_str!("../../templates/tasks-template.md");
    pub const CHECKLIST_TEMPLATE: &str = include_str!("../../templates/checklist-template.md");
    pub const CONSTITUTION_TEMPLATE: &str =
        include_str!("../../templates/constitution-template.md");
    pub const AGENT_FILE_TEMPLATE: &str = include_str!("../../templates/agent-file-template.md");

    pub fn all() -> Vec<(&'static str, &'static str)> {
        vec![
            ("spec-template.md", SPEC_TEMPLATE),
            ("plan-template.md", PLAN_TEMPLATE),
            ("tasks-template.md", TASKS_TEMPLATE),
            ("checklist-template.md", CHECKLIST_TEMPLATE),
            ("constitution-template.md", CONSTITUTION_TEMPLATE),
            ("agent-file-template.md", AGENT_FILE_TEMPLATE),
        ]
    }
}

/// Embedded shell scripts
pub mod scripts {
    pub const BASH_COMMON: &str = include_str!("../../scripts/bash/common.sh");
    pub const BASH_CHECK: &str = include_str!("../../scripts/bash/check-prerequisites.sh");
    pub const BASH_NEW_FEATURE: &str = include_str!("../../scripts/bash/create-new-feature.sh");
    pub const BASH_SETUP_PLAN: &str = include_str!("../../scripts/bash/setup-plan.sh");
    pub const BASH_UPDATE_AGENT: &str = include_str!("../../scripts/bash/update-agent-context.sh");

    pub const PS_COMMON: &str = include_str!("../../scripts/powershell/common.ps1");
    pub const PS_CHECK: &str = include_str!("../../scripts/powershell/check-prerequisites.ps1");
    pub const PS_NEW_FEATURE: &str =
        include_str!("../../scripts/powershell/create-new-feature.ps1");
    pub const PS_SETUP_PLAN: &str = include_str!("../../scripts/powershell/setup-plan.ps1");
    pub const PS_UPDATE_AGENT: &str =
        include_str!("../../scripts/powershell/update-agent-context.ps1");

    pub fn bash_scripts() -> Vec<(&'static str, &'static str)> {
        vec![
            ("common.sh", BASH_COMMON),
            ("check-prerequisites.sh", BASH_CHECK),
            ("create-new-feature.sh", BASH_NEW_FEATURE),
            ("setup-plan.sh", BASH_SETUP_PLAN),
            ("update-agent-context.sh", BASH_UPDATE_AGENT),
        ]
    }

    pub fn powershell_scripts() -> Vec<(&'static str, &'static str)> {
        vec![
            ("common.ps1", PS_COMMON),
            ("check-prerequisites.ps1", PS_CHECK),
            ("create-new-feature.ps1", PS_NEW_FEATURE),
            ("setup-plan.ps1", PS_SETUP_PLAN),
            ("update-agent-context.ps1", PS_UPDATE_AGENT),
        ]
    }
}

/// Render a template string with the given variables.
pub fn render(template_str: &str, vars: &HashMap<String, String>) -> Result<String> {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]); // Disable HTML auto-escaping — we generate markdown, not HTML
    tera.add_raw_template("template", template_str)
        .map_err(|e| SolidSpecError::Template {
            template: "inline".into(),
            message: format!("Failed to parse template: {e}"),
            fix: "Check template syntax (Tera/Jinja2 format).".into(),
        })?;

    let mut context = Context::new();
    for (key, value) in vars {
        context.insert(key.as_str(), value);
    }

    tera.render("template", &context)
        .map_err(|e| SolidSpecError::Template {
            template: "inline".into(),
            message: format!("Failed to render template: {e}"),
            fix: "Ensure all required variables are provided.".into(),
        })
        .map_err(Into::into)
}

/// Copy all embedded templates to a target directory.
pub fn copy_embedded_templates(target_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(target_dir)?;
    for (name, content) in embedded::all() {
        let path = target_dir.join(name);
        if !path.exists() {
            std::fs::write(&path, content)?;
        }
    }
    Ok(())
}

/// Copy all embedded scripts to `.solidspec/scripts/`.
/// Always overwrites (scripts are not user-customizable).
pub fn copy_embedded_scripts(solidspec_dir: &Path) -> Result<()> {
    let bash_dir = solidspec_dir.join("scripts/bash");
    std::fs::create_dir_all(&bash_dir)?;
    for (name, content) in scripts::bash_scripts() {
        std::fs::write(bash_dir.join(name), content)?;
    }

    let ps_dir = solidspec_dir.join("scripts/powershell");
    std::fs::create_dir_all(&ps_dir)?;
    for (name, content) in scripts::powershell_scripts() {
        std::fs::write(ps_dir.join(name), content)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_with_all_variables() {
        let template = "# {{ feature_name }}\nID: {{ feature_id }}\nBranch: {{ branch_name }}\nDate: {{ date }}\nProject: {{ project_name }}";
        let mut vars = HashMap::new();
        vars.insert("feature_name".into(), "Auth System".into());
        vars.insert("feature_id".into(), "001".into());
        vars.insert("branch_name".into(), "001-auth-system".into());
        vars.insert("date".into(), "2026-03-14".into());
        vars.insert("project_name".into(), "myapp".into());

        let result = render(template, &vars).unwrap();
        assert!(result.contains("Auth System"));
        assert!(result.contains("001"));
        assert!(result.contains("001-auth-system"));
        assert!(result.contains("2026-03-14"));
        assert!(result.contains("myapp"));
    }

    #[test]
    fn render_missing_variable_returns_error() {
        let template = "Hello {{ name }}";
        let vars = HashMap::new(); // no variables
        assert!(render(template, &vars).is_err());
    }

    #[test]
    fn render_empty_arguments_handled() {
        let template = "Args: {{ arguments }}";
        let mut vars = HashMap::new();
        vars.insert("arguments".into(), "".into());
        let result = render(template, &vars).unwrap();
        assert_eq!(result, "Args: ");
    }

    #[test]
    fn render_preserves_special_characters_in_markdown() {
        let template = "Name: {{ feature_name }}";
        let mut vars = HashMap::new();
        vars.insert("feature_name".into(), "auth & payments <v2>".into());
        let result = render(template, &vars).unwrap();
        // Markdown output must NOT be HTML-escaped
        assert!(result.contains("auth & payments <v2>"), "Got: {result}");
        assert!(
            !result.contains("&amp;"),
            "HTML escaping detected — markdown corrupted: {result}"
        );
    }

    #[test]
    fn all_embedded_templates_are_nonempty() {
        for (name, content) in embedded::all() {
            assert!(!content.is_empty(), "Template {name} is empty");
        }
    }

    #[test]
    fn embedded_templates_contain_expected_markers() {
        assert!(embedded::SPEC_TEMPLATE.contains("Feature Specification"));
        assert!(embedded::PLAN_TEMPLATE.contains("Implementation Plan"));
        assert!(embedded::TASKS_TEMPLATE.contains("Tasks"));
        assert!(embedded::CONSTITUTION_TEMPLATE.contains("Constitution"));
    }

    #[test]
    fn copy_embedded_templates_creates_files() {
        let dir = tempfile::TempDir::new().unwrap();
        let target = dir.path().join("templates");
        copy_embedded_templates(&target).unwrap();

        for (name, _) in embedded::all() {
            assert!(target.join(name).exists(), "Missing template: {name}");
        }
    }

    #[test]
    fn copy_embedded_templates_preserves_existing() {
        let dir = tempfile::TempDir::new().unwrap();
        let target = dir.path().join("templates");
        std::fs::create_dir_all(&target).unwrap();

        // Write a custom spec template
        let custom = "CUSTOM CONTENT";
        std::fs::write(target.join("spec-template.md"), custom).unwrap();

        // Copy should NOT overwrite
        copy_embedded_templates(&target).unwrap();

        let content = std::fs::read_to_string(target.join("spec-template.md")).unwrap();
        assert_eq!(content, custom);
    }

    #[test]
    fn all_bash_scripts_are_nonempty() {
        for (name, content) in scripts::bash_scripts() {
            assert!(!content.is_empty(), "Bash script {name} is empty");
            assert!(
                content.starts_with("#!/"),
                "Bash script {name} missing shebang"
            );
        }
    }

    #[test]
    fn all_powershell_scripts_are_nonempty() {
        for (name, content) in scripts::powershell_scripts() {
            assert!(!content.is_empty(), "PowerShell script {name} is empty");
        }
    }

    #[test]
    fn copy_embedded_scripts_creates_files() {
        let dir = tempfile::TempDir::new().unwrap();
        let solidspec_dir = dir.path().join(".solidspec");
        copy_embedded_scripts(&solidspec_dir).unwrap();

        for (name, _) in scripts::bash_scripts() {
            assert!(
                solidspec_dir.join("scripts/bash").join(name).exists(),
                "Missing bash script: {name}"
            );
        }
        for (name, _) in scripts::powershell_scripts() {
            assert!(
                solidspec_dir.join("scripts/powershell").join(name).exists(),
                "Missing powershell script: {name}"
            );
        }
    }

    #[test]
    fn copy_embedded_scripts_overwrites_existing() {
        let dir = tempfile::TempDir::new().unwrap();
        let solidspec_dir = dir.path().join(".solidspec");
        std::fs::create_dir_all(solidspec_dir.join("scripts/bash")).unwrap();
        std::fs::write(solidspec_dir.join("scripts/bash/common.sh"), "OLD").unwrap();

        copy_embedded_scripts(&solidspec_dir).unwrap();

        let content =
            std::fs::read_to_string(solidspec_dir.join("scripts/bash/common.sh")).unwrap();
        assert_ne!(content, "OLD", "Scripts should be overwritten on copy");
        assert!(content.contains("get_repo_root"));
    }
}
