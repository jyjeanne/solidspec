use anyhow::Result;

use super::pipeline;

/// `solidspec continue [feature_id]` — resume whatever feature is
/// auto-detected (or given) at whatever phase the artifact graph says is
/// next: `pipeline::should_skip` already skips every completed phase, so
/// running the full phase list with `--auto` and no explicit `--from` *is*
/// "continue" (docs/simplification-study-openspec.md item #9). `pipeline`
/// itself keeps `--from`/`--to`/`--only` for anyone who wants explicit
/// control.
pub fn run(feature_id: Option<&str>, no_agent: bool) -> Result<()> {
    pipeline::run(
        feature_id,
        None,
        None,
        None,
        None,
        false,
        false,
        true,
        no_agent,
        "spec-driven",
    )
}
