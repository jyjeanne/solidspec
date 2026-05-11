use std::io::Write;

use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{Shell, generate};

use super::Cli;

pub fn generate_completions(shell: Shell) -> Result<String> {
    let mut cmd = Cli::command();
    let mut buf = Vec::new();
    generate(shell, &mut cmd, "solidspec", &mut buf);
    Ok(String::from_utf8(buf)?)
}

pub fn run(shell: &str) -> Result<()> {
    let shell = match shell.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" | "ps" => Shell::PowerShell,
        _ => anyhow::bail!("Unknown shell '{shell}'. Supported: bash, zsh, fish, powershell"),
    };

    let completions = generate_completions(shell)?;
    std::io::stdout().write_all(completions.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bash_completions_contain_subcommands() {
        let output = generate_completions(Shell::Bash).unwrap();
        assert!(output.contains("init"), "Missing init in bash completions");
        assert!(output.contains("specify"), "Missing specify");
        assert!(output.contains("plan"), "Missing plan");
        assert!(output.contains("preset"), "Missing preset");
        assert!(output.contains("extension"), "Missing extension");
    }

    #[test]
    fn powershell_completions_generated() {
        let output = generate_completions(Shell::PowerShell).unwrap();
        assert!(!output.is_empty());
        // PowerShell completions use Register-ArgumentCompleter
        assert!(output.contains("solidspec"));
    }

    #[test]
    fn fish_completions_generated() {
        let output = generate_completions(Shell::Fish).unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn zsh_completions_generated() {
        let output = generate_completions(Shell::Zsh).unwrap();
        assert!(!output.is_empty());
    }
}

#[cfg(test)]
mod integration_tests {
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
}
