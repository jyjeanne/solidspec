use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Subcommand;

use crate::core::okf;

#[derive(Subcommand)]
pub enum OkfCommands {
    /// Generate an Open Knowledge Format (OKF) knowledge-graph bundle
    /// in-process (no external `okf-rs` binary — see src/core/okf.rs)
    Generate {
        /// Project directory to analyze
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Bundle output directory
        #[arg(long, default_value = okf::DEFAULT_BUNDLE_DIR)]
        output: PathBuf,
    },
    /// Validate that a directory is a conformant OKF bundle
    Validate {
        /// Bundle directory to validate
        #[arg(default_value = okf::DEFAULT_BUNDLE_DIR)]
        bundle: PathBuf,

        /// Treat orphaned-concept warnings as failures too (for CI gating)
        #[arg(long)]
        ci: bool,
    },
}

pub fn run(cmd: OkfCommands) -> Result<()> {
    match cmd {
        OkfCommands::Generate { path, output } => generate(&path, &output),
        OkfCommands::Validate { bundle, ci } => validate(&bundle, ci),
    }
}

fn generate(path: &Path, output: &Path) -> Result<()> {
    let report = okf::generate(path, output)?;

    println!(
        "Generated {} concept(s) into {} ({} file(s) parsed, {} reused from cache)",
        report.total_concepts,
        output.display(),
        report.files_parsed,
        report.files_reused_from_cache
    );
    for (kind, count) in &report.by_kind {
        println!("  {kind:?}\t{count}");
    }
    Ok(())
}

fn validate(bundle: &Path, ci: bool) -> Result<()> {
    let report = okf::validate(bundle)?;

    if report.issues.is_empty() {
        println!("{} — no issues found", bundle.display());
    } else {
        for issue in &report.issues {
            let label = match issue.severity {
                okf_validator::Severity::Error => "error",
                okf_validator::Severity::Warning => "warning",
            };
            println!("{label}: {}: {}", issue.file, issue.message);
        }
    }

    if okf::validation_should_fail(&report, ci) {
        anyhow::bail!("bundle validation failed");
    }
    Ok(())
}
