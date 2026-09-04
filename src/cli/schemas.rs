use anyhow::Result;

use crate::agents::spcx::schema_short_name;
use crate::config;
use crate::core::schema;

/// `solidspec schemas` — the discovery command for "what workflows exist and
/// when do I use each one" (docs/simplification-study-openspec.md item #5),
/// so that information doesn't have to live in a README table a new user
/// reads before running anything.
pub fn run() -> Result<()> {
    let project_root = config::find_project_root(&std::env::current_dir()?)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    let mut schemas = schema::list_available_schemas(&project_root);
    schemas.sort_by(|a, b| a.name.cmp(&b.name));

    // The current project's own default (set by 'solidspec init --schema',
    // "minimal" otherwise) — not hardcoded, since it's exactly what
    // go/continue/status/tasks/pipeline themselves fall back to when
    // --schema is left unset (see cli::resolved_schema).
    let default_schema = config::project_default_schema(&project_root);

    println!("Available workflow schemas:\n");
    for s in &schemas {
        let default_marker = if s.name == default_schema {
            " (default)"
        } else {
            ""
        };
        println!(
            "{}{}  (v{}, {})",
            s.name, default_marker, s.version, s.source
        );
        println!("  {}", s.description);
        println!("  {} artifact(s)", s.artifact_count);
        if !s.use_case.is_empty() {
            println!("  Use when: {}", s.use_case);
        }
        // The reduced name /spcx:<short>:* slash commands are namespaced
        // under (src/agents/spcx.rs's schema_short_name) — the same string
        // templates/commands/spcx/explore.md points agents at, so that
        // pointer actually resolves to a real command.
        println!(
            "  Slash commands: /spcx:{}:new / :apply / :finalise",
            schema_short_name(&s.name)
        );
        println!();
    }
    println!("Run 'solidspec pipeline --new \"...\" --schema <name>' to use one.");

    Ok(())
}
