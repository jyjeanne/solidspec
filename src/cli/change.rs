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

#[cfg(test)]
mod integration_tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::TempDir;

    fn init_project(dir: &std::path::Path) {
        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["init", "--here", "--no-git"])
            .current_dir(dir)
            .assert()
            .success();
        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["specify", "Test feature for changes"])
            .current_dir(dir)
            .assert()
            .success();
    }

    #[test]
    fn change_propose_creates_directory_and_files() {
        let dir = TempDir::new().unwrap();
        init_project(dir.path());

        Command::cargo_bin("solidspec")
            .unwrap()
            .args([
                "change",
                "propose",
                "Add social login",
                "--feature-id",
                "001",
            ])
            .current_dir(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("Created change:"));

        let change_dir = dir
            .path()
            .join("specs/001-test-feature-for-changes/changes/add-social-login");
        assert!(change_dir.exists());
        assert!(change_dir.join("proposal.md").exists());
        assert!(change_dir.join("delta-spec.md").exists());
        assert!(change_dir.join(".change.yaml").exists());

        let prop = std::fs::read_to_string(change_dir.join("proposal.md")).unwrap();
        assert!(prop.contains("Add social login"));
        assert!(prop.contains("## Why"));

        let delta = std::fs::read_to_string(change_dir.join("delta-spec.md")).unwrap();
        assert!(delta.contains("## Added Requirements"));
        assert!(delta.contains("## Modified Requirements"));
        assert!(delta.contains("## Removed Requirements"));
    }

    #[test]
    fn change_list_shows_active_changes() {
        let dir = TempDir::new().unwrap();
        init_project(dir.path());

        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["change", "propose", "First change", "--feature-id", "001"])
            .current_dir(dir.path())
            .assert()
            .success();
        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["change", "propose", "Second change", "--feature-id", "001"])
            .current_dir(dir.path())
            .assert()
            .success();

        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["change", "list", "--feature-id", "001"])
            .current_dir(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("First change"))
            .stdout(predicate::str::contains("Second change"))
            .stdout(predicate::str::contains("proposed"));
    }

    #[test]
    fn change_archive_merges_deltas_and_moves_to_archive() {
        let dir = TempDir::new().unwrap();
        init_project(dir.path());

        // Propose a change
        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["change", "propose", "Delete FR-001", "--feature-id", "001"])
            .current_dir(dir.path())
            .assert()
            .success();

        let feature_dir = dir.path().join("specs/001-test-feature-for-changes");
        let change_dir = feature_dir.join("changes/delete-fr-001");

        // Write a delta spec that removes FR-001
        std::fs::write(
            change_dir.join("delta-spec.md"),
            "## Removed Requirements\n\n- FR-001\n",
        )
        .unwrap();

        // Archive it
        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["change", "archive", "delete-fr-001", "--feature-id", "001"])
            .current_dir(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("Archived change"));

        // Change should be moved to archive
        assert!(!change_dir.exists());
        assert!(feature_dir.join("changes/archive/delete-fr-001").exists());

        // spec.md should no longer contain FR-001
        let updated_spec = std::fs::read_to_string(feature_dir.join("spec.md")).unwrap();
        assert!(!updated_spec.contains("FR-001"));
    }
}
