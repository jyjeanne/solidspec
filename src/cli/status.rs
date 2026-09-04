use anyhow::Result;

use crate::config;
use crate::core::artifact_graph::ArtifactState;
use crate::core::schema;
use crate::core::{analyzer, feature, pipeline};

pub fn run(feature_id: Option<&str>, schema_name: &str) -> Result<()> {
    let project_root = config::find_project_root(&std::env::current_dir()?)
        .ok_or_else(|| anyhow::anyhow!("Not a SolidSpec project. Run 'solidspec init' first."))?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);

    if !feature_dir.exists() {
        anyhow::bail!(
            "Feature directory not found: {}\nRun 'solidspec specify' first.",
            feature_dir.display()
        );
    }

    let (graph, source) = schema::load_graph(schema_name, &project_root)?;
    let completed = graph.detect_completion(&feature_dir);
    let states = graph.compute_states(&completed);

    // Header
    println!(
        "Feature: {}  |  Schema: {} ({})",
        feature_dir_name,
        schema_name,
        match source {
            schema::SchemaSource::Builtin => "built-in",
            schema::SchemaSource::Default => "default",
            schema::SchemaSource::ProjectLocal(_) => "project-local",
        }
    );
    println!(
        "{} artifacts, {} complete, {} ready",
        graph.nodes.len(),
        completed.len(),
        states
            .values()
            .filter(|s| **s == ArtifactState::Ready)
            .count(),
    );
    println!();

    // Artifact table
    let order: Vec<&crate::core::artifact_graph::ArtifactNode> = match graph.topological_order() {
        Ok(order) => order,
        Err(e) => {
            eprintln!(
                "Warning: schema '{schema_name}' has an invalid dependency graph ({e}); \
                 showing artifacts in unspecified order."
            );
            graph.nodes.values().collect()
        }
    };

    println!(
        "{:<5} {:<15} {:<15} {:<30}",
        "#", "Artifact", "Status", "Depends On"
    );
    println!("{:-<65}", "");

    for (i, node) in order.iter().enumerate() {
        let status = match states.get(&node.id) {
            Some(ArtifactState::Done) => "✓ done".to_string(),
            Some(ArtifactState::Ready) => "▶ ready".to_string(),
            Some(ArtifactState::Blocked { missing_deps }) => {
                format!("⏸ blocked ({})", missing_deps.join(", "))
            }
            None => "? unknown".to_string(),
        };
        let deps = if node.requires.is_empty() {
            "—".to_string()
        } else {
            node.requires.join(", ")
        };

        println!("{:<5} {:<15} {:<15} {:<30}", i + 1, node.id, status, deps,);
    }

    println!();
    println!("{}", pipeline::next_step_hint(graph.first_ready(&states)));

    // IDSD: show intent drift score when the schema is intent-based and intent.md exists
    if schema::is_intent_schema(schema_name)
        && let Some(drift) = analyzer::compute_drift(&feature_dir)
    {
        println!();
        if drift.score > 0.0 {
            println!(
                "Intent Drift: {:.0}%  ({} evidence criteria unsatisfied)",
                drift.score,
                drift.unsatisfied.len()
            );
            for item in &drift.unsatisfied {
                println!("  ✗ {item}");
            }
        } else {
            println!("Intent Drift: 0%  (baseline or all criteria satisfied)");
        }
    }

    Ok(())
}
