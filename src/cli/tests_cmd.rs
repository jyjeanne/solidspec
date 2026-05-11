use anyhow::{Context, Result};

use crate::config;
use crate::core::{feature, spec_parser, test_generator};

pub fn run(
    feature_id: Option<&str>,
    framework: Option<&str>,
    output: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);

    // Parse spec
    let spec_path = feature_dir.join("spec.md");
    let spec = spec_parser::parse_spec(&spec_path)?;

    // Extract scenarios
    let scenarios = test_generator::extract_scenarios(&spec);
    if scenarios.is_empty() {
        println!("0 acceptance scenarios found in spec.md. No tests generated.");
        return Ok(());
    }

    // Detect or override framework
    let fw = if let Some(name) = framework {
        test_generator::framework_from_name(name)
            .ok_or_else(|| anyhow::anyhow!(
                "Unknown framework '{}'. Supported: jest, vitest, mocha, pytest, cargo, go, generic",
                name
            ))?
    } else {
        test_generator::detect_framework(&project_root)
    };

    println!("Generating tests: {feature_dir_name}");
    println!("  Detected: {} ({})", fw.name, fw.language);
    println!(
        "  Parsed: {} acceptance scenarios from {} user stories",
        scenarios.len(),
        spec.user_stories.len()
    );

    // Determine output directory
    let output_dir = if let Some(dir) = output {
        project_root.join(dir)
    } else {
        feature_dir.join("tests")
    };

    if dry_run {
        println!("  [dry-run] Would generate to: {}", output_dir.display());
        // Group scenarios by story
        let mut story_groups: std::collections::HashMap<
            usize,
            Vec<&test_generator::AcceptanceScenario>,
        > = std::collections::HashMap::new();
        for s in &scenarios {
            story_groups.entry(s.story_index).or_default().push(s);
        }
        let mut sorted: Vec<_> = story_groups.into_iter().collect();
        sorted.sort_by_key(|(idx, _)| *idx);
        for (idx, group) in &sorted {
            let title = &group[0].story_title;
            let file_name = test_generator::test_file_name(*idx, title, &fw);
            println!("  [dry-run] {} ({} tests)", file_name, group.len());
        }
        return Ok(());
    }

    std::fs::create_dir_all(&output_dir)?;

    // Clean up stale test files from previous runs (matching framework extension)
    if let Ok(entries) = std::fs::read_dir(&output_dir) {
        let ext = &fw.file_extension;
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_default();
            // Remove only generated test files (us<N>_ pattern)
            if name.starts_with("us") && name.ends_with(ext.as_str()) && path.is_file() {
                std::fs::remove_file(&path).ok();
            }
        }
    }

    // Group scenarios by story index
    let mut story_groups: std::collections::HashMap<
        usize,
        Vec<test_generator::AcceptanceScenario>,
    > = std::collections::HashMap::new();
    for s in scenarios {
        story_groups.entry(s.story_index).or_default().push(s);
    }

    let mut files_generated = 0;
    let mut scenarios_total = 0;

    let mut sorted_stories: Vec<_> = story_groups.into_iter().collect();
    sorted_stories.sort_by_key(|(idx, _)| *idx);

    for (idx, group) in &sorted_stories {
        let title = &group[0].story_title;
        let priority = &group[0].story_priority;
        let file_name = test_generator::test_file_name(*idx, title, &fw);
        let file_path = output_dir.join(&file_name);

        let content =
            test_generator::render_test_file(&feature_dir_name, *idx, title, priority, group, &fw);

        std::fs::write(&file_path, &content)?;
        println!("  Created {}", file_path.display());
        files_generated += 1;
        scenarios_total += group.len();
    }

    println!(
        "  Generated {} test files ({} scenarios)",
        files_generated, scenarios_total
    );
    Ok(())
}
