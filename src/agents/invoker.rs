use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

use super::config::{AgentConfig, find_agent};
use super::registry::find_binary;

/// Build the prompt for a pipeline phase, with detailed per-phase instructions.
pub fn build_phase_prompt(
    phase: &str,
    feature_dir_name: &str,
    description: Option<&str>,
) -> String {
    let specs_path = format!("specs/{feature_dir_name}");

    match phase {
        "specify" => {
            let desc = description.unwrap_or(feature_dir_name);
            format!(
                "Read the project context from .rustyspec/AGENT.md.\n\n\
                 Feature: {desc}\n\
                 Feature directory: {specs_path}\n\n\
                 Fill in {specs_path}/spec.md with real content based on the feature description above.\n\n\
                 Steps:\n\
                 1. Replace [Brief Title] with a descriptive story title\n\
                 2. Write user stories with clear Given/When/Then acceptance scenarios\n\
                 3. Define functional requirements (FR-001, FR-002, etc.)\n\
                 4. Identify key entities and their relationships\n\
                 5. Define measurable success criteria\n\
                 6. List edge cases\n\n\
                 Keep requirements technology-agnostic. Focus on WHAT, not HOW.\n\
                 Do NOT create new files. Only edit the existing {specs_path}/spec.md."
            )
        }
        "clarify" => {
            format!(
                "Read the project context from .rustyspec/AGENT.md.\n\n\
                 Feature: {specs_path}\n\n\
                 Read {specs_path}/spec.md and find all [NEEDS CLARIFICATION] markers.\n\
                 For each marker:\n\
                 1. Identify the ambiguity\n\
                 2. Propose a resolution based on best practices\n\
                 3. Update spec.md with the resolution\n\
                 4. Remove the [NEEDS CLARIFICATION] marker\n\n\
                 Do NOT create new files. Only edit {specs_path}/spec.md."
            )
        }
        "plan" => {
            format!(
                "Read the project context from .rustyspec/AGENT.md.\n\n\
                 Feature: {specs_path}\n\
                 Read {specs_path}/spec.md for requirements.\n\n\
                 Fill in the planning documents with real content:\n\
                 1. {specs_path}/plan.md — Architecture decisions, tech stack, project structure, constitution check\n\
                 2. {specs_path}/research.md — Technology investigation findings\n\
                 3. {specs_path}/data-model.md — Entity definitions and relationships\n\
                 4. {specs_path}/contracts/api.md — API contracts if applicable\n\
                 5. {specs_path}/quickstart.md — Key validation scenarios\n\n\
                 Complete the Constitution Check in plan.md.\n\
                 Fill all [NEEDS CLARIFICATION] and [To be filled] sections with concrete content.\n\
                 Only edit existing files in {specs_path}/."
            )
        }
        "tasks" => {
            format!(
                "Read the project context from .rustyspec/AGENT.md.\n\n\
                 Feature: {specs_path}\n\
                 Read {specs_path}/spec.md and {specs_path}/plan.md.\n\n\
                 Fill in {specs_path}/tasks.md with concrete tasks:\n\
                 1. Define specific, actionable tasks with clear deliverables\n\
                 2. Organize by phases (Setup → Foundational → User Stories → Polish)\n\
                 3. Mark parallel-safe tasks with [P]\n\
                 4. Link tasks to user stories with [US1], [US2], etc.\n\
                 5. Each task should be independently completable\n\
                 6. Replace all placeholder text like [Brief Title] with real content\n\n\
                 Only edit the existing {specs_path}/tasks.md."
            )
        }
        "tests" => {
            format!(
                "Read the project context from .rustyspec/AGENT.md.\n\n\
                 Feature: {specs_path}\n\
                 Read {specs_path}/spec.md for acceptance scenarios.\n\n\
                 Review and enhance test scaffolds in {specs_path}/tests/:\n\
                 1. Add concrete test implementations for each Given/When/Then scenario\n\
                 2. Replace placeholder text with real test assertions\n\
                 3. Add edge case tests based on the spec\n\
                 4. Ensure tests are runnable with the project's test framework\n\n\
                 Only edit existing files in {specs_path}/tests/."
            )
        }
        "analyze" => {
            format!(
                "Read the project context from .rustyspec/AGENT.md.\n\n\
                 Feature: {specs_path}\n\n\
                 Validate cross-artifact consistency:\n\
                 1. Check that plan.md addresses all requirements from spec.md\n\
                 2. Check that tasks.md covers all planned work\n\
                 3. Check that tests cover all acceptance scenarios\n\
                 4. Report any gaps or inconsistencies\n\
                 5. Suggest improvements if any artifacts are incomplete"
            )
        }
        "review" => {
            format!(
                "Read the project context from .rustyspec/AGENT.md.\n\n\
                 Feature: {specs_path}\n\n\
                 Perform a comprehensive spec quality review:\n\
                 1. Check for placeholder text and incomplete sections\n\
                 2. Validate requirement quality and testability\n\
                 3. Check cross-artifact consistency (spec → plan → tasks)\n\
                 4. Assess security, performance, and maintainability concerns\n\
                 5. Write findings to {specs_path}/review-report.md"
            )
        }
        _ => {
            format!(
                "Read the project context from .rustyspec/AGENT.md, then execute the '{phase}' workflow for feature {specs_path}."
            )
        }
    }
}

/// Result of an agent CLI invocation.
#[derive(Debug)]
pub enum InvokeResult {
    /// Agent was invoked successfully
    Success { output: String },
    /// Agent CLI is not available — fall back to handoff
    NotAvailable { reason: String },
    /// Agent invocation failed
    Failed { error: String },
}

/// Invoke an AI agent's CLI to process a pipeline phase.
///
/// Returns `InvokeResult::NotAvailable` if the agent doesn't support CLI invocation,
/// allowing the pipeline to fall back to handoff mode.
pub fn invoke_agent(
    agent_id: &str,
    phase: &str,
    feature_dir_name: &str,
    project_root: &Path,
    description: Option<&str>,
) -> InvokeResult {
    let agent = match find_agent(agent_id) {
        Some(a) => a,
        None => {
            return InvokeResult::NotAvailable {
                reason: format!("Unknown agent '{agent_id}'"),
            };
        }
    };

    // Check if agent has CLI support
    if agent.cli_binary.is_empty() {
        return InvokeResult::NotAvailable {
            reason: format!("{} does not support CLI invocation", agent.name),
        };
    }

    // Check if CLI binary is available (checks PATH and common npm/nvm locations)
    let binary_path = match find_binary(agent.cli_binary) {
        Some(p) => p,
        None => {
            return InvokeResult::NotAvailable {
                reason: format!(
                    "'{}' CLI not found. Install {} or add it to PATH.",
                    agent.cli_binary, agent.name
                ),
            };
        }
    };

    let prompt = build_phase_prompt(phase, feature_dir_name, description);

    match run_agent_cli(agent, &binary_path, &prompt, project_root) {
        Ok(output) => InvokeResult::Success { output },
        Err(e) => InvokeResult::Failed {
            error: format!("{e}"),
        },
    }
}

/// Execute the agent CLI process with the given prompt.
fn run_agent_cli(agent: &AgentConfig, binary_path: &Path, prompt: &str, working_dir: &Path) -> Result<String> {
    let mut cmd = Command::new(binary_path);
    cmd.current_dir(working_dir);

    // Special handling for agents with non-standard invocation
    match agent.id {
        "codex" => {
            // codex uses subcommand: `codex exec "prompt"`
            cmd.arg("exec").arg(prompt);
        }
        "kimi" => {
            // kimi uses: `kimi --yolo` with prompt piped or as positional
            cmd.arg("--yolo").arg(prompt);
        }
        _ => {
            // Standard: `binary <prompt_flag> "prompt"`
            cmd.arg(agent.cli_prompt_flag).arg(prompt);
        }
    }

    // Add extra flags
    for flag in agent.cli_extra_flags {
        cmd.arg(flag);
    }

    log::info!(
        "Invoking {} CLI: {} {} ...",
        agent.name,
        agent.cli_binary,
        agent.cli_prompt_flag
    );
    log::debug!("Prompt length: {} chars", prompt.len());

    let output = cmd
        .output()
        .with_context(|| format!("Failed to execute '{}' CLI", agent.cli_binary))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        log::info!("{} CLI completed successfully", agent.name);
        if !stderr.is_empty() {
            log::debug!("stderr: {}", &stderr[..stderr.len().min(500)]);
        }
        Ok(stdout)
    } else {
        let code = output.status.code().unwrap_or(-1);
        anyhow::bail!(
            "{} CLI exited with code {}: {}",
            agent.name,
            code,
            if stderr.is_empty() { &stdout } else { &stderr }
        )
    }
}

/// Check if an agent supports CLI invocation.
pub fn supports_cli(agent_id: &str) -> bool {
    find_agent(agent_id)
        .map(|a| !a.cli_binary.is_empty() && find_binary(a.cli_binary).is_some())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_specify_prompt_includes_feature() {
        let prompt = build_phase_prompt("specify", "001-auth", Some("User authentication"));
        assert!(prompt.contains("User authentication"));
        assert!(prompt.contains("specs/001-auth/spec.md"));
        assert!(prompt.contains("Given/When/Then"));
    }

    #[test]
    fn build_plan_prompt_includes_all_docs() {
        let prompt = build_phase_prompt("plan", "001-auth", None);
        assert!(prompt.contains("plan.md"));
        assert!(prompt.contains("research.md"));
        assert!(prompt.contains("data-model.md"));
        assert!(prompt.contains("contracts/api.md"));
        assert!(prompt.contains("quickstart.md"));
    }

    #[test]
    fn build_tasks_prompt_mentions_phases() {
        let prompt = build_phase_prompt("tasks", "001-auth", None);
        assert!(prompt.contains("Setup"));
        assert!(prompt.contains("Foundational"));
        assert!(prompt.contains("[US1]"));
    }

    #[test]
    fn build_clarify_prompt_mentions_markers() {
        let prompt = build_phase_prompt("clarify", "001-auth", None);
        assert!(prompt.contains("[NEEDS CLARIFICATION]"));
    }

    #[test]
    fn build_tests_prompt_mentions_scaffolds() {
        let prompt = build_phase_prompt("tests", "001-auth", None);
        assert!(prompt.contains("test scaffolds"));
        assert!(prompt.contains("tests/"));
    }

    #[test]
    fn build_analyze_prompt_mentions_consistency() {
        let prompt = build_phase_prompt("analyze", "001-auth", None);
        assert!(prompt.contains("consistency"));
    }

    #[test]
    fn build_unknown_phase_returns_generic() {
        let prompt = build_phase_prompt("unknown-phase", "001-auth", None);
        assert!(prompt.contains("unknown-phase"));
    }

    #[test]
    fn invoke_unknown_agent_returns_not_available() {
        let result = invoke_agent("nonexistent", "specify", "001", Path::new("."), None);
        matches!(result, InvokeResult::NotAvailable { .. });
    }

    #[test]
    fn invoke_no_cli_agent_returns_not_available() {
        // windsurf has empty cli_binary
        let result = invoke_agent("windsurf", "specify", "001", Path::new("."), None);
        matches!(result, InvokeResult::NotAvailable { .. });
    }

    #[test]
    fn supports_cli_false_for_no_binary() {
        assert!(!supports_cli("windsurf"));
        assert!(!supports_cli("kilocode"));
        assert!(!supports_cli("bob"));
    }

    #[test]
    fn supports_cli_false_for_unknown() {
        assert!(!supports_cli("nonexistent-agent"));
    }
}
