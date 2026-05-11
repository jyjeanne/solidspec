use anyhow::Result;

use crate::core::git;

pub fn run() -> Result<()> {
    println!("SolidSpec System Check");
    println!("======================\n");

    // Check Git
    let cwd = std::env::current_dir()?;
    if git::is_git_repo(&cwd) {
        println!("  [OK] Git repository detected");
    } else {
        println!("  [--] Git not available in current directory");
    }

    // Check project structure
    let solidspec_dir = cwd.join(".solidspec");
    if solidspec_dir.exists() {
        println!("  [OK] .solidspec/ directory found");

        let constitution = solidspec_dir.join("constitution.md");
        if constitution.exists() {
            println!("  [OK] Constitution file present");
        } else {
            println!("  [!!] Constitution file missing");
        }
    } else {
        println!("  [--] Not a SolidSpec project (no .solidspec/ directory)");
    }

    let config_path = cwd.join("solidspec.toml");
    if config_path.exists() {
        println!("  [OK] solidspec.toml found");
    } else {
        println!("  [--] solidspec.toml not found");
    }

    println!("\nSolidSpec v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::TempDir;

    #[test]
    fn check_detects_non_project_directory() {
        let dir = TempDir::new().unwrap();

        Command::cargo_bin("solidspec")
            .unwrap()
            .arg("check")
            .current_dir(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("Not a SolidSpec project"));
    }

    #[test]
    fn check_detects_initialized_project() {
        let dir = TempDir::new().unwrap();

        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["init", "--here", "--no-git"])
            .current_dir(dir.path())
            .assert()
            .success();

        Command::cargo_bin("solidspec")
            .unwrap()
            .arg("check")
            .current_dir(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("solidspec.toml found"))
            .stdout(predicate::str::contains(".solidspec/ directory found"))
            .stdout(predicate::str::contains("Constitution file present"));
    }
}
