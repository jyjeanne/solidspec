//! Integration tests for the bundled `extensions/okf` extension (step 2 of
//! `docs/okf-rs-integration-plan.md`): it must install cleanly as a dev
//! extension and its `after_init` hook must never fail `solidspec init`,
//! whether or not the `solidspec` binary itself is resolvable on `PATH`
//! from the hook's shell — this extension no longer depends on any
//! external `okf-rs` binary (see src/core/okf.rs).

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn okf_extension_source() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("extensions/okf")
}

/// Directory containing the just-built `solidspec` binary under test —
/// prepending it to `PATH` lets the hook's `command -v solidspec` (and the
/// call itself) succeed even though this binary was never `cargo install`ed.
fn solidspec_bin_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_BIN_EXE_solidspec"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn path_with_solidspec_on_it() -> std::ffi::OsString {
    let existing = std::env::var_os("PATH").unwrap_or_default();
    let mut paths = vec![solidspec_bin_dir()];
    paths.extend(std::env::split_paths(&existing));
    std::env::join_paths(paths).unwrap()
}

fn install_okf_extension(dir: &std::path::Path) {
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir)
        .assert()
        .success();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args([
            "extension",
            "add",
            okf_extension_source().to_str().unwrap(),
            "--dev",
        ])
        .current_dir(dir)
        .assert()
        .success();
}

#[test]
fn okf_extension_installs_and_registers_hook() {
    let dir = TempDir::new().unwrap();
    install_okf_extension(dir.path());

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["extension", "info", "okf"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("after_init → hooks/after-init.sh"));
}

#[test]
fn okf_extension_hook_generates_a_real_bundle_when_solidspec_is_on_path() {
    let dir = TempDir::new().unwrap();
    install_okf_extension(dir.path());

    // Re-running init fires the after_init hook (the only way to trigger it
    // on an extension installed after the project already exists — see
    // extensions/okf/README.md).
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .env("PATH", path_with_solidspec_on_it())
        .assert()
        .success();

    assert!(
        dir.path().join(".solidspec/knowledge/index.md").exists(),
        "the hook should have generated a real OKF bundle now that `solidspec` is on PATH"
    );
}

#[test]
fn okf_extension_hook_never_fails_init_when_solidspec_is_not_on_path() {
    let dir = TempDir::new().unwrap();
    install_okf_extension(dir.path());

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        // A PATH with no `solidspec` on it at all — the hook's `command -v`
        // check must no-op cleanly rather than failing `init`.
        .env("PATH", "/usr/bin:/bin")
        .assert()
        .success();

    assert!(!dir.path().join(".solidspec/knowledge/index.md").exists());
}
