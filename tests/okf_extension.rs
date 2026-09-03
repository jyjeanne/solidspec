//! Integration tests for the bundled `extensions/okf` extension (step 2 of
//! `docs/okf-rs-integration-plan.md`): it must install cleanly as a dev
//! extension and its `after_init` hook must never fail `solidspec init`,
//! whether or not `okf-rs` itself happens to be installed in the
//! environment running the test.

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn okf_extension_source() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("extensions/okf")
}

#[test]
fn okf_extension_installs_and_registers_hook() {
    let dir = TempDir::new().unwrap();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
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
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Extension 'okf' installed"));

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["extension", "info", "okf"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("after_init → hooks/after-init.sh"));
}

#[test]
fn okf_extension_hook_never_fails_init_regardless_of_okf_rs_availability() {
    let dir = TempDir::new().unwrap();

    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
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
        .current_dir(dir.path())
        .assert()
        .success();

    // Re-running init fires the after_init hook (the only way to trigger it
    // on an extension installed after the project already exists — see
    // extensions/okf/README.md). This must succeed whether or not `okf-rs`
    // is on PATH: the hook script probes for it and no-ops otherwise.
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();

    if which::which("okf-rs").is_ok() {
        let okf_toml = dir.path().join("okf.toml");
        assert!(
            okf_toml.exists(),
            "okf-rs is on PATH, so the hook should have written okf.toml"
        );
        let content = std::fs::read_to_string(okf_toml).unwrap();
        assert!(content.contains(".solidspec/knowledge"));
    }
}
