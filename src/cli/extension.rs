use anyhow::{Context, Result};
use clap::Subcommand;

use crate::config;
use crate::extensions::manager;

#[derive(Subcommand)]
pub enum ExtensionCommands {
    /// Install an extension from a local directory
    Add {
        /// Path or extension ID
        source: String,
        /// Install from local directory (dev mode)
        #[arg(long)]
        dev: bool,
    },
    /// Remove an installed extension
    Remove {
        /// Extension ID
        id: String,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },
    /// Enable a disabled extension
    Enable {
        /// Extension ID
        id: String,
    },
    /// Disable an extension (keeps files, unregisters commands)
    Disable {
        /// Extension ID
        id: String,
    },
    /// List installed extensions
    List,
    /// Search extensions by keyword
    Search {
        /// Search query
        query: String,
    },
    /// Show extension details
    Info {
        /// Extension ID or name
        name: String,
    },
}

pub fn run(cmd: ExtensionCommands) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root =
        config::find_project_root(&cwd).context("Not inside a SolidSpec project.")?;

    match cmd {
        ExtensionCommands::Add { source, dev } => {
            if dev {
                let path = std::path::Path::new(&source);
                let id = manager::add_extension_dev(&project_root, path)?;
                println!("Extension '{id}' installed (dev mode).");
            } else {
                println!("Catalog-based install not yet implemented. Use --dev for local install.");
            }
        }
        ExtensionCommands::Remove { id, force: _ } => {
            manager::remove_extension(&project_root, &id)?;
            println!("Extension '{id}' removed.");
        }
        ExtensionCommands::Enable { id } => {
            manager::enable_extension(&project_root, &id)?;
            println!("Extension '{id}' enabled.");
        }
        ExtensionCommands::Disable { id } => {
            manager::disable_extension(&project_root, &id)?;
            println!("Extension '{id}' disabled.");
        }
        ExtensionCommands::List => {
            let extensions = manager::list_extensions(&project_root)?;
            if extensions.is_empty() {
                println!("No extensions installed.");
            } else {
                println!("{:<20} {:<10} {:<8} NAME", "ID", "VERSION", "ENABLED");
                println!("{}", "-".repeat(55));
                for e in &extensions {
                    println!("{:<20} {:<10} {:<8} {}", e.id, e.version, e.enabled, e.name);
                }
            }
        }
        ExtensionCommands::Search { query } => {
            let results = manager::search_extensions(&project_root, &query)?;
            if results.is_empty() {
                println!("No extensions matching '{query}'.");
            } else {
                for e in &results {
                    println!("  {} (v{}) — {}", e.id, e.version, e.name);
                }
            }
        }
        ExtensionCommands::Info { name } => match manager::info_extension(&project_root, &name)? {
            Some(e) => {
                println!("Extension: {}", e.id);
                println!("  Name:      {}", e.name);
                println!("  Version:   {}", e.version);
                println!("  Enabled:   {}", e.enabled);
                println!("  Dev:       {}", e.dev);
                println!("  Installed: {}", e.installed_timestamp);
                if !e.hooks.is_empty() {
                    println!("  Hooks:");
                    for h in &e.hooks {
                        println!("    - {} → {}", h.trigger, h.file);
                    }
                }
            }
            None => println!("Extension '{name}' not found."),
        },
    }
    Ok(())
}
