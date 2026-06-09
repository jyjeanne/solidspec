use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::config;
use crate::core::{constitution, feature, intent_parser, spec_parser};
use crate::presets::manager as preset_manager;
use crate::templates;
use crate::templates::resolver;

/// `schema` is passed explicitly from the pipeline so the correct template is selected
/// regardless of what `solidspec.toml` contains.
pub fn run(feature_id: Option<&str>, schema: Option<&str>) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);
    let spec_path = feature_dir.join("spec.md");

    println!("Generating plan: {feature_dir_name}");

    // Parse spec
    let spec = spec_parser::parse_spec(&spec_path)?;

    // Prerequisite: no unresolved markers
    if !spec.clarification_markers.is_empty() {
        anyhow::bail!(
            "Spec has {} unresolved [NEEDS CLARIFICATION] markers. Run 'solidspec clarify {}' first.",
            spec.clarification_markers.len(),
            feature_dir_name
        );
    }

    // Load constitution and run first compliance check
    let constitution_path = project_root.join(".solidspec/constitution.md");
    let const_doc = constitution::load_constitution(&constitution_path)?;
    let gate_count = const_doc.gates.len();
    println!(
        "  Constitution check #1 (pre-research): {gate_count} gates loaded (will validate post-design)"
    );

    // Load config for template vars
    let root_config = config::RootConfig::load(&project_root.join("solidspec.toml"))?;
    // Prefer explicit schema passed from pipeline CLI; fall back to toml config.
    let effective_schema = schema.unwrap_or(&root_config.pipeline.schema);
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();

    let mut vars = HashMap::new();
    vars.insert("feature_name".into(), feature_dir_name.clone());
    vars.insert(
        "feature_id".into(),
        feature_dir_name
            .split('-')
            .next()
            .unwrap_or("000")
            .to_string(),
    );
    vars.insert("branch_name".into(), feature_dir_name.clone());
    vars.insert("date".into(), date);
    vars.insert("project_name".into(), root_config.project.name.clone());
    vars.insert("arguments".into(), feature_dir_name.clone());

    // IDSD: load intent.md and inject intent vars into template context
    let intent_spec = if effective_schema == "intent-driven" {
        let intent_path = feature_dir.join("intent.md");
        if intent_path.exists() {
            match intent_parser::parse_intent(&intent_path) {
                Ok(i) => Some(i),
                Err(e) => {
                    log::warn!("Failed to parse intent.md: {e}");
                    None
                }
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(ref intent) = intent_spec {
        vars.insert("intent_goal".into(), intent.goal.clone());
        let constraints = if intent.constraints.is_empty() {
            "[No constraints defined]".to_string()
        } else {
            intent
                .constraints
                .iter()
                .map(|c| format!("- {c}"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        let evidence = if intent.evidence.is_empty() {
            "[No evidence criteria defined]".to_string()
        } else {
            intent
                .evidence
                .iter()
                .map(|e| format!("- {e}"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        vars.insert("intent_constraints".into(), constraints);
        vars.insert("intent_evidence".into(), evidence);
    } else {
        // Provide safe defaults so the IDSD template doesn't error
        vars.insert("intent_goal".into(), "[No intent.md found]".to_string());
        vars.insert("intent_constraints".into(), "[None]".to_string());
        vars.insert("intent_evidence".into(), "[None]".to_string());
    }

    // Phase 0: Generate research.md
    let research_content = format!(
        "# Research: {}\n\n**Date**: {}\n\n## Technology Investigation\n\n[Research findings to be filled]\n",
        feature_dir_name, vars["date"]
    );
    std::fs::write(feature_dir.join("research.md"), &research_content)?;
    println!("  Created research.md (Phase 0)");

    // Phase 1: Generate plan.md + supporting docs (resolved through hierarchy)
    let preset_priorities =
        preset_manager::get_preset_priorities(&project_root).unwrap_or_default();
    let (plan_template_name, plan_fallback) = if effective_schema == "intent-driven" {
        (
            "idsd/plan-template.md",
            templates::embedded::IDSD_PLAN_TEMPLATE,
        )
    } else {
        ("plan-template.md", templates::embedded::PLAN_TEMPLATE)
    };
    let (plan_template, _) =
        resolver::load_template(plan_template_name, &project_root, &preset_priorities)
            .unwrap_or_else(|e| {
                log::warn!("Failed to load plan template, using default: {e}");
                (
                    plan_fallback.to_string(),
                    resolver::TemplateSource::EmbeddedDefault,
                )
            });
    let plan_content = templates::render(&plan_template, &vars)?;

    // Constitution check #2 (post-design) — check BEFORE writing to disk
    let gate_results_2 = constitution::check_plan_compliance(&const_doc, &plan_content);
    let mut failed_gates: Vec<_> = gate_results_2.iter().filter(|g| !g.passed).collect();

    // IDSD: also validate intent constraints against plan
    let intent_gate;
    if let Some(ref intent) = intent_spec {
        intent_gate = constitution::check_intent_constraints(intent, &plan_content);
        if !intent_gate.passed {
            failed_gates.push(&intent_gate);
        }
    }

    if failed_gates.is_empty() {
        println!("  Constitution check #2 (post-design): all gates passed");
    } else {
        println!(
            "  Constitution check #2 (post-design): {} violations found",
            failed_gates.len()
        );
        for gate in &failed_gates {
            for v in &gate.violations {
                println!("    [!!] {}: {v}", gate.gate_name);
            }
        }
    }

    std::fs::write(feature_dir.join("plan.md"), &plan_content)?;
    println!("  Created plan.md (Phase 1)");

    // Data model — extract entity descriptions from spec
    let spec_content = std::fs::read_to_string(&spec_path)?;
    let entities_with_desc =
        crate::core::spec_parser::extract_entities_with_descriptions(&spec_content);

    let data_model = format!(
        "# Data Model: {}\n\n## Entities\n\n{}\n",
        feature_dir_name,
        if entities_with_desc.is_empty() {
            "[No entities defined in spec]".to_string()
        } else {
            entities_with_desc
                .iter()
                .map(|(name, desc)| {
                    if desc.is_empty() {
                        format!("### {name}\n\n[Attributes to be defined]\n")
                    } else {
                        format!("### {name}\n\n{desc}\n")
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
    );
    std::fs::write(feature_dir.join("data-model.md"), &data_model)?;
    println!("  Created data-model.md");

    // Quickstart
    let quickstart = format!(
        "# Quickstart: {}\n\n## Key Validation Scenarios\n\n{}\n",
        feature_dir_name,
        spec.user_stories
            .iter()
            .enumerate()
            .map(|(i, s)| format!(
                "{}. **{}** ({}): [validation steps]",
                i + 1,
                s.title,
                s.priority
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
    std::fs::write(feature_dir.join("quickstart.md"), &quickstart)?;
    println!("  Created quickstart.md");

    // Contracts directory
    let contracts_dir = feature_dir.join("contracts");
    std::fs::create_dir_all(&contracts_dir)?;
    std::fs::write(
        contracts_dir.join("api.md"),
        format!(
            "# API Contracts: {}\n\n[To be defined based on plan]\n",
            feature_dir_name
        ),
    )?;
    println!("  Created contracts/");

    // Update AGENT.md
    let agent_vars = HashMap::from([
        ("project_name".to_string(), root_config.project.name),
        (
            "date".to_string(),
            chrono::Local::now().format("%Y-%m-%d").to_string(),
        ),
    ]);
    let agent_content = templates::render(templates::embedded::AGENT_FILE_TEMPLATE, &agent_vars)?;
    std::fs::write(project_root.join(".solidspec/AGENT.md"), &agent_content)?;
    println!("  Updated AGENT.md");

    println!("  Plan complete. Run 'solidspec tasks {feature_dir_name}' next.");
    Ok(())
}
