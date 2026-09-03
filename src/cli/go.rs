use anyhow::Result;

use super::pipeline;

/// `solidspec go "<description>"` — the flagless front door for starting a
/// new feature (docs/simplification-study-openspec.md item #9): equivalent
/// to `solidspec pipeline --new "<description>" --auto`, on the default
/// `spec-driven` schema. `pipeline` itself keeps its full flag surface for
/// scripted/CI use; this is the common case with nothing to remember.
pub fn run(description: &str, no_agent: bool) -> Result<()> {
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
        "spec-driven",
    )
}
