use anyhow::Result;
use clap::Subcommand;

use crate::config;
use crate::core::change;
use crate::core::feature;

#[derive(Subcommand)]
pub enum ChangeCommands {
    /// Propose a new change (delta spec) for a feature
    Propose {
        /// Change title (e.g., "Add social login")
        #[arg(name = "title")]
        title: String,

        /// Feature ID (e.g., 001) — auto-detected if omitted
        #[arg(long)]
        feature_id: Option<String>,
    },
    /// List active changes for a feature
    List {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        #[arg(long)]
        feature_id: Option<String>,
    },
    /// Archive a change (merge deltas into main spec, move to archive/)
    Archive {
        /// Change slug to archive
        change_slug: String,

        /// Feature ID (e.g., 001) — auto-detected if omitted
        #[arg(long)]
        feature_id: Option<String>,
    },
}

pub fn run(command: ChangeCommands) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .ok_or_else(|| anyhow::anyhow!("Not a SolidSpec project. Run 'solidspec init' first."))?;

    match command {
        ChangeCommands::Propose { feature_id, title } => {
            let feature_dir_name = feature::resolve_feature(feature_id.as_deref(), &project_root)?;
            let feature_dir = project_root.join("specs").join(&feature_dir_name);

            if !feature_dir.exists() {
                anyhow::bail!(
                    "Feature directory not found: {}\nRun 'solidspec specify' first.",
                    feature_dir.display()
                );
            }

            let (slug, change_dir) = change::create_change(&feature_dir, &title)?;
            println!("Created change: {slug}");
            println!("  Directory: {}", change_dir.display());
            println!();
            println!("  Next steps:");
            println!("    1. Edit {}/proposal.md to describe the change", slug);
            println!(
                "    2. Edit {}/delta-spec.md with ADDED/MODIFIED/REMOVED requirements",
                slug
            );
            println!(
                "    3. Run 'solidspec change archive {} {}' when done",
                feature_dir_name, slug
            );
        }

        ChangeCommands::List { feature_id } => {
            let feature_dir_name = feature::resolve_feature(feature_id.as_deref(), &project_root)?;
            let feature_dir = project_root.join("specs").join(&feature_dir_name);

            let changes = change::list_changes(&feature_dir)?;
            if changes.is_empty() {
                println!("No active changes for feature '{}'.", feature_dir_name);
                println!(
                    "Run 'solidspec change propose {} \"Title\"' to create one.",
                    feature_dir_name
                );
                return Ok(());
            }

            println!("Changes for feature '{}':\n", feature_dir_name);
            for c in &changes {
                let status = match c.status {
                    change::ChangeStatus::Proposed => "proposed",
                    change::ChangeStatus::InProgress => "in progress",
                    change::ChangeStatus::Archived => "archived",
                };
                println!("  {:6}  {:30}  {}", status, c.title, c.slug);
            }
        }

        ChangeCommands::Archive {
            feature_id,
            change_slug,
        } => {
            let feature_dir_name = feature::resolve_feature(feature_id.as_deref(), &project_root)?;
            let feature_dir = project_root.join("specs").join(&feature_dir_name);

            change::archive_change(&feature_dir, &change_slug)?;
            println!("Archived change '{}'.", change_slug);
            println!("  Deltas merged into {}/spec.md", feature_dir_name);
            println!(
                "  Moved to {}/changes/archive/{}",
                feature_dir_name, change_slug
            );
        }
    }

    Ok(())
}
