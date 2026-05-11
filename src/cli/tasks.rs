use anyhow::{Context, Result};

use crate::config;
use crate::core::{feature, spec_parser, task_generator};
use crate::extensions;

pub fn run(feature_id: Option<&str>) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);

    println!("Generating tasks: {feature_dir_name}");

    // Read spec (required)
    let spec_path = feature_dir.join("spec.md");
    let spec = spec_parser::parse_spec(&spec_path)?;

    // Read plan (required)
    let plan_path = feature_dir.join("plan.md");
    if !plan_path.exists() {
        anyhow::bail!(
            "plan.md not found. Run 'solidspec plan {}' first.",
            feature_dir_name
        );
    }
    let plan_content = std::fs::read_to_string(&plan_path)?;

    // Check optional supporting docs
    let has_data_model = feature_dir.join("data-model.md").exists();

    // Generate tasks
    let task_list = task_generator::generate_tasks(&spec, &plan_content, has_data_model);
    let output = task_generator::format_task_list(&task_list, &feature_dir_name, &feature_dir_name);

    // Write tasks.md
    let tasks_path = feature_dir.join("tasks.md");
    std::fs::write(&tasks_path, &output)?;

    let total_tasks: usize = task_list.phases.iter().map(|p| p.tasks.len()).sum();
    println!(
        "  Created tasks.md ({} tasks across {} phases)",
        total_tasks,
        task_list.phases.len()
    );

    // Fire after_tasks hooks
    let ext_registry = extensions::manager::load_registry(&project_root).unwrap_or_default();
    extensions::hooks::fire_hooks("after_tasks", &project_root, &ext_registry);

    println!(
        "  Run 'solidspec analyze {feature_dir_name}' to validate, then begin implementation."
    );
    Ok(())
}
