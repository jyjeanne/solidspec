use anyhow::Result;

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

    println!("Available workflow schemas:\n");
    for s in &schemas {
        let default_marker = if s.name == "spec-driven" {
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
        println!();
    }
    println!("Run 'solidspec pipeline --new \"...\" --schema <name>' to use one.");

    Ok(())
}
