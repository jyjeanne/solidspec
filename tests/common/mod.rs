//! Shared helpers for integration tests. `tests/common/mod.rs` is the
//! standard Cargo convention for code shared between integration test
//! binaries without being compiled as a test binary itself.
#![allow(dead_code)] // not every test file uses every helper

use assert_cmd::Command;
use tempfile::TempDir;

/// Build a `Command` for the compiled `solidspec` binary.
pub fn solidspec() -> Command {
    Command::cargo_bin("solidspec").unwrap()
}

/// Initialise a bare SolidSpec project in a fresh temp dir and return it.
///
/// Pre-creates `.claude/` before `init` so the claude agent is always
/// detected regardless of whether the `claude` CLI binary is present on
/// PATH (required for reliable results on CI).
pub fn init_project() -> TempDir {
    let dir = TempDir::new().unwrap();
    std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();
    dir
}

/// Find the first feature directory under `dir/specs/`.
pub fn first_feature_dir(dir: &std::path::Path) -> std::path::PathBuf {
    let specs = dir.join("specs");
    std::fs::read_dir(&specs)
        .unwrap()
        .flatten()
        .find(|e| e.file_type().unwrap().is_dir())
        .expect("no feature dir found")
        .path()
}
