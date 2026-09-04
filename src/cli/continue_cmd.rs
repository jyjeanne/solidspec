use anyhow::Result;

use super::pipeline;
use crate::config;

/// `solidspec continue [feature_id]` — resume whatever feature is
/// auto-detected (or given) at whatever phase the artifact graph says is
/// next: `pipeline::should_skip` already skips every completed phase, so
/// running the full phase list with `--auto` and no explicit `--from` *is*
/// "continue" (docs/simplification-study-openspec.md item #9). Runs on the
/// current project's own default schema (see `go::run`'s doc comment).
/// `pipeline` itself keeps `--from`/`--to`/`--only`/`--schema` for anyone
/// who wants explicit control.
pub fn run(feature_id: Option<&str>, no_agent: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let schema = config::project_default_schema(&cwd);
    pipeline::run(
        feature_id, None, None, None, None, false, false, true, no_agent, &schema,
    )
}
