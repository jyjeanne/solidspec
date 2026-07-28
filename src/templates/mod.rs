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
    pub const INTENT_TEMPLATE: &str = include_str!("../../templates/intent-template.md");
    pub const IDSD_SPEC_TEMPLATE: &str = include_str!("../../templates/idsd/spec-template.md");
    pub const IDSD_PLAN_TEMPLATE: &str = include_str!("../../templates/idsd/plan-template.md");

    pub fn all() -> Vec<(&'static str, &'static str)> {
        vec![
            ("spec-template.md", SPEC_TEMPLATE),
            ("plan-template.md", PLAN_TEMPLATE),
            ("tasks-template.md", TASKS_TEMPLATE),
            ("checklist-template.md", CHECKLIST_TEMPLATE),
            ("constitution-template.md", CONSTITUTION_TEMPLATE),
            ("agent-file-template.md", AGENT_FILE_TEMPLATE),
            ("intent-template.md", INTENT_TEMPLATE),
            ("idsd/spec-template.md", IDSD_SPEC_TEMPLATE),
            ("idsd/plan-template.md", IDSD_PLAN_TEMPLATE),
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
    tera.autoescape_on(Vec::<&str>::new()); // Disable HTML auto-escaping — we generate markdown, not HTML
    tera.add_raw_template("template", template_str)
        .map_err(|e| SolidSpecError::Template {
            template: "inline".into(),
            message: format!("Failed to parse template: {e}"),
            fix: "Check template syntax (Tera/Jinja2 format).".into(),
        })?;

    let mut context = Context::new();
    for (key, value) in vars {
        context.insert(key.to_string(), value);
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
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
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

    /// Execute the embedded bash scripts end-to-end (they ship in every project).
    #[cfg(unix)]
    mod bash_execution {
        use super::*;
        use std::process::Command;

        /// Write the embedded bash scripts + a minimal project into a temp dir.
        fn setup_script_project(dir: &Path) -> std::path::PathBuf {
            std::fs::write(dir.join("solidspec.toml"), "[project]\nname = \"t\"\n").unwrap();
            std::fs::create_dir_all(dir.join(".solidspec")).unwrap();
            std::fs::create_dir_all(dir.join("specs/001-auth")).unwrap();
            let scripts = dir.join("scripts");
            std::fs::create_dir_all(&scripts).unwrap();
            for (name, content) in scripts::bash_scripts() {
                std::fs::write(scripts.join(name), content).unwrap();
            }
            scripts
        }

        #[test]
        fn create_new_feature_increments_past_existing_features() {
            let dir = tempfile::TempDir::new().unwrap();
            let scripts = setup_script_project(dir.path());

            // specs/ already has 001-auth — the script must allocate 002, not
            // reallocate 001 (trailing-slash parsing regression).
            let output = Command::new("bash")
                .arg(scripts.join("create-new-feature.sh"))
                .arg("second feature idea")
                .current_dir(dir.path())
                .output()
                .expect("bash must run");
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            assert!(output.status.success(), "script failed: {stderr}");
            assert!(
                stdout.trim().starts_with("002-"),
                "must allocate 002 after 001-auth, got: {stdout} (stderr: {stderr})"
            );
            assert!(
                !stderr.contains("error"),
                "script must not print errors: {stderr}"
            );
            assert!(dir.path().join("specs").join(stdout.trim()).is_dir());
        }

        #[test]
        fn get_current_branch_resolves_env_var_prefix_to_dir_name() {
            let dir = tempfile::TempDir::new().unwrap();
            let scripts = setup_script_project(dir.path());

            // A bare "001" prefix must resolve to the full directory name,
            // matching the Rust CLI's SOLIDSPEC_FEATURE handling.
            let output = Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "source '{}' && get_current_branch",
                    scripts.join("common.sh").display()
                ))
                .env("SOLIDSPEC_FEATURE", "001")
                .current_dir(dir.path())
                .output()
                .expect("bash must run");
            let stdout = String::from_utf8_lossy(&output.stdout);

            assert!(output.status.success());
            assert_eq!(
                stdout.trim(),
                "001-auth",
                "prefix must resolve to full feature dir name"
            );
        }

        #[test]
        fn get_feature_paths_emits_eval_safe_single_line_vars() {
            let dir = tempfile::TempDir::new().unwrap();
            let scripts = setup_script_project(dir.path());

            let output = Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "source '{}' && get_feature_paths",
                    scripts.join("common.sh").display()
                ))
                .current_dir(dir.path())
                .output()
                .expect("bash must run");
            let stdout = String::from_utf8_lossy(&output.stdout);

            assert!(output.status.success());
            let has_git_lines: Vec<&str> = stdout
                .lines()
                .filter(|l| l.starts_with("HAS_GIT="))
                .collect();
            assert_eq!(has_git_lines, vec!["HAS_GIT=false"]);
            // Every line must be a VAR=... assignment (agents eval this output)
            for line in stdout.lines().filter(|l| !l.is_empty()) {
                assert!(
                    line.contains('='),
                    "non-assignment line in get_feature_paths output: {line}"
                );
            }
        }

        #[test]
        fn check_prerequisites_passes_for_complete_project() {
            let dir = tempfile::TempDir::new().unwrap();
            let scripts = setup_script_project(dir.path());
            std::fs::write(
                dir.path().join(".solidspec/constitution.md"),
                "# Constitution\n",
            )
            .unwrap();

            let output = Command::new("bash")
                .arg(scripts.join("check-prerequisites.sh"))
                .current_dir(dir.path())
                .output()
                .expect("bash must run");
            let stdout = String::from_utf8_lossy(&output.stdout);

            assert!(output.status.success(), "stdout: {stdout}");
            assert!(stdout.contains("All checks passed."));
        }

        #[test]
        fn check_prerequisites_fails_when_constitution_missing() {
            let dir = tempfile::TempDir::new().unwrap();
            let scripts = setup_script_project(dir.path());
            // setup_script_project does not write constitution.md

            let output = Command::new("bash")
                .arg(scripts.join("check-prerequisites.sh"))
                .current_dir(dir.path())
                .output()
                .expect("bash must run");
            let stdout = String::from_utf8_lossy(&output.stdout);

            assert!(
                !output.status.success(),
                "script must exit non-zero when constitution.md is missing"
            );
            assert!(stdout.contains("Constitution file missing"));
            assert!(stdout.contains("issue(s) found"));
        }

        #[test]
        fn setup_plan_creates_supporting_files() {
            let dir = tempfile::TempDir::new().unwrap();
            let scripts = setup_script_project(dir.path());

            let output = Command::new("bash")
                .arg(scripts.join("setup-plan.sh"))
                .arg("001-auth")
                .current_dir(dir.path())
                .output()
                .expect("bash must run");
            let stderr = String::from_utf8_lossy(&output.stderr);

            assert!(output.status.success(), "script failed: {stderr}");
            let feature_dir = dir.path().join("specs/001-auth");
            assert!(feature_dir.join("research.md").exists());
            assert!(feature_dir.join("data-model.md").exists());
            assert!(feature_dir.join("quickstart.md").exists());
            assert!(feature_dir.join("contracts/api.md").exists());
        }

        #[test]
        fn setup_plan_does_not_overwrite_existing_files() {
            let dir = tempfile::TempDir::new().unwrap();
            let scripts = setup_script_project(dir.path());
            let research_path = dir.path().join("specs/001-auth/research.md");
            std::fs::write(&research_path, "# Custom research — do not clobber\n").unwrap();

            let output = Command::new("bash")
                .arg(scripts.join("setup-plan.sh"))
                .arg("001-auth")
                .current_dir(dir.path())
                .output()
                .expect("bash must run");

            assert!(output.status.success());
            let content = std::fs::read_to_string(&research_path).unwrap();
            assert!(
                content.contains("do not clobber"),
                "setup-plan.sh must not overwrite an existing research.md"
            );
        }

        #[test]
        fn update_agent_context_lists_feature_status() {
            let dir = tempfile::TempDir::new().unwrap();
            let scripts = setup_script_project(dir.path());
            std::fs::write(
                dir.path().join(".solidspec/constitution.md"),
                "# Constitution\n",
            )
            .unwrap();
            std::fs::write(
                dir.path().join("specs/001-auth/tasks.md"),
                "- [x] T001 Done\n- [ ] T002 Pending\n",
            )
            .unwrap();

            let output = Command::new("bash")
                .arg(scripts.join("update-agent-context.sh"))
                .current_dir(dir.path())
                .output()
                .expect("bash must run");
            let stderr = String::from_utf8_lossy(&output.stderr);

            assert!(output.status.success(), "script failed: {stderr}");
            let agent_md = std::fs::read_to_string(dir.path().join(".solidspec/AGENT.md")).unwrap();
            assert!(agent_md.contains("001-auth"));
            assert!(
                agent_md.contains("in-progress (1/2)"),
                "expected in-progress (1/2) status, got:\n{agent_md}"
            );
        }
    }
}
