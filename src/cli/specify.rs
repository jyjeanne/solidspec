use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::config;
use crate::core::{feature, git, spec_parser};
use crate::presets::manager as preset_manager;
use crate::templates;
use crate::templates::resolver;

pub fn run(feature_name: &str) -> Result<()> {
    let feature_name = feature_name.trim();
    if feature_name.is_empty() {
        anyhow::bail!("Feature description must not be empty.");
    }

    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let specs_dir = project_root.join("specs");
    let num = feature::next_feature_number(&specs_dir)?;
    let feature_id = feature::format_feature_id(num);
    let short_name = feature::generate_branch_name(feature_name)?;
    let branch_name = format!("{feature_id}-{short_name}");

    println!("Creating feature specification: {branch_name}");

    // Create Git branch FIRST (before writing files) so files land on the right branch
    if git::is_git_repo(&project_root) {
        match git::create_branch(&project_root, &branch_name) {
            Ok(()) => println!("  Created branch: {branch_name}"),
            Err(e) => {
                log::warn!("Could not create branch: {e}");
                println!(
                    "  Warning: Could not create branch ({e}). Files will be on current branch."
                );
            }
        }
    } else {
        println!("  Skipping branch creation (not a git repository)");
    }

    // Create feature directory
    let feature_dir = specs_dir.join(&branch_name);
    std::fs::create_dir_all(&feature_dir)?;

    // Create checklists directory
    let checklists_dir = feature_dir.join("checklists");
    std::fs::create_dir_all(&checklists_dir)?;

    // Render spec from template
    let root_config = config::RootConfig::load(&project_root.join("solidspec.toml"))?;
    let vars = build_template_vars(&root_config, &feature_id, feature_name, &branch_name);

    // Resolve spec template through 4-layer hierarchy
    let preset_priorities =
        preset_manager::get_preset_priorities(&project_root).unwrap_or_default();
    let (spec_template, source) =
        resolver::load_template("spec-template.md", &project_root, &preset_priorities)
            .unwrap_or_else(|e| {
                log::warn!("Failed to load spec template, using default: {e}");
                (
                    templates::embedded::SPEC_TEMPLATE.to_string(),
                    resolver::TemplateSource::EmbeddedDefault,
                )
            });
    log::debug!("Using spec template from {:?}", source);

    let spec_content = templates::render(&spec_template, &vars)?;
    let spec_path = feature_dir.join("spec.md");
    std::fs::write(&spec_path, &spec_content)?;
    println!("  Created {}", spec_path.display());

    // Quality validation (report issues once; auto-fix will be added later)
    let issues = spec_parser::validate_spec_quality(&spec_content);
    if !issues.is_empty() {
        println!("  Quality validation: {} issues found", issues.len());
        for issue in &issues {
            println!("    - {issue}");
        }
    }

    // Generate mandatory quality checklist
    let (checklist_template, _) =
        resolver::load_template("checklist-template.md", &project_root, &preset_priorities)
            .unwrap_or_else(|e| {
                log::warn!("Failed to load checklist template, using default: {e}");
                (
                    templates::embedded::CHECKLIST_TEMPLATE.to_string(),
                    resolver::TemplateSource::EmbeddedDefault,
                )
            });
    let checklist_content = templates::render(&checklist_template, &vars)?;
    let checklist_path = checklists_dir.join("requirements.md");
    std::fs::write(&checklist_path, &checklist_content)?;
    println!("  Created {}", checklist_path.display());

    println!("  Feature {feature_id} ready at specs/{branch_name}/");
    Ok(())
}

fn build_template_vars(
    config: &config::RootConfig,
    feature_id: &str,
    feature_name: &str,
    branch_name: &str,
) -> HashMap<String, String> {
    let mut vars = HashMap::new();
    vars.insert("feature_name".into(), feature_name.to_string());
    vars.insert("feature_id".into(), feature_id.to_string());
    vars.insert("branch_name".into(), branch_name.to_string());
    vars.insert(
        "date".into(),
        chrono::Local::now().format("%Y-%m-%d").to_string(),
    );
    vars.insert("project_name".into(), config.project.name.clone());
    vars.insert("arguments".into(), feature_name.to_string());
    vars
}
