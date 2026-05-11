use anyhow::{Context, Result};
use clap::Subcommand;

use crate::config;
use crate::presets::manager;

#[derive(Subcommand)]
pub enum PresetCommands {
    /// Install a preset from a local directory
    Add {
        /// Path to preset directory
        path: String,
        /// Priority (lower number = higher precedence)
        #[arg(long, default_value = "10")]
        priority: u32,
    },
    /// Remove an installed preset
    Remove {
        /// Preset ID to remove
        id: String,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },
    /// List installed presets
    List,
    /// Search presets by keyword
    Search {
        /// Search query
        query: String,
    },
    /// Show preset details
    Info {
        /// Preset ID
        id: String,
    },
}

pub fn run(cmd: PresetCommands) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root =
        config::find_project_root(&cwd).context("Not inside a SolidSpec project.")?;

    match cmd {
        PresetCommands::Add { path, priority } => {
            let source = std::path::Path::new(&path);
            if !source.exists() {
                anyhow::bail!("Preset source directory not found: {path}");
            }
            let id = manager::add_preset(&project_root, source, priority)?;
            println!("Preset '{id}' installed with priority {priority}.");
        }
        PresetCommands::Remove { id, force: _ } => {
            manager::remove_preset(&project_root, &id)?;
            println!("Preset '{id}' removed.");
        }
        PresetCommands::List => {
            let presets = manager::list_presets(&project_root)?;
            if presets.is_empty() {
                println!("No presets installed.");
            } else {
                println!(
                    "{:<20} {:<10} {:<10} DESCRIPTION",
                    "ID", "VERSION", "PRIORITY"
                );
                println!("{}", "-".repeat(60));
                for p in &presets {
                    println!(
                        "{:<20} {:<10} {:<10} {}",
                        p.id, p.version, p.priority, p.description
                    );
                }
            }
        }
        PresetCommands::Search { query } => {
            let results = manager::search_presets(&project_root, &query)?;
            if results.is_empty() {
                println!("No presets matching '{query}'.");
            } else {
                for p in &results {
                    println!("  {} (v{}) — {}", p.id, p.version, p.description);
                }
            }
        }
        PresetCommands::Info { id } => match manager::info_preset(&project_root, &id)? {
            Some(p) => {
                println!("Preset: {}", p.id);
                println!("  Name:        {}", p.name);
                println!("  Version:     {}", p.version);
                println!("  Priority:    {}", p.priority);
                println!("  Description: {}", p.description);
                println!("  Installed:   {}", p.installed_at);
            }
            None => println!("Preset '{id}' not found."),
        },
    }
    Ok(())
}
