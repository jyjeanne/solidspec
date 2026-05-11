use anyhow::Result;

use crate::config;
use crate::core::artifact_graph::ArtifactState;
use crate::core::feature;
use crate::core::schema;

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
    let order = graph.topological_order().unwrap_or_else(|_| {
        let ids: Vec<_> = graph.nodes.keys().cloned().collect();
        ids.iter().map(|id| graph.nodes.get(id).unwrap()).collect()
    });

    println!(
        "{:<5} {:<15} {:<15} {:<30}",
        "#", "Artifact", "Status", "Depends On"
    );
    println!("{:-<65}", "");

    for (i, node) in order.iter().enumerate() {
        let state = states
            .get(&node.id)
            .expect("artifact missing from states map");
        let status = match state {
            ArtifactState::Done => "✓ done".to_string(),
            ArtifactState::Ready => "▶ ready".to_string(),
            ArtifactState::Blocked { missing_deps } => {
                format!("⏸ blocked ({})", missing_deps.join(", "))
            }
        };
        let deps = if node.requires.is_empty() {
            "—".to_string()
        } else {
            node.requires.join(", ")
        };

        println!("{:<5} {:<15} {:<15} {:<30}", i + 1, node.id, status, deps,);
    }

    println!();
    println!(
        "Run 'solidspec pipeline {} --from <phase>' to execute a phase.",
        feature_dir_name
    );

    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::TempDir;

    #[test]
    fn status_shows_artifacts_after_pipeline_scaffold() {
        let dir = TempDir::new().unwrap();

        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["init", "--here", "--no-git"])
            .current_dir(dir.path())
            .assert()
            .success();
        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["specify", "Status test feature"])
            .current_dir(dir.path())
            .assert()
            .success();
        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["plan", "001"])
            .current_dir(dir.path())
            .assert()
            .success();

        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["status", "001"])
            .current_dir(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("Schema: spec-driven"))
            .stdout(predicate::str::contains("artifacts"))
            .stdout(predicate::str::contains("done"))
            .stdout(predicate::str::contains("ready"))
            .stdout(predicate::str::contains("spec"))
            .stdout(predicate::str::contains("plan"))
            .stdout(predicate::str::contains("tasks"));
    }

    #[test]
    fn status_with_minimal_schema() {
        let dir = TempDir::new().unwrap();

        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["init", "--here", "--no-git"])
            .current_dir(dir.path())
            .assert()
            .success();
        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["specify", "Minimal test"])
            .current_dir(dir.path())
            .assert()
            .success();

        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["status", "001", "--schema", "minimal"])
            .current_dir(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("Schema: minimal"))
            .stdout(predicate::str::contains("artifacts"));
    }

    #[test]
    fn status_fails_in_non_solidspec_dir() {
        let dir = TempDir::new().unwrap();

        Command::cargo_bin("solidspec")
            .unwrap()
            .args(["status", "001"])
            .current_dir(dir.path())
            .assert()
            .failure()
            .stderr(predicate::str::contains("Not a SolidSpec project"));
    }
}
