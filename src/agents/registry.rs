#![allow(dead_code)]
use std::path::{Path, PathBuf};

use anyhow::Result;

use super::config::{AGENTS, AgentConfig, find_agent};
use super::formats;
use super::guardrails;
use crate::core::apex;

/// Embedded default slash-command bodies, one per named workflow phase.
/// Each contains the canonical `$ARGUMENTS` placeholder — `register_commands`
/// converts it to the target agent's placeholder afterward via
/// `formats::translate_placeholder`, so these bodies are agent-agnostic.
mod command_bodies {
    pub const SPECIFY: &str = include_str!("../../templates/commands/specify.md");
    pub const CLARIFY: &str = include_str!("../../templates/commands/clarify.md");
    pub const PLAN: &str = include_str!("../../templates/commands/plan.md");
    pub const TASKS: &str = include_str!("../../templates/commands/tasks.md");
    pub const IMPLEMENT: &str = include_str!("../../templates/commands/implement.md");
    pub const TESTS: &str = include_str!("../../templates/commands/tests.md");
    pub const ANALYZE: &str = include_str!("../../templates/commands/analyze.md");
    pub const REVIEW: &str = include_str!("../../templates/commands/review.md");
    pub const APEX: &str = include_str!("../../templates/commands/apex.md");
    pub const TDD_TESTS: &str = include_str!("../../templates/commands/tdd-tests.md");
    pub const TDD_REFACTOR: &str = include_str!("../../templates/commands/tdd-refactor.md");
}

/// Resolve a command's body: a project-local override at
/// `.solidspec/templates/overrides/commands/<cmd_name>.md` wins if present,
/// otherwise the embedded default for known phases, otherwise a generic
/// fallback for phases with no dedicated body (e.g. `checklist`).
fn command_body(cmd_name: &str, project_root: &Path) -> String {
    let override_path = project_root
        .join(".solidspec/templates/overrides/commands")
        .join(format!("{cmd_name}.md"));
    if let Ok(content) = std::fs::read_to_string(&override_path) {
        return content;
    }

    match cmd_name {
        "specify" => command_bodies::SPECIFY.to_string(),
        "clarify" => command_bodies::CLARIFY.to_string(),
        "plan" => command_bodies::PLAN.to_string(),
        "tasks" => command_bodies::TASKS.to_string(),
        "implement" => command_bodies::IMPLEMENT.to_string(),
        "tests" => command_bodies::TESTS.to_string(),
        "analyze" => command_bodies::ANALYZE.to_string(),
        "review" => command_bodies::REVIEW.to_string(),
        "apex" => command_bodies::APEX.to_string(),
        "tdd-tests" => command_bodies::TDD_TESTS.to_string(),
        "tdd-refactor" => command_bodies::TDD_REFACTOR.to_string(),
        _ => format!(
            "Read the project context from .solidspec/AGENT.md, then execute the '{cmd_name}' workflow for the feature specified by $ARGUMENTS."
        ),
    }
}

/// Detected agent in a repository.
#[derive(Debug, Clone)]
pub struct DetectedAgent {
    pub config: &'static AgentConfig,
    pub dir_exists: bool,
    pub cli_available: bool,
}

/// SolidSpec commands to register with agents.
const COMMANDS: &[(&str, &str)] = &[
    ("specify", "Create a new feature specification"),
    ("clarify", "Resolve ambiguities in a specification"),
    ("plan", "Generate an architecture plan from a specification"),
    (
        "tasks",
        "Generate a story-driven task breakdown from the plan",
    ),
    ("implement", "Execute tasks from the task breakdown"),
    ("tests", "Generate test scaffolds from acceptance scenarios"),
    ("analyze", "Validate cross-artifact consistency"),
    ("review", "Review spec quality with preflight heuristics"),
    ("checklist", "Generate a quality validation checklist"),
    (
        "apex",
        "Launch the APEX implementation workflow (Analyze-Plan-Execute-eXamine)",
    ),
    (
        "tdd-tests",
        "Generate real failing tests for every acceptance criterion (TDD RED phase)",
    ),
    (
        "tdd-refactor",
        "Refactor implementation while keeping all tests GREEN (TDD REFACTOR phase)",
    ),
];

/// Detect all agents present in the repository.
pub fn detect_agents(project_root: &Path) -> Vec<DetectedAgent> {
    AGENTS
        .iter()
        .map(|agent| {
            let agent_path = project_root.join(agent.command_dir);
            let dir_exists = agent_path.exists();
            let cli_available = if !agent.cli_binary.is_empty() {
                find_binary(agent.cli_binary).is_some()
            } else {
                false
            };
            DetectedAgent {
                config: agent,
                dir_exists,
                cli_available,
            }
        })
        .collect()
}

/// Register SolidSpec commands for a specific agent.
pub fn register_commands(project_root: &Path, agent: &AgentConfig) -> Result<()> {
    let cmd_dir = project_root
        .join(agent.command_dir)
        .join(agent.commands_subdir);
    std::fs::create_dir_all(&cmd_dir)?;

    for (cmd_name, description) in COMMANDS {
        let body = command_body(cmd_name, project_root);
        let mut body = formats::translate_placeholder(&body, agent.arg_placeholder);
        body.push('\n');
        body.push_str(&guardrails::compliance_footer());

        // Copilot: .agent.md and .prompt.md use different frontmatter formats
        if agent.id == "copilot" {
            let agent_content =
                formats::adjust_script_paths(&formats::render_copilot_agent(description, &body));
            let prompt_content =
                formats::adjust_script_paths(&formats::render_copilot_prompt(description, &body));
            let agents_dir = project_root
                .join(agent.command_dir)
                .join(agent.commands_subdir);
            let file_name = format!("solidspec-{cmd_name}{}", agent.extension);
            std::fs::write(agents_dir.join(&file_name), &agent_content)?;

            let prompts_dir = project_root.join(".github/prompts");
            std::fs::create_dir_all(&prompts_dir)?;
            std::fs::write(
                prompts_dir.join(format!("solidspec-{cmd_name}.prompt.md")),
                &prompt_content,
            )?;
            continue;
        }

        let content = if agent.id == "vibe" {
            let rendered = formats::render_vibe_skill(cmd_name, description, &body);
            formats::adjust_script_paths(&rendered)
        } else if agent.id == "opencode" {
            let rendered = formats::render_opencode_skill(cmd_name, description, &body);
            formats::adjust_script_paths(&rendered)
        } else {
            let rendered = formats::render_command(agent.format, description, &body);
            formats::adjust_script_paths(&rendered)
        };

        write_command_file(project_root, agent, cmd_name, &content)?;
    }

    Ok(())
}

/// Write a single command file, handling agent-specific paths.
fn write_command_file(
    project_root: &Path,
    agent: &AgentConfig,
    cmd_name: &str,
    content: &str,
) -> Result<()> {
    let cmd_dir = project_root
        .join(agent.command_dir)
        .join(agent.commands_subdir);

    if agent.id == "kimi" {
        // Kimi: directory-based skills with dot-separator
        let skill_name = formats::kimi_command_name(cmd_name);
        let skill_dir = cmd_dir.join(&skill_name);
        std::fs::create_dir_all(&skill_dir)?;
        std::fs::write(skill_dir.join("SKILL.md"), content)?;
    } else if agent.id == "vibe" || agent.id == "opencode" {
        // Vibe/OpenCode: directory-based skills with hyphen-separator (SKILL.md)
        let skill_name = formats::standard_command_name(cmd_name);
        let skill_dir = cmd_dir.join(&skill_name);
        std::fs::create_dir_all(&skill_dir)?;
        std::fs::write(skill_dir.join("SKILL.md"), content)?;
    } else {
        // Standard: flat file with hyphen-separator
        let file_name = format!(
            "{}{}",
            formats::standard_command_name(cmd_name),
            agent.extension
        );
        std::fs::write(cmd_dir.join(&file_name), content)?;
    }

    Ok(())
}

/// Return the directory where APEX skill files should be written for a given agent.
fn apex_skill_dir(agent_id: &str, project_root: &Path) -> Option<PathBuf> {
    match agent_id {
        "claude" => Some(project_root.join(".claude/commands/apex")),
        "kimi" => Some(project_root.join(".kimi/skills/apex")),
        "vibe" => Some(project_root.join(".vibe/skills/apex")),
        "opencode" => Some(project_root.join(".opencode/skills/apex")),
        _ => None,
    }
}

/// Extract the APEX skill files into the agent's skill directory.
/// Returns `Ok(true)` when the agent supports APEX, `Ok(false)` otherwise.
pub fn register_apex_skill(agent_id: &str, project_root: &Path) -> Result<bool> {
    match apex_skill_dir(agent_id, project_root) {
        Some(dir) => {
            apex::extract_skill(&dir)?;
            Ok(true)
        }
        None => Ok(false),
    }
}

/// Remove the APEX skill directory for the given agent (if supported).
pub fn unregister_apex_skill(agent_id: &str, project_root: &Path) -> Result<()> {
    if let Some(dir) = apex_skill_dir(agent_id, project_root)
        && dir.exists()
    {
        std::fs::remove_dir_all(&dir)?;
    }
    Ok(())
}

/// Unregister all SolidSpec commands for a specific agent.
pub fn unregister_commands(project_root: &Path, agent: &AgentConfig) -> Result<()> {
    let cmd_dir = project_root
        .join(agent.command_dir)
        .join(agent.commands_subdir);

    if !cmd_dir.exists() {
        return Ok(());
    }

    for (cmd_name, _) in COMMANDS {
        if agent.id == "kimi" {
            let skill_name = formats::kimi_command_name(cmd_name);
            let skill_dir = cmd_dir.join(&skill_name);
            if skill_dir.exists() {
                std::fs::remove_dir_all(&skill_dir)?;
            }
        } else if agent.id == "vibe" || agent.id == "opencode" {
            let skill_name = formats::standard_command_name(cmd_name);
            let skill_dir = cmd_dir.join(&skill_name);
            if skill_dir.exists() {
                std::fs::remove_dir_all(&skill_dir)?;
            }
        } else if agent.id == "copilot" {
            let file_name = format!("solidspec-{cmd_name}{}", agent.extension);
            let path = cmd_dir.join(&file_name);
            if path.exists() {
                std::fs::remove_file(&path)?;
            }

            // Remove companion .prompt.md
            let prompt = project_root
                .join(".github/prompts")
                .join(format!("solidspec-{cmd_name}.prompt.md"));
            if prompt.exists() {
                std::fs::remove_file(&prompt)?;
            }
        } else {
            let file_name = format!(
                "{}{}",
                formats::standard_command_name(cmd_name),
                agent.extension
            );
            let path = cmd_dir.join(&file_name);
            if path.exists() {
                std::fs::remove_file(&path)?;
            }
        }
    }

    unregister_apex_skill(agent.id, project_root)?;

    Ok(())
}

/// Register commands for all detected agents.
pub fn register_all(project_root: &Path, target_agent: Option<&str>) -> Result<Vec<String>> {
    let mut registered = Vec::new();

    if let Some(agent_id) = target_agent {
        // Register for a specific agent
        let agent = find_agent(agent_id).ok_or_else(|| {
            anyhow::anyhow!(
                "Unknown agent '{}'. Available: {}",
                agent_id,
                AGENTS.iter().map(|a| a.id).collect::<Vec<_>>().join(", ")
            )
        })?;

        let cmd_dir = project_root.join(agent.command_dir);
        std::fs::create_dir_all(cmd_dir.join(agent.commands_subdir))?;
        register_commands(project_root, agent)?;
        register_apex_skill(agent_id, project_root)?;
        registered.push(agent.id.to_string());
    } else {
        // Auto-detect: register for agents whose dir exists OR whose CLI is available
        let detected = detect_agents(project_root);
        for det in &detected {
            if det.dir_exists || det.cli_available {
                register_commands(project_root, det.config)?;
                register_apex_skill(det.config.id, project_root)?;
                registered.push(det.config.id.to_string());
            }
        }
    }

    Ok(registered)
}

/// Resolve a CLI binary by name, checking PATH first then common npm/nvm install locations.
pub fn find_binary(name: &str) -> Option<PathBuf> {
    // 1. Standard PATH lookup
    if let Ok(p) = which::which(name) {
        return Some(p);
    }

    // 2. nvm-managed Node.js installations (~/.nvm/versions/node/*/bin/<name>)
    if let Ok(home) = std::env::var("HOME") {
        let nvm_root = PathBuf::from(&home).join(".nvm/versions/node");
        if let Ok(entries) = std::fs::read_dir(&nvm_root) {
            let mut versions: Vec<_> = entries.flatten().collect();
            versions.sort_by_key(|b| std::cmp::Reverse(b.file_name()));
            for entry in versions {
                let bin = entry.path().join("bin").join(name);
                if bin.exists() {
                    return Some(bin);
                }
            }
        }

        // 3. npm global bin directories
        for npm_dir in &[".npm-global/bin", ".local/share/npm/bin"] {
            let bin = PathBuf::from(&home).join(npm_dir).join(name);
            if bin.exists() {
                return Some(bin);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn detect_claude_when_dir_exists() {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
        let detected = detect_agents(dir.path());
        let claude = detected.iter().find(|d| d.config.id == "claude").unwrap();
        assert!(claude.dir_exists);
    }

    #[test]
    fn detect_multiple_agents() {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
        std::fs::create_dir_all(dir.path().join(".cursor")).unwrap();
        let detected = detect_agents(dir.path());
        let present: Vec<_> = detected
            .iter()
            .filter(|d| d.dir_exists)
            .map(|d| d.config.id)
            .collect();
        assert!(present.contains(&"claude"));
        assert!(present.contains(&"cursor"));
    }

    #[test]
    fn empty_repo_detects_nothing() {
        let dir = TempDir::new().unwrap();
        let detected = detect_agents(dir.path());
        assert!(detected.iter().all(|d| !d.dir_exists));
    }

    #[test]
    fn register_markdown_agent_creates_md_files() {
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();

        let cmd_dir = dir.path().join(".claude/commands");
        assert!(cmd_dir.exists());

        let specify = cmd_dir.join("solidspec-specify.md");
        assert!(specify.exists());
        let content = std::fs::read_to_string(&specify).unwrap();
        assert!(content.starts_with("---\n"));
        assert!(content.contains("description:"));
        assert!(content.contains("$ARGUMENTS"));
    }

    #[test]
    fn command_body_generic_fallback_for_unknown_phase() {
        let dir = TempDir::new().unwrap();
        let body = command_body("checklist", dir.path());
        assert!(body.contains("'checklist' workflow"));
        assert!(body.contains("$ARGUMENTS"));
    }

    #[test]
    fn project_local_override_wins_over_embedded_command_body() {
        let dir = TempDir::new().unwrap();
        let overrides_dir = dir.path().join(".solidspec/templates/overrides/commands");
        std::fs::create_dir_all(&overrides_dir).unwrap();
        std::fs::write(overrides_dir.join("specify.md"), "CUSTOM SPECIFY BODY").unwrap();

        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();

        let content =
            std::fs::read_to_string(dir.path().join(".claude/commands/solidspec-specify.md"))
                .unwrap();
        assert!(content.contains("CUSTOM SPECIFY BODY"));
        // Guardrails footer must still be appended to the override.
        assert!(content.contains("Before You Skip Any Step"));
    }

    #[test]
    fn no_override_falls_back_to_embedded_default() {
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();

        let content =
            std::fs::read_to_string(dir.path().join(".claude/commands/solidspec-specify.md"))
                .unwrap();
        assert!(content.contains("Replace [Brief Title] with a descriptive story title"));
    }

    #[test]
    fn register_toml_agent_creates_toml_files() {
        let dir = TempDir::new().unwrap();
        let gemini = find_agent("gemini").unwrap();
        register_commands(dir.path(), gemini).unwrap();

        let cmd_dir = dir.path().join(".gemini/commands");
        let specify = cmd_dir.join("solidspec-specify.toml");
        assert!(specify.exists());
        let content = std::fs::read_to_string(&specify).unwrap();
        assert!(content.contains("description = "));
        assert!(content.contains("prompt = \"\"\""));
        assert!(content.contains("{{args}}"));
        assert!(!content.contains("$ARGUMENTS"));
    }

    #[test]
    fn copilot_creates_agent_md_and_prompt_md() {
        let dir = TempDir::new().unwrap();
        let copilot = find_agent("copilot").unwrap();
        register_commands(dir.path(), copilot).unwrap();

        // .agent.md in .github/agents/
        let agent_file = dir.path().join(".github/agents/solidspec-specify.agent.md");
        assert!(agent_file.exists());

        // .prompt.md in .github/prompts/
        let prompt_file = dir
            .path()
            .join(".github/prompts/solidspec-specify.prompt.md");
        assert!(prompt_file.exists());
    }

    #[test]
    fn kimi_creates_directory_based_skills() {
        let dir = TempDir::new().unwrap();
        let kimi = find_agent("kimi").unwrap();
        register_commands(dir.path(), kimi).unwrap();

        // Directory-based: .kimi/skills/solidspec.specify/SKILL.md
        let skill = dir.path().join(".kimi/skills/solidspec.specify/SKILL.md");
        assert!(
            skill.exists(),
            "Kimi skill not found at {}",
            skill.display()
        );
    }

    #[test]
    fn unregister_removes_copilot_files() {
        let dir = TempDir::new().unwrap();
        let copilot = find_agent("copilot").unwrap();
        register_commands(dir.path(), copilot).unwrap();

        let agent_file = dir.path().join(".github/agents/solidspec-specify.agent.md");
        let prompt_file = dir
            .path()
            .join(".github/prompts/solidspec-specify.prompt.md");
        assert!(agent_file.exists());
        assert!(prompt_file.exists());

        unregister_commands(dir.path(), copilot).unwrap();
        assert!(!agent_file.exists());
        assert!(!prompt_file.exists());
    }

    #[test]
    fn unregister_removes_kimi_dirs() {
        let dir = TempDir::new().unwrap();
        let kimi = find_agent("kimi").unwrap();
        register_commands(dir.path(), kimi).unwrap();
        unregister_commands(dir.path(), kimi).unwrap();

        let skill = dir.path().join(".kimi/skills/solidspec.specify");
        assert!(!skill.exists());
    }

    #[test]
    fn register_all_with_specific_agent() {
        let dir = TempDir::new().unwrap();
        let registered = register_all(dir.path(), Some("claude")).unwrap();
        assert_eq!(registered, vec!["claude"]);
        assert!(
            dir.path()
                .join(".claude/commands/solidspec-specify.md")
                .exists()
        );
    }

    #[test]
    fn register_all_with_invalid_agent_returns_error() {
        let dir = TempDir::new().unwrap();
        let result = register_all(dir.path(), Some("nonexistent"));
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Unknown agent"));
        assert!(err.contains("Available:"));
    }

    #[test]
    fn register_all_auto_detect() {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
        std::fs::create_dir_all(dir.path().join(".cursor")).unwrap();

        let registered = register_all(dir.path(), None).unwrap();
        assert!(registered.contains(&"claude".to_string()));
        assert!(registered.contains(&"cursor".to_string()));
    }

    #[test]
    fn vibe_creates_directory_based_skills() {
        let dir = TempDir::new().unwrap();
        let vibe = find_agent("vibe").unwrap();
        register_commands(dir.path(), vibe).unwrap();

        // Directory-based: .vibe/skills/solidspec-specify/SKILL.md
        let skill = dir.path().join(".vibe/skills/solidspec-specify/SKILL.md");
        assert!(
            skill.exists(),
            "Vibe skill not found at {}",
            skill.display()
        );

        let content = std::fs::read_to_string(&skill).unwrap();
        assert!(content.contains("name: solidspec-specify"));
        assert!(content.contains("user-invocable: true"));
        assert!(content.contains("allowed-tools:"));
    }

    #[test]
    fn unregister_removes_vibe_dirs() {
        let dir = TempDir::new().unwrap();
        let vibe = find_agent("vibe").unwrap();
        register_commands(dir.path(), vibe).unwrap();
        unregister_commands(dir.path(), vibe).unwrap();

        let skill = dir.path().join(".vibe/skills/solidspec-specify");
        assert!(!skill.exists());
    }

    #[test]
    fn opencode_creates_directory_based_skills() {
        let dir = TempDir::new().unwrap();
        let opencode = find_agent("opencode").unwrap();
        register_commands(dir.path(), opencode).unwrap();

        let skill = dir
            .path()
            .join(".opencode/skills/solidspec-specify/SKILL.md");
        assert!(
            skill.exists(),
            "OpenCode skill not found at {}",
            skill.display()
        );

        let content = std::fs::read_to_string(&skill).unwrap();
        assert!(content.starts_with("---\n"), "Missing YAML frontmatter");
        assert!(
            content.contains("name: solidspec-specify"),
            "Missing name field"
        );
        assert!(
            content.contains("description:"),
            "Missing description field"
        );
        assert!(
            content.contains("Before You Skip Any Step"),
            "Missing compliance guardrails"
        );
    }

    #[test]
    fn unregister_removes_opencode_skills() {
        let dir = TempDir::new().unwrap();
        let opencode = find_agent("opencode").unwrap();
        register_commands(dir.path(), opencode).unwrap();
        unregister_commands(dir.path(), opencode).unwrap();

        let skill = dir.path().join(".opencode/skills/solidspec-specify");
        assert!(!skill.exists());
    }

    #[test]
    fn kimi_uses_dot_separator_others_use_hyphen() {
        let dir = TempDir::new().unwrap();

        // Kimi: dot separator
        let kimi = find_agent("kimi").unwrap();
        register_commands(dir.path(), kimi).unwrap();
        assert!(
            dir.path()
                .join(".kimi/skills/solidspec.specify/SKILL.md")
                .exists()
        );

        // Claude: hyphen separator
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();
        assert!(
            dir.path()
                .join(".claude/commands/solidspec-specify.md")
                .exists()
        );
    }

    #[test]
    fn command_files_contain_compliance_guardrails() {
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();

        let content =
            std::fs::read_to_string(dir.path().join(".claude/commands/solidspec-specify.md"))
                .unwrap();
        assert!(content.contains("Before You Skip Any Step"));
        assert!(content.contains("Mandatory Verification Checklist"));
        assert!(content.contains("[NEEDS CLARIFICATION]"));
    }

    // ── APEX skill registration tests ──────────────────────────────────────

    #[test]
    fn apex_skill_dir_returns_correct_paths() {
        let dir = TempDir::new().unwrap();
        assert_eq!(
            apex_skill_dir("claude", dir.path()),
            Some(dir.path().join(".claude/commands/apex"))
        );
        assert_eq!(
            apex_skill_dir("kimi", dir.path()),
            Some(dir.path().join(".kimi/skills/apex"))
        );
        assert_eq!(
            apex_skill_dir("vibe", dir.path()),
            Some(dir.path().join(".vibe/skills/apex"))
        );
        assert_eq!(
            apex_skill_dir("opencode", dir.path()),
            Some(dir.path().join(".opencode/skills/apex"))
        );
        assert_eq!(apex_skill_dir("cursor", dir.path()), None);
        assert_eq!(apex_skill_dir("gemini", dir.path()), None);
    }

    #[test]
    fn register_apex_skill_creates_files_for_claude() {
        let dir = TempDir::new().unwrap();
        let did_register = register_apex_skill("claude", dir.path()).unwrap();
        assert!(did_register);

        let skill_dir = dir.path().join(".claude/commands/apex");
        assert!(skill_dir.exists(), "apex skill dir not created");
        assert!(skill_dir.join("SKILL.md").exists(), "SKILL.md missing");
        assert!(skill_dir.join("steps").is_dir(), "steps/ subdir missing");
        assert!(
            skill_dir.join("templates").is_dir(),
            "templates/ subdir missing"
        );
    }

    #[test]
    fn register_apex_skill_returns_false_for_unsupported_agent() {
        let dir = TempDir::new().unwrap();
        let did_register = register_apex_skill("cursor", dir.path()).unwrap();
        assert!(!did_register);
        assert!(!dir.path().join(".cursor/commands/apex").exists());
    }

    #[test]
    fn unregister_apex_skill_removes_directory() {
        let dir = TempDir::new().unwrap();
        register_apex_skill("claude", dir.path()).unwrap();

        let skill_dir = dir.path().join(".claude/commands/apex");
        assert!(skill_dir.exists());

        unregister_apex_skill("claude", dir.path()).unwrap();
        assert!(!skill_dir.exists());
    }

    #[test]
    fn unregister_apex_skill_is_idempotent() {
        let dir = TempDir::new().unwrap();
        // No prior registration — must not error
        unregister_apex_skill("claude", dir.path()).unwrap();
        unregister_apex_skill("cursor", dir.path()).unwrap();
    }

    #[test]
    fn register_all_also_registers_apex_skill_for_claude() {
        let dir = TempDir::new().unwrap();
        register_all(dir.path(), Some("claude")).unwrap();

        let apex_dir = dir.path().join(".claude/commands/apex");
        assert!(
            apex_dir.exists(),
            "APEX skill dir missing after register_all"
        );
        assert!(apex_dir.join("SKILL.md").exists());
    }

    #[test]
    fn unregister_removes_apex_skill_directory() {
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();
        register_apex_skill("claude", dir.path()).unwrap();

        let apex_dir = dir.path().join(".claude/commands/apex");
        assert!(apex_dir.exists());

        unregister_commands(dir.path(), claude).unwrap();
        assert!(
            !apex_dir.exists(),
            "APEX dir should be removed by unregister_commands"
        );
    }

    #[test]
    fn apex_command_file_contains_apex_workflow_text() {
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();

        let content =
            std::fs::read_to_string(dir.path().join(".claude/commands/solidspec-apex.md")).unwrap();
        assert!(content.contains("APEX"), "missing APEX keyword");
        assert!(content.contains("Analyze"), "missing Analyze step");
        assert!(content.contains("eXamine"), "missing eXamine step");
        assert!(
            content.contains("apex-context.md"),
            "missing context file ref"
        );
    }
}
