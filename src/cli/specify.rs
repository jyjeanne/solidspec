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

    // IDSD soft reminder: warn if intent.md is absent when using intent-driven schema
    if root_config.pipeline.schema == "intent-driven" && !feature_dir.join("intent.md").exists() {
        println!("  Hint: Schema is 'intent-driven' but intent.md is missing for {feature_id}.");
        println!(
            "  Run 'solidspec intent \"{feature_name}\" --feature {feature_id}' to capture intent first."
        );
    }

    let preset_priorities =
        preset_manager::get_preset_priorities(&project_root).unwrap_or_default();
    write_spec(
        &feature_dir,
        &checklists_dir,
        &vars,
        &root_config.pipeline.schema,
        &project_root,
        &preset_priorities,
    )?;

    println!("  Feature {feature_id} ready at specs/{branch_name}/");
    Ok(())
}

/// Generate `spec.md` into an already-existing feature directory.
/// Used by the IDSD pipeline when the `intent` phase already created the feature dir.
/// `schema` is passed explicitly from the pipeline CLI so the correct template is used
/// even when `solidspec.toml` still has the default `spec-driven` schema.
pub fn run_for_existing(feature_dir_name: &str, feature_title: &str, schema: &str) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    // Guard against path traversal: reject separators and parent-dir components.
    if feature_dir_name.contains('/')
        || feature_dir_name.contains('\\')
        || feature_dir_name.contains("..")
    {
        anyhow::bail!(
            "Invalid feature directory name '{}': must not contain path separators or '..'.",
            feature_dir_name
        );
    }

    let specs_dir = project_root.join("specs");
    let feature_dir = specs_dir.join(feature_dir_name);

    // Canonicalize containment check (defence-in-depth).
    let canonical_specs = specs_dir
        .canonicalize()
        .unwrap_or_else(|_| specs_dir.clone());
    let canonical_feature = feature_dir
        .canonicalize()
        .unwrap_or_else(|_| feature_dir.clone());
    if !canonical_feature.starts_with(&canonical_specs) {
        anyhow::bail!(
            "Feature directory '{}' resolves outside the specs/ directory.",
            feature_dir_name
        );
    }

    if !feature_dir.exists() {
        anyhow::bail!(
            "Feature directory '{}' does not exist. Run 'solidspec intent' first.",
            feature_dir.display()
        );
    }

    let feature_id: String = feature_dir_name.chars().take(3).collect();
    let root_config = config::RootConfig::load(&project_root.join("solidspec.toml"))?;
    let vars = build_template_vars(&root_config, &feature_id, feature_title, feature_dir_name);

    let checklists_dir = feature_dir.join("checklists");
    std::fs::create_dir_all(&checklists_dir)?;

    let preset_priorities =
        preset_manager::get_preset_priorities(&project_root).unwrap_or_default();
    write_spec(
        &feature_dir,
        &checklists_dir,
        &vars,
        schema,
        &project_root,
        &preset_priorities,
    )?;

    println!("  Feature {feature_id} spec added to {feature_dir_name}/");
    Ok(())
}

/// Render and write `spec.md` + checklist. Shared by `run` and `run_for_existing`.
fn write_spec(
    feature_dir: &std::path::Path,
    checklists_dir: &std::path::Path,
    vars: &HashMap<String, String>,
    schema: &str,
    project_root: &std::path::Path,
    preset_priorities: &[(String, u32)],
) -> Result<()> {
    let (template_name, fallback) = if schema == "intent-driven" {
        (
            "idsd/spec-template.md",
            templates::embedded::IDSD_SPEC_TEMPLATE,
        )
    } else {
        ("spec-template.md", templates::embedded::SPEC_TEMPLATE)
    };

    let (spec_template, source) =
        resolver::load_template(template_name, project_root, preset_priorities).unwrap_or_else(
            |e| {
                log::warn!("Failed to load spec template, using default: {e}");
                (
                    fallback.to_string(),
                    resolver::TemplateSource::EmbeddedDefault,
                )
            },
        );
    log::debug!("Using spec template from {:?}", source);

    let spec_content = templates::render(&spec_template, vars)?;
    let spec_path = feature_dir.join("spec.md");
    std::fs::write(&spec_path, &spec_content)?;
    println!("  Created {}", spec_path.display());

    // Quality validation
    let issues = spec_parser::validate_spec_quality(&spec_content);
    if !issues.is_empty() {
        println!("  Quality validation: {} issues found", issues.len());
        for issue in &issues {
            println!("    - {issue}");
        }
    }

    // Generate mandatory quality checklist
    let (checklist_template, _) =
        resolver::load_template("checklist-template.md", project_root, preset_priorities)
            .unwrap_or_else(|e| {
                log::warn!("Failed to load checklist template, using default: {e}");
                (
                    templates::embedded::CHECKLIST_TEMPLATE.to_string(),
                    resolver::TemplateSource::EmbeddedDefault,
                )
            });
    let checklist_content = templates::render(&checklist_template, vars)?;
    let checklist_path = checklists_dir.join("requirements.md");
    std::fs::write(&checklist_path, &checklist_content)?;
    println!("  Created {}", checklist_path.display());

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
