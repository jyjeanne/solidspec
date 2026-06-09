use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::config;
use crate::core::{feature, git};
use crate::presets::manager as preset_manager;
use crate::templates;
use crate::templates::resolver;

pub fn run(title: &str, feature_id: Option<&str>) -> Result<()> {
    let title = title.trim();
    if title.is_empty() {
        anyhow::bail!("Intent title must not be empty.");
    }

    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let specs_dir = project_root.join("specs");
    let root_config = config::RootConfig::load(&project_root.join("solidspec.toml"))?;

    let (feature_id_short, branch_name, feature_dir) = if let Some(fid) = feature_id {
        // Add intent to an existing feature directory
        let dir_name = feature::find_feature_dir_by_prefix(&specs_dir, fid)?;

        // Guard against path traversal: dir_name must be a plain directory component.
        // It must not contain path separators or dot-dot sequences.
        if dir_name.contains('/') || dir_name.contains('\\') || dir_name.contains("..") {
            anyhow::bail!(
                "Invalid feature identifier '{}': must not contain path separators or '..'.",
                dir_name
            );
        }
        let feature_dir = specs_dir.join(&dir_name);
        // Canonicalize both paths and assert containment as a second line of defence.
        // (canonicalize requires the path to exist, which it does — find_feature_dir_by_prefix
        //  already confirmed the directory is present on disk.)
        let canonical_specs = specs_dir.canonicalize()?;
        let canonical_feature = feature_dir.canonicalize()?;
        if !canonical_feature.starts_with(&canonical_specs) {
            anyhow::bail!(
                "Feature directory '{}' is outside the specs/ tree — refusing to write.",
                canonical_feature.display()
            );
        }

        // Safe to take the 3-char numeric prefix now; use chars() to avoid UTF-8 byte panics.
        let short: String = dir_name.chars().take(3).collect();
        (short, dir_name, feature_dir)
    } else {
        // Create a new feature directory (same flow as specify)
        let num = feature::next_feature_number(&specs_dir)?;
        let fid = feature::format_feature_id(num);
        let short_name = feature::generate_branch_name(title)?;
        let bname = format!("{fid}-{short_name}");

        println!("Creating intent: {bname}");

        if git::is_git_repo(&project_root) {
            match git::create_branch(&project_root, &bname) {
                Ok(()) => println!("  Created branch: {bname}"),
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

        let feature_dir = specs_dir.join(&bname);
        std::fs::create_dir_all(&feature_dir)?;
        std::fs::create_dir_all(feature_dir.join("checklists"))?;
        (fid, bname, feature_dir)
    };

    // Bail out cleanly if intent.md already exists
    let intent_path = feature_dir.join("intent.md");
    if intent_path.exists() {
        println!("  intent.md already exists at {}", intent_path.display());
        println!(
            "  Edit it directly or re-run with '--feature {}' to replace.",
            feature_id_short
        );
        return Ok(());
    }

    // Render intent template through the 4-layer resolution hierarchy
    let preset_priorities =
        preset_manager::get_preset_priorities(&project_root).unwrap_or_default();
    let (intent_template, source) =
        resolver::load_template("intent-template.md", &project_root, &preset_priorities)
            .unwrap_or_else(|e| {
                log::warn!("Failed to load intent template, using default: {e}");
                (
                    templates::embedded::INTENT_TEMPLATE.to_string(),
                    resolver::TemplateSource::EmbeddedDefault,
                )
            });
    log::debug!("Using intent template from {:?}", source);

    let vars = build_template_vars(&root_config, &feature_id_short, title, &branch_name);
    let intent_content = templates::render(&intent_template, &vars)?;
    std::fs::write(&intent_path, &intent_content)?;
    println!("  Created {}", intent_path.display());

    println!();
    println!("  Feature {} intent ready.", feature_id_short);
    println!("  Next steps:");
    println!("    1. Edit intent.md — refine the Goal, Constraints, and Evidence.");
    println!("    2. Set Status to 'active' when ready.");
    println!("    3. Run 'solidspec specify' to generate spec.md from the intent.");

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
