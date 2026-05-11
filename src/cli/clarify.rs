use anyhow::{Context, Result};

use crate::config;
use crate::core::{feature, spec_parser};

pub fn run(feature_id: Option<&str>) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);
    let spec_path = feature_dir.join("spec.md");

    println!("Clarifying: {feature_dir_name}");

    let spec = spec_parser::parse_spec(&spec_path)?;

    if spec.clarification_markers.is_empty() {
        println!("  No [NEEDS CLARIFICATION] markers found. Spec is clear.");
        return Ok(());
    }

    let markers = &spec.clarification_markers;
    let count = markers.len().min(5); // cap at 5
    println!("  Found {} markers (processing {})", markers.len(), count);

    // Generate structured questions
    let mut questions = Vec::new();
    for marker in markers.iter().take(5) {
        questions.push(format!(
            "Q: {} (line {})\n  | Option | Answer | Implications |\n  |--------|--------|--------------|",
            if marker.text.is_empty() { "Unspecified ambiguity" } else { &marker.text },
            marker.line_number,
        ));
    }

    // Write clarifications.md with session markers
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let clarifications_path = feature_dir.join("clarifications.md");

    let mut content = if clarifications_path.exists() {
        std::fs::read_to_string(&clarifications_path)?
    } else {
        format!("# Clarifications: {feature_dir_name}\n\n## Clarifications\n")
    };

    content.push_str(&format!("\n### Session {date}\n\n"));
    for (i, question) in questions.iter().enumerate() {
        content.push_str(&format!("**Question {}**:\n{question}\n\n", i + 1));
        content.push_str("**Resolution**: [To be resolved]\n\n");
    }

    // Atomic save: write clarifications
    std::fs::write(&clarifications_path, &content)?;
    println!("  Created {}", clarifications_path.display());

    println!(
        "  {} questions generated. Edit clarifications.md and re-run clarify to update spec.",
        count
    );
    Ok(())
}
