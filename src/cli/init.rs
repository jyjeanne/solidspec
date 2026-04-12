use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::agents::registry;
use crate::config::{InitOptions, ProjectInternalConfig, RootConfig};
use crate::core::git;
use crate::extensions;
use crate::templates;

pub fn run(name: Option<String>, here: bool, no_git: bool, _force: bool, agent: Option<String>) -> Result<()> {
    let project_dir = resolve_project_dir(name.as_deref(), here)?;
    let project_name = project_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "my_project".to_string());

    println!("Initializing RustySpec project: {project_name}");

    // Create directory structure
    create_directory_structure(&project_dir)?;

    // Save root config
    let root_config = RootConfig::new(&project_name);
    root_config.save(&project_dir.join("rustyspec.toml"))?;

    // Save internal config
    let internal_config = ProjectInternalConfig::default();
    internal_config.save(&project_dir.join(".rustyspec/config.toml"))?;

    // Copy embedded templates (preserves existing)
    templates::copy_embedded_templates(&project_dir.join(".rustyspec/templates"))?;

    // Copy embedded scripts (always overwrite)
    templates::copy_embedded_scripts(&project_dir.join(".rustyspec"))?;

    // Generate constitution from template (preserves existing)
    generate_constitution(&project_dir, &project_name)?;

    // Generate AGENT.md
    generate_agent_file(&project_dir, &project_name)?;

    // Detect and register AI agent commands
    let registered = registry::register_all(&project_dir, agent.as_deref())?;
    if registered.is_empty() {
        println!("  No AI agent directories detected (create .claude/, .cursor/, etc. to enable)");
    } else {
        println!(
            "  Registered commands for {} agent(s): {}",
            registered.len(),
            registered.join(", ")
        );
    }

    // Save init options (use first detected agent, detect script type from OS)
    let ai_assistant = registered.first().map(|s| s.as_str()).unwrap_or("claude");
    let script_type = if cfg!(windows) { "ps" } else { "sh" };
    let init_options = InitOptions {
        ai_assistant: ai_assistant.into(),
        script_type: script_type.into(),
        installed_at: chrono::Utc::now().to_rfc3339(),
    };
    init_options.save(&project_dir.join(".rustyspec/init-options.json"))?;

    // Git init
    if !no_git && !git::is_git_repo(&project_dir) {
        println!("  Initializing Git repository...");
        git::init_repo(&project_dir)?;
    } else if no_git {
        println!("  Skipping Git initialization (--no-git)");
    } else {
        println!("  Git repository already exists");
    }

    // Fire after_init hooks
    let ext_registry = extensions::manager::load_registry(&project_dir).unwrap_or_default();
    extensions::hooks::fire_hooks("after_init", &project_dir, &ext_registry);

    println!("  Project initialized at {}", project_dir.display());
    Ok(())
}

fn resolve_project_dir(name: Option<&str>, here: bool) -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;

    match name {
        Some(n) if !here => {
            let dir = cwd.join(n);
            std::fs::create_dir_all(&dir)?;
            Ok(dir)
        }
        _ => Ok(cwd),
    }
}

fn create_directory_structure(project_dir: &Path) -> Result<()> {
    let dirs = [
        ".rustyspec/templates/overrides",
        ".rustyspec/presets",
        ".rustyspec/extensions/.cache/catalogs",
        "specs",
    ];

    for dir in &dirs {
        std::fs::create_dir_all(project_dir.join(dir))?;
    }

    // Create empty registry files
    let registries = [
        ".rustyspec/presets/.registry",
        ".rustyspec/extensions/.registry",
    ];
    for reg in &registries {
        let path = project_dir.join(reg);
        if !path.exists() {
            std::fs::write(&path, "{}")?;
        }
    }

    Ok(())
}

fn generate_constitution(project_dir: &Path, project_name: &str) -> Result<()> {
    let path = project_dir.join(".rustyspec/constitution.md");
    if path.exists() {
        println!("  Constitution already exists, preserving");
        return Ok(());
    }

    let mut vars = HashMap::new();
    vars.insert("project_name".into(), project_name.to_string());
    vars.insert(
        "date".into(),
        chrono::Local::now().format("%Y-%m-%d").to_string(),
    );

    let content = templates::render(templates::embedded::CONSTITUTION_TEMPLATE, &vars)?;
    std::fs::write(&path, content)?;
    println!("  Generated constitution.md");
    Ok(())
}

fn generate_agent_file(project_dir: &Path, project_name: &str) -> Result<()> {
    let path = project_dir.join(".rustyspec/AGENT.md");

    let mut vars = HashMap::new();
    vars.insert("project_name".into(), project_name.to_string());
    vars.insert(
        "date".into(),
        chrono::Local::now().format("%Y-%m-%d").to_string(),
    );

    let content = templates::render(templates::embedded::AGENT_FILE_TEMPLATE, &vars)?;
    std::fs::write(&path, content)?;
    Ok(())
}
