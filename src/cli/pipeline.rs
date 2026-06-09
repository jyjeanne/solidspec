use std::io::Write;
use std::time::Instant;

use anyhow::{Context, Result};

use crate::agents::invoker::{self, InvokeResult};
use crate::config;
use crate::core::{feature, pipeline};

#[allow(clippy::too_many_arguments)]
pub fn run(
    feature_id: Option<&str>,
    new_desc: Option<&str>,
    from: Option<&str>,
    to: Option<&str>,
    only: Option<&str>,
    force: bool,
    dry_run: bool,
    auto: bool,
    no_agent: bool,
    schema: &str,
) -> Result<()> {
    // Validate mutual exclusivity
    if new_desc.is_some() && feature_id.is_some() {
        anyhow::bail!("Cannot use --new with a feature ID. Use one or the other.");
    }

    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let root_config = config::RootConfig::load(&project_root.join("solidspec.toml"))?;
    let default_agent = &root_config.ai.default_agent;

    // Validate pipeline config agent IDs
    let valid_agents: Vec<&str> = crate::agents::config::AGENTS.iter().map(|a| a.id).collect();
    let mut all_valid: Vec<&str> = valid_agents.clone();
    for a in crate::agents::config::AGENTS {
        for alias in a.aliases {
            all_valid.push(alias);
        }
    }
    root_config.pipeline.validate(&all_valid)?;

    // Resolve --only to --from + --to
    let (from, to) = if let Some(phase) = only {
        (Some(phase), Some(phase))
    } else {
        (from, to)
    };

    // Filter phases (schema selects SDD vs IDSD phase set)
    let phases = pipeline::filter_phases(schema, from, to)?;

    // Resolve or create feature
    let is_new = new_desc.is_some();
    let mut feature_dir_name = if is_new {
        let desc = new_desc.unwrap();
        if desc.trim().is_empty() {
            anyhow::bail!("Feature description must not be empty.");
        }
        // Pre-compute name for display; specify will create the actual dir
        let specs_dir = project_root.join("specs");
        let num = feature::next_feature_number(&specs_dir)?;
        let fid = feature::format_feature_id(num);
        let short = feature::generate_branch_name(desc)?;
        format!("{fid}-{short}")
    } else {
        feature::resolve_feature(feature_id, &project_root)?
    };

    let mut feature_dir = project_root.join("specs").join(&feature_dir_name);

    // Check agent CLI availability upfront
    let agent_mode = if no_agent {
        AgentMode::Disabled
    } else {
        check_agent_availability(&phases, &root_config.pipeline, default_agent)
    };

    match &agent_mode {
        AgentMode::Disabled => {
            println!("Pipeline: {feature_dir_name} [scaffold-only]\n");
        }
        AgentMode::AllCli => {
            println!("Pipeline: {feature_dir_name} [fully automated]\n");
        }
        AgentMode::Mixed { handoff_phases } => {
            println!("Pipeline: {feature_dir_name} [mixed mode]");
            println!("  Handoff required for: {}\n", handoff_phases.join(", "));
        }
    }

    if dry_run {
        for (i, phase) in phases.iter().enumerate() {
            let agent = root_config.pipeline.agent_for_phase(phase, default_agent);
            let skip = pipeline::should_skip(phase, &feature_dir, force);
            let ptype = pipeline::phase_type(phase);
            let type_label = if ptype == pipeline::PhaseType::Handoff {
                " [HANDOFF]"
            } else if agent_mode == AgentMode::Disabled {
                " [scaffold]"
            } else if invoker::supports_cli(&agent) {
                " [auto+agent]"
            } else {
                " [HANDOFF]"
            };
            let status = if skip { "○ skip" } else { "● run" };
            println!(
                "  Phase {}/{}: {} ({}){} — {}",
                i + 1,
                phases.len(),
                phase,
                agent,
                type_label,
                status
            );
        }
        println!("\n[dry-run] No files created or modified.");
        return Ok(());
    }

    // For --new, do NOT pre-create the feature dir.
    // The specify phase will create it with the correct numbering.

    let mut results: Vec<pipeline::PhaseResult> = Vec::new();

    for (i, phase) in phases.iter().enumerate() {
        let agent = root_config.pipeline.agent_for_phase(phase, default_agent);
        let ptype = pipeline::phase_type(phase);

        // Check skip
        if pipeline::should_skip(phase, &feature_dir, force) {
            let reason = skip_reason(phase, &feature_dir);
            println!(
                "  Phase {}/{}: {} ({})\n    ○ skipped — {reason}",
                i + 1,
                phases.len(),
                phase,
                agent
            );
            results.push(pipeline::PhaseResult {
                name: phase.to_string(),
                agent: agent.clone(),
                status: pipeline::PhaseStatus::Skipped,
                duration_ms: 0,
                output: reason,
            });
            continue;
        }

        // Print phase header
        let type_label = if ptype == pipeline::PhaseType::Handoff {
            " [HANDOFF]"
        } else {
            ""
        };
        println!(
            "  Phase {}/{}: {} ({}){type_label}",
            i + 1,
            phases.len(),
            phase,
            agent
        );

        // Execute phase
        let start = Instant::now();
        let result = execute_phase(
            phase,
            &feature_dir_name,
            &feature_dir,
            &project_root,
            &agent,
            new_desc,
            auto,
            &agent_mode,
            schema,
        );
        let elapsed_ms = start.elapsed().as_millis() as u64;

        match result {
            Ok(output) => {
                let status = if ptype == pipeline::PhaseType::Handoff {
                    pipeline::PhaseStatus::Handoff
                } else {
                    pipeline::PhaseStatus::Done
                };
                let duration_str = if status == pipeline::PhaseStatus::Handoff {
                    "user-confirmed".to_string()
                } else {
                    format!("{:.1}s", elapsed_ms as f64 / 1000.0)
                };
                println!("    ✓ {} ({duration_str})", output);
                results.push(pipeline::PhaseResult {
                    name: phase.to_string(),
                    agent: agent.clone(),
                    status,
                    duration_ms: elapsed_ms,
                    output,
                });

                // After intent or specify with --new, re-detect the actual feature dir
                // (both commands create the dir with their own numbering)
                if (*phase == "intent" || *phase == "specify") && is_new {
                    feature_dir_name = feature::resolve_feature(None, &project_root)?;
                    feature_dir = project_root.join("specs").join(&feature_dir_name);
                }
            }
            Err(e) => {
                println!("    ✗ FAILED: {e}");
                results.push(pipeline::PhaseResult {
                    name: phase.to_string(),
                    agent,
                    status: pipeline::PhaseStatus::Failed,
                    duration_ms: elapsed_ms,
                    output: format!("error: {e}"),
                });
                // Write partial log before stopping
                if feature_dir.exists() {
                    pipeline::write_log(&feature_dir, &feature_dir_name, &results).ok();
                }
                anyhow::bail!("Pipeline stopped at phase '{}': {e}", phase);
            }
        }
    }

    // Write pipeline log
    if feature_dir.exists() {
        pipeline::write_log(&feature_dir, &feature_dir_name, &results)?;
    }

    let agent_list: Vec<String> = {
        let mut seen = Vec::new();
        for r in &results {
            if !seen.contains(&r.agent) {
                seen.push(r.agent.clone());
            }
        }
        seen
    };

    println!(
        "\nPipeline complete: {} phases, {} agents ({})",
        results.len(),
        agent_list.len(),
        agent_list.join(", ")
    );
    println!("Log: specs/{feature_dir_name}/pipeline-log.md");
    Ok(())
}

/// Agent execution mode for the pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
enum AgentMode {
    /// No agent invocation — scaffold only (--no-agent)
    Disabled,
    /// All phases have CLI-capable agents
    AllCli,
    /// Some phases need handoff (agent CLI not available)
    Mixed { handoff_phases: Vec<String> },
}

/// Check which agents are available for CLI invocation.
fn check_agent_availability(
    phases: &[&str],
    pipeline_config: &config::PipelineConfig,
    default_agent: &str,
) -> AgentMode {
    let mut handoff = Vec::new();

    for phase in phases {
        let agent_id = pipeline_config.agent_for_phase(phase, default_agent);
        // implement and apex are always handoffs — AI agent does the actual coding
        if *phase == "implement" || *phase == "apex" {
            handoff.push(format!("{phase} ({agent_id})"));
            continue;
        }
        if !invoker::supports_cli(&agent_id) {
            handoff.push(format!("{phase} ({agent_id})"));
        }
    }

    if handoff.is_empty() {
        AgentMode::AllCli
    } else {
        AgentMode::Mixed {
            handoff_phases: handoff,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn execute_phase(
    phase: &str,
    feature_dir_name: &str,
    feature_dir: &std::path::Path,
    project_root: &std::path::Path,
    agent: &str,
    new_desc: Option<&str>,
    auto: bool,
    agent_mode: &AgentMode,
    schema: &str,
) -> Result<String> {
    match phase {
        "intent" => {
            let desc = new_desc.unwrap_or(feature_dir_name);
            crate::cli::intent::run(desc, None)?;
            Ok("intent.md created".into())
        }
        "specify" => {
            let desc = new_desc.unwrap_or(feature_dir_name);
            // In IDSD mode the intent phase already created the feature dir.
            // Use run_for_existing so specify doesn't allocate a new feature number.
            if schema == "intent-driven" && feature_dir.exists() {
                crate::cli::specify::run_for_existing(feature_dir_name, desc, schema)?;
            } else {
                crate::cli::specify::run(desc)?;
            }

            if *agent_mode != AgentMode::Disabled {
                invoke_or_handoff(
                    agent,
                    phase,
                    feature_dir_name,
                    project_root,
                    Some(desc),
                    auto,
                )?;
            }
            Ok("spec.md created".into())
        }
        "clarify" => {
            crate::cli::clarify::run(Some(feature_dir_name))?;

            if *agent_mode != AgentMode::Disabled {
                invoke_or_handoff(agent, phase, feature_dir_name, project_root, None, auto)?;
            }
            Ok("clarification complete".into())
        }
        "plan" => {
            crate::cli::plan::run(Some(feature_dir_name), Some(schema))?;

            if *agent_mode != AgentMode::Disabled {
                invoke_or_handoff(agent, phase, feature_dir_name, project_root, None, auto)?;
            }
            Ok("plan.md + supporting docs".into())
        }
        "tasks" => {
            crate::cli::tasks::run(Some(feature_dir_name))?;

            if *agent_mode != AgentMode::Disabled {
                invoke_or_handoff(agent, phase, feature_dir_name, project_root, None, auto)?;
            }
            Ok("tasks.md generated".into())
        }
        "tests" => {
            crate::cli::tests_cmd::run(Some(feature_dir_name), None, None, false)?;

            if *agent_mode != AgentMode::Disabled {
                invoke_or_handoff(agent, phase, feature_dir_name, project_root, None, auto)?;
            }
            Ok("test scaffolds generated".into())
        }
        "implement" => {
            // Implement is always a handoff — AI agent does the actual coding
            println!(
                "    → Open {} and run: /solidspec-implement {feature_dir_name}",
                agent
            );
            if auto {
                println!("    [auto] Skipping confirmation");
            } else {
                print!("    ⏳ Press Enter when done (or Ctrl+C to abort)... ");
                std::io::stdout().flush()?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
            }
            Ok("user-confirmed".into())
        }
        "apex" => {
            // APEX is always a handoff — write context then hand off to AI agent
            crate::cli::apex::run(Some(feature_dir_name), false, true, false)?;
            println!(
                "    → Open {} and run: /solidspec-apex {feature_dir_name}",
                agent
            );
            if auto {
                println!("    [auto] Skipping confirmation");
            } else {
                print!("    ⏳ Press Enter when done (or Ctrl+C to abort)... ");
                std::io::stdout().flush()?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
            }
            Ok("user-confirmed".into())
        }
        "evidence" => {
            crate::cli::evidence::run(Some(feature_dir_name), false)?;
            Ok("evidence-report.md written".into())
        }
        "analyze" => {
            crate::cli::analyze::run(Some(feature_dir_name))?;
            Ok("analysis complete".into())
        }
        "review" => {
            crate::cli::review::run(Some(feature_dir_name))?;
            Ok("review complete".into())
        }
        _ => anyhow::bail!("Unknown phase: {phase}"),
    }
}

/// Invoke the AI agent CLI, or fall back to handoff if CLI is not available.
fn invoke_or_handoff(
    agent_id: &str,
    phase: &str,
    feature_dir_name: &str,
    project_root: &std::path::Path,
    description: Option<&str>,
    auto: bool,
) -> Result<()> {
    println!("    → Invoking {} for '{}'...", agent_id, phase);

    match invoker::invoke_agent(
        agent_id,
        phase,
        feature_dir_name,
        project_root,
        description,
        None,
    ) {
        InvokeResult::Success { output } => {
            let preview = output.lines().take(3).collect::<Vec<_>>().join(" ");
            let preview = if preview.len() > 100 {
                format!("{}...", &preview[..100])
            } else if preview.is_empty() {
                "(completed)".to_string()
            } else {
                preview
            };
            println!("    → Agent done: {preview}");
            Ok(())
        }
        InvokeResult::NotAvailable { reason } => {
            println!("    ⚠ {reason}");
            println!("    → Run manually: /solidspec-{phase} {feature_dir_name}");
            if !auto {
                print!("    ⏳ Press Enter when done (or Ctrl+C to abort)... ");
                std::io::stdout().flush()?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
            } else {
                println!("    [auto] Continuing without agent");
            }
            Ok(())
        }
        InvokeResult::Failed { error } => {
            println!("    ⚠ Agent failed: {error}");
            println!("    → Run manually: /solidspec-{phase} {feature_dir_name}");
            if !auto {
                print!("    ⏳ Press Enter to continue or Ctrl+C to abort... ");
                std::io::stdout().flush()?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
            } else {
                println!("    [auto] Continuing despite agent failure");
            }
            Ok(())
        }
    }
}

fn skip_reason(phase: &str, _feature_dir: &std::path::Path) -> String {
    match phase {
        "intent" => "intent.md already exists".into(),
        "specify" => "spec.md already exists".into(),
        "clarify" => "no [NEEDS CLARIFICATION] markers".into(),
        "plan" => "plan.md already exists".into(),
        "tasks" => "tasks.md already exists".into(),
        "tests" => "tests/ directory exists".into(),
        "implement" => "all tasks completed".into(),
        "apex" => "apex/*/09-finish.md exists".into(),
        "evidence" => "evidence-report.md already exists".into(),
        "analyze" => "never skipped".into(),
        "review" => "review-report.md already exists".into(),
        _ => "condition met".into(),
    }
}
