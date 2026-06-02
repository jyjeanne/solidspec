use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn completions_command_produces_output() {
    for shell in &["bash", "zsh", "fish", "powershell"] {
        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["completions", shell])
            .assert()
            .success()
            .stdout(predicate::str::is_empty().not());
    }
}

#[test]
fn completions_unknown_shell_errors() {
    Command::cargo_bin("solidspec")
        .unwrap()
        .args(["completions", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown shell"));
}
