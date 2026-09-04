use anyhow::Result;

use super::pipeline;
use crate::config;

/// `solidspec go "<description>"` — the flagless front door for starting a
/// new feature (docs/simplification-study-openspec.md item #9): equivalent
/// to `solidspec pipeline --new "<description>" --auto`, on the current
/// project's own default schema (`solidspec.toml`'s `[pipeline].schema`, set
/// by `solidspec init --schema <name>`; "spec-driven" outside any project).
/// `pipeline` itself keeps its full flag surface — including an explicit
/// `--schema` override — for scripted/CI use; this is the common case with
/// nothing to remember.
pub fn run(description: &str, no_agent: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let schema = config::project_default_schema(&cwd);
    pipeline::run(
        None,
        Some(description),
        None,
        None,
        None,
        false,
        false,
        true,
        no_agent,
        &schema,
    )
}
