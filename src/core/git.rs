use std::path::Path;

use anyhow::Result;
use git2::{Repository, Signature};

use super::errors::SolidSpecError;

pub fn is_git_repo(path: &Path) -> bool {
    Repository::discover(path).is_ok()
}

pub fn init_repo(path: &Path) -> Result<()> {
    let repo = Repository::init(path).map_err(|e| SolidSpecError::Git {
        message: format!("Failed to init git repo at {}: {e}", path.display()),
        fix: "Check directory permissions or init manually with 'git init'.".into(),
    })?;

    // Stage all files and create initial commit
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;
    let sig = Signature::now("SolidSpec", "noreply@solidspec.dev")?;

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Initial commit from SolidSpec template",
        &tree,
        &[],
    )?;

    Ok(())
}

pub fn create_branch(repo_path: &Path, branch_name: &str) -> Result<()> {
    let repo = Repository::open(repo_path).map_err(|e| SolidSpecError::Git {
        message: format!("Cannot open repo: {e}"),
        fix: "Ensure you're inside a git repository.".into(),
    })?;

    let head = repo.head()?;
    let commit = head.peel_to_commit()?;

    repo.branch(branch_name, &commit, false)
        .map_err(|e| SolidSpecError::Git {
            message: format!("Failed to create branch '{branch_name}': {e}"),
            fix: "Check if a branch with this name already exists.".into(),
        })?;

    // Checkout the new branch
    let refname = format!("refs/heads/{branch_name}");
    let obj = repo.revparse_single(&refname)?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head(&refname)?;

    Ok(())
}

pub fn current_branch(repo_path: &Path) -> Option<String> {
    let repo = Repository::open(repo_path).ok()?;
    let head = repo.head().ok()?;
    head.shorthand().ok().map(String::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn is_git_repo_false_for_plain_dir() {
        let dir = TempDir::new().unwrap();
        assert!(!is_git_repo(dir.path()));
    }

    #[test]
    fn init_repo_creates_git_dir_and_initial_commit() {
        let dir = TempDir::new().unwrap();
        // Create a file so the commit isn't empty
        std::fs::write(dir.path().join("README.md"), "# Test").unwrap();

        init_repo(dir.path()).unwrap();

        assert!(is_git_repo(dir.path()));
        assert!(dir.path().join(".git").exists());

        // Verify initial commit exists
        let repo = Repository::open(dir.path()).unwrap();
        let head = repo.head().unwrap();
        let commit = head.peel_to_commit().unwrap();
        assert!(
            commit
                .message()
                .unwrap()
                .contains("Initial commit from SolidSpec")
        );
    }

    #[test]
    fn create_branch_and_read_current() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("f.txt"), "x").unwrap();
        init_repo(dir.path()).unwrap();

        create_branch(dir.path(), "001-auth-system").unwrap();

        let branch = current_branch(dir.path()).unwrap();
        assert_eq!(branch, "001-auth-system");
    }

    #[test]
    fn current_branch_returns_none_for_non_git() {
        let dir = TempDir::new().unwrap();
        assert!(current_branch(dir.path()).is_none());
    }
}
