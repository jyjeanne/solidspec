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
    pub const SECURITY_REVIEW: &str = include_str!("../../templates/commands/security-review.md");
    pub const APEX: &str = include_str!("../../templates/commands/apex.md");
    pub const TDD_TESTS: &str = include_str!("../../templates/commands/tdd-tests.md");
    pub const TDD_REFACTOR: &str = include_str!("../../templates/commands/tdd-refactor.md");

    /// `/spcx:explore` is the one meta command with no schema-specific
    /// content (no files written, nothing to chain) — see `super::spcx` for
    /// the other three (`new`/`apply`/`finalise`), generated per schema
    /// instead of embedded statically.
    pub const SPCX_EXPLORE: &str = include_str!("../../templates/commands/spcx/explore.md");
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
        "security-review" => command_bodies::SECURITY_REVIEW.to_string(),
        "apex" => command_bodies::APEX.to_string(),
        "tdd-tests" => command_bodies::TDD_TESTS.to_string(),
        "tdd-refactor" => command_bodies::TDD_REFACTOR.to_string(),
        "spcx-explore" => command_bodies::SPCX_EXPLORE.to_string(),
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
    (
        "security-review",
        "Deepen the OWASP Top 10 heuristic audit of plan.md (security-first workflow)",
    ),
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
    // /spcx:explore: the one meta command with no schema-specific content
    // (see `super::spcx` and `all_schema_spcx_commands` below for the other 3).
    (
        "spcx-explore",
        "Exploratory research and discussion — no files written",
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
    let commands: Vec<(String, &str, String)> = COMMANDS
        .iter()
        .map(|(cmd_name, description)| {
            (
                cmd_name.to_string(),
                *description,
                command_body(cmd_name, project_root),
            )
        })
        .collect();
    write_commands_for_agent(project_root, agent, &commands)
}

/// The 3 meta-command phases, and their descriptions, generated per schema
/// by `super::spcx::generate_bodies` and consumed by `all_schema_spcx_commands`
/// below.
const SPCX_PHASES: &[(&str, &str)] = &[
    (
        "new",
        "Start a feature end-to-end through the implement handoff",
    ),
    ("apply", "Implement the feature's tasks"),
    (
        "finalise",
        "Whatever comes after implement for this schema (analyze/review/ship, or nothing)",
    ),
];

/// Builds the `/spcx:<short>:{new,apply,finalise}` `(name, description,
/// body)` triples for every built-in workflow schema
/// (docs/simplification-study-openspec.md's slash-command design, extended
/// so an agent isn't limited to the project's own default schema): lets an
/// AI agent run any workflow's DAG-specific steps — `/spcx:tdd:new`,
/// `/spcx:sec:apply`, ... — without changing `solidspec.toml`'s
/// `[pipeline].schema`. There is deliberately no flagless `/spcx:new`
/// shorthand any more: the schema is always a namespace segment
/// (`<namespace>:<domain>:<action>`), never baked into the action name or
/// left implicit, so `default_schema_name` below only matters when it names
/// a schema outside the 7 built-ins.
///
/// Project-local overrides at `.solidspec/workflows/<name>/schema.yaml`
/// (via `schema::resolve_schema`) apply here too, same as everywhere else a
/// schema name is resolved — a customized `spec-driven` changes what
/// `/spcx:sdd:*` says without this function needing to know that.
///
/// `default_schema_name` is the project's actually-configured schema
/// *identifier* — `solidspec.toml`'s `[pipeline].schema` string, the same
/// one `schema::resolve_schema` takes to locate a `.solidspec/workflows/`
/// override or a built-in on disk — not a `WorkflowSchema.name` read back
/// out of already-parsed YAML. Those two can disagree: nothing enforces
/// that a project-local `schema.yaml`'s own `name:` field matches the
/// directory it lives in (e.g. a custom schema started as a copy of
/// `spec-driven`'s `schema.yaml` with the `name:` field left unedited), and
/// namespacing this function's output by the wrong one would either drop
/// the custom schema's commands entirely or generate them under the wrong
/// short name. Resolving by identifier here — the same string used to look
/// the file up in the first place — is what keeps `/spcx:<name>:*` in sync
/// with whatever `--schema <name>` / `solidspec.toml` actually names.
///
/// `default_schema_name` is folded into the 7 built-in names so a project
/// running a fully custom-named schema (one not among the 7) still gets its
/// own `/spcx:<name>:*` commands instead of none — the built-ins alone
/// would silently drop it now that there's no flagless fallback to catch
/// it.
///
/// Schema-independent of any one agent — callers compute this once and
/// pass it to `write_commands_for_agent` per agent (see `register_all`)
/// rather than re-resolving every schema and regenerating its bodies once
/// per agent for identical output.
fn all_schema_spcx_commands(
    project_root: &Path,
    default_schema_name: &str,
) -> Result<Vec<(String, &'static str, String)>> {
    let mut commands = Vec::new();

    for schema_name in crate::core::schema::builtin::names() {
        let short = super::spcx::schema_short_name(schema_name);
        let (schema, _) = crate::core::schema::resolve_schema(schema_name, project_root)?;
        push_spcx_commands(&mut commands, &short, &schema)?;
    }

    // `default_schema_name` names something outside the 7 built-ins (a
    // fully custom `.solidspec/workflows/<name>/schema.yaml`).
    if !crate::core::schema::builtin::names().contains(&default_schema_name) {
        let short = super::spcx::schema_short_name(default_schema_name);
        // A custom schema whose short name collides with a built-in's (e.g.
        // named exactly "tdd", matching tdd-driven's reduced name — compared
        // case-insensitively since `.claude/commands/spcx/<short>/` is a
        // real directory and several common filesystems, including macOS's
        // default APFS and Windows's NTFS, are case-insensitive, so "TDD"
        // and "tdd" would collide on disk even though they compare unequal
        // as plain strings) would otherwise silently clobber that built-in's
        // /spcx:<short>:* files — write_commands_for_agent writes both to
        // the identical path with no warning. Fail loudly instead, same as
        // init's own unknown-schema check, rather than let one workflow's
        // commands overwrite another's.
        if crate::core::schema::builtin::names()
            .iter()
            .any(|builtin_name| {
                super::spcx::schema_short_name(builtin_name).eq_ignore_ascii_case(&short)
            })
        {
            anyhow::bail!(
                "Schema '{default_schema_name}' has the same short name ('{short}') as a \
                 built-in workflow's /spcx:{short}:* commands — rename it to avoid overwriting \
                 the built-in's slash commands (e.g. '{default_schema_name}-custom')."
            );
        }
        let (schema, _) = crate::core::schema::resolve_schema(default_schema_name, project_root)?;
        push_spcx_commands(&mut commands, &short, &schema)?;
    }

    Ok(commands)
}

/// Push the `new`/`apply`/`finalise` `(name, description, body)` triples for
/// one schema, namespaced under `short`, onto `commands`. Shared by
/// `all_schema_spcx_commands`'s built-in loop and its custom-schema fallback
/// above so both go through identical body generation.
fn push_spcx_commands(
    commands: &mut Vec<(String, &'static str, String)>,
    short: &str,
    schema: &crate::core::schema::WorkflowSchema,
) -> Result<()> {
    let bodies = super::spcx::generate_bodies(schema)?;
    for (phase, description) in SPCX_PHASES {
        let body = match *phase {
            "new" => bodies.new.clone(),
            "apply" => bodies.apply.clone(),
            "finalise" => bodies.finalise.clone(),
            _ => unreachable!("SPCX_PHASES only names the 3 above"),
        };
        commands.push((format!("spcx-{short}-{phase}"), *description, body));
    }
    Ok(())
}

/// Shared per-agent writer: translates the canonical `$ARGUMENTS`
/// placeholder, appends the compliance footer, and renders/writes each
/// `(name, description, body)` triple in the target agent's format. Used by
/// `register_commands` (static bodies) and `register_all` (with
/// `all_schema_spcx_commands`'s schema-generated bodies) so agent-specific
/// handling (Copilot's dual files, Claude's spcx/ namespacing,
/// directory-based skills, ...) lives in exactly one place.
fn write_commands_for_agent(
    project_root: &Path,
    agent: &AgentConfig,
    commands: &[(String, &str, String)],
) -> Result<()> {
    let cmd_dir = project_root
        .join(agent.command_dir)
        .join(agent.commands_subdir);
    std::fs::create_dir_all(&cmd_dir)?;

    for (cmd_name, description, body) in commands {
        let mut body = formats::translate_placeholder(body, agent.arg_placeholder);
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

        let content = if agent.id == "opencode" {
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

    if agent.id == "claude"
        && let Some(suffix) = cmd_name.strip_prefix("spcx-")
    {
        // Claude Code: a subdirectory under commands/ is a genuine namespaced
        // slash command (/spcx:explore, not /solidspec-spcx-explore) — the
        // one place this repo uses that convention today, matching
        // OpenSpec's own /opsx:propose-style naming for the same 4 meta
        // commands. Written instead of (not in addition to) the flat
        // solidspec-spcx-* file the generic branch below would otherwise
        // produce.
        let spcx_dir = cmd_dir.join("spcx");
        if let Some((short, phase)) = suffix.rsplit_once('-') {
            // Per-schema meta command (`all_schema_spcx_commands`'s
            // `spcx-<short>-<phase>`): one more directory level is one more
            // colon segment, giving the full `<namespace>:<domain>:<action>`
            // pattern — /spcx:sdd:new, not /spcx:spec-driven-new.
            let short_dir = spcx_dir.join(short);
            std::fs::create_dir_all(&short_dir)?;
            std::fs::write(short_dir.join(format!("{phase}.md")), content)?;
        } else {
            // Schema-independent meta command (currently just `explore`,
            // which has no schema-specific content and so no short-name
            // segment) — stays a flat /spcx:explore.
            std::fs::create_dir_all(&spcx_dir)?;
            std::fs::write(spcx_dir.join(format!("{suffix}.md")), content)?;
        }
    } else if agent.id == "kimi" {
        // Kimi: directory-based skills with dot-separator
        let skill_name = formats::kimi_command_name(cmd_name);
        let skill_dir = cmd_dir.join(&skill_name);
        std::fs::create_dir_all(&skill_dir)?;
        std::fs::write(skill_dir.join("SKILL.md"), content)?;
    } else if agent.id == "opencode" {
        // OpenCode: directory-based skills with hyphen-separator (SKILL.md)
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
///
/// Tested (see `mod tests` below) but only called internally by
/// `unregister_commands` — no CLI command unregisters an agent yet
/// (candidate for a future `solidspec agent remove <id>`).
#[allow(dead_code)]
pub fn unregister_apex_skill(agent_id: &str, project_root: &Path) -> Result<()> {
    if let Some(dir) = apex_skill_dir(agent_id, project_root)
        && dir.exists()
    {
        std::fs::remove_dir_all(&dir)?;
    }
    Ok(())
}

/// Unregister all SolidSpec commands for a specific agent.
///
/// `default_schema_name` should mirror whatever identifier was passed to
/// the `register_all` call that wrote these files — pass `Some` when the
/// project might be running a fully custom-named schema (not one of the 7
/// built-ins) so its `/spcx:<name>:*` files get cleaned up too, `None` to
/// only ever touch the 7 built-ins' files.
///
/// Tested (see `mod tests` below) but not yet called from any CLI command —
/// candidate for a future `solidspec agent remove <id>`.
#[allow(dead_code)]
pub fn unregister_commands(
    project_root: &Path,
    agent: &AgentConfig,
    default_schema_name: Option<&str>,
) -> Result<()> {
    let cmd_dir = project_root
        .join(agent.command_dir)
        .join(agent.commands_subdir);

    if !cmd_dir.exists() {
        return Ok(());
    }

    for (cmd_name, _) in COMMANDS {
        if agent.id == "claude"
            && let Some(suffix) = cmd_name.strip_prefix("spcx-")
        {
            let path = cmd_dir.join("spcx").join(format!("{suffix}.md"));
            if path.exists() {
                std::fs::remove_file(&path)?;
            }
        } else if agent.id == "kimi" {
            let skill_name = formats::kimi_command_name(cmd_name);
            let skill_dir = cmd_dir.join(&skill_name);
            if skill_dir.exists() {
                std::fs::remove_dir_all(&skill_dir)?;
            }
        } else if agent.id == "opencode" {
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

    unregister_all_schema_spcx_commands(project_root, agent, default_schema_name)?;
    unregister_apex_skill(agent.id, project_root)?;

    Ok(())
}

/// Remove the per-schema-namespaced `/spcx:<short>:*` command files written
/// via `all_schema_spcx_commands` in `register_all`. Mirrors
/// `unregister_commands`'s per-agent path logic but iterates the dynamic
/// per-schema name set instead of the static `COMMANDS` table.
///
/// Covers the same schema set `all_schema_spcx_commands` would have written
/// for `default_schema_name` — the 7 built-ins, plus `default_schema_name`
/// itself when it names something outside that set — so a custom-named
/// schema's files don't linger as orphans after "removal".
/// `default_schema_name: None` (no caller currently has one on hand) only
/// ever touches the 7 built-ins' files, same as before this parameter
/// existed.
fn unregister_all_schema_spcx_commands(
    project_root: &Path,
    agent: &AgentConfig,
    default_schema_name: Option<&str>,
) -> Result<()> {
    let cmd_dir = project_root
        .join(agent.command_dir)
        .join(agent.commands_subdir);
    if !cmd_dir.exists() {
        return Ok(());
    }

    let mut schema_names: Vec<String> = crate::core::schema::builtin::names()
        .iter()
        .map(|s| s.to_string())
        .collect();
    if let Some(name) = default_schema_name
        && !schema_names.iter().any(|s| s == name)
    {
        schema_names.push(name.to_string());
    }

    for schema_name in &schema_names {
        let short = super::spcx::schema_short_name(schema_name);
        for (phase, _) in SPCX_PHASES {
            let cmd_name = format!("spcx-{short}-{phase}");
            if agent.id == "claude" {
                let path = cmd_dir
                    .join("spcx")
                    .join(&short)
                    .join(format!("{phase}.md"));
                if path.exists() {
                    std::fs::remove_file(&path)?;
                }
            } else if agent.id == "kimi" {
                let skill_dir = cmd_dir.join(formats::kimi_command_name(&cmd_name));
                if skill_dir.exists() {
                    std::fs::remove_dir_all(&skill_dir)?;
                }
            } else if agent.id == "opencode" {
                let skill_dir = cmd_dir.join(formats::standard_command_name(&cmd_name));
                if skill_dir.exists() {
                    std::fs::remove_dir_all(&skill_dir)?;
                }
            } else if agent.id == "copilot" {
                let path = cmd_dir.join(format!("solidspec-{cmd_name}{}", agent.extension));
                if path.exists() {
                    std::fs::remove_file(&path)?;
                }
                let prompt = project_root
                    .join(".github/prompts")
                    .join(format!("solidspec-{cmd_name}.prompt.md"));
                if prompt.exists() {
                    std::fs::remove_file(&prompt)?;
                }
            } else {
                let path = cmd_dir.join(format!(
                    "{}{}",
                    formats::standard_command_name(&cmd_name),
                    agent.extension
                ));
                if path.exists() {
                    std::fs::remove_file(&path)?;
                }
            }
        }
        // Best-effort: drop the now-empty per-schema directory
        // (.claude/commands/spcx/<short>/) `remove_dir` only succeeds when
        // empty, so this is a no-op if anything unexpected is still in it.
        if agent.id == "claude" {
            let _ = std::fs::remove_dir(cmd_dir.join("spcx").join(&short));
        }
    }

    Ok(())
}

/// Register commands for all detected agents.
///
/// `schema_name` is the project's configured schema *identifier*
/// (`solidspec.toml`'s `[pipeline].schema`, or a `--schema` flag value) —
/// the same string `schema::resolve_schema` uses to find the schema on
/// disk, resolved internally by `all_schema_spcx_commands` rather than
/// pre-resolved by the caller, so command namespacing can never drift from
/// a `WorkflowSchema.name` that happens to disagree with it (see that
/// function's doc comment).
pub fn register_all(
    project_root: &Path,
    target_agent: Option<&str>,
    schema_name: &str,
) -> Result<Vec<String>> {
    let mut registered = Vec::new();

    // Computed once and reused for every agent below: the (name,
    // description, body) triples for every /spcx:<short>:* command (the 7
    // built-in schemas plus `schema_name` itself if it names something
    // outside that set) are agent-independent (only write_commands_for_agent's
    // placement differs per agent) — resolving each schema and generating
    // its bodies again per agent would redo the same schema.yaml reads and
    // DAG walks once per detected agent for no behavioral difference.
    let schema_spcx_commands = all_schema_spcx_commands(project_root, schema_name)?;

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
        write_commands_for_agent(project_root, agent, &schema_spcx_commands)?;
        register_apex_skill(agent_id, project_root)?;
        registered.push(agent.id.to_string());
    } else {
        // Auto-detect: register for agents whose dir exists OR whose CLI is available
        let detected = detect_agents(project_root);
        for det in &detected {
            if det.dir_exists || det.cli_available {
                register_commands(project_root, det.config)?;
                write_commands_for_agent(project_root, det.config, &schema_spcx_commands)?;
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

    /// Test-only equivalent of what `register_all` does for the per-schema
    /// `/spcx:*` commands: compute them once, write them for one agent.
    /// `register_all` itself now inlines this (see its doc comment on why
    /// it isn't a standalone production function anymore).
    fn register_all_schema_spcx_commands(
        project_root: &Path,
        agent: &AgentConfig,
        default_schema_name: &str,
    ) -> Result<()> {
        let commands = all_schema_spcx_commands(project_root, default_schema_name)?;
        write_commands_for_agent(project_root, agent, &commands)
    }

    /// Write a fully custom project-local schema at
    /// `.solidspec/workflows/<dir_name>/schema.yaml`, using spec-driven's
    /// own content verbatim — including its internal `name: spec-driven`
    /// field, deliberately left mismatched from `dir_name`. That mismatch is
    /// the exact scenario `all_schema_spcx_commands`/`resolve_schema` must
    /// handle by identifier (`dir_name`, matching what
    /// `.solidspec/workflows/<name>/schema.yaml` and `[pipeline].schema` use)
    /// rather than by the parsed `WorkflowSchema.name` label, which a
    /// project author could easily leave unedited after copying an existing
    /// schema.yaml as a starting point.
    fn write_custom_schema(project_root: &Path, dir_name: &str) {
        let dir = project_root.join(".solidspec/workflows").join(dir_name);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("schema.yaml"),
            crate::core::schema::builtin::SPEC_DRIVEN,
        )
        .unwrap();
    }

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

        unregister_commands(dir.path(), copilot, None).unwrap();
        assert!(!agent_file.exists());
        assert!(!prompt_file.exists());
    }

    #[test]
    fn unregister_removes_kimi_dirs() {
        let dir = TempDir::new().unwrap();
        let kimi = find_agent("kimi").unwrap();
        register_commands(dir.path(), kimi).unwrap();
        unregister_commands(dir.path(), kimi, None).unwrap();

        let skill = dir.path().join(".kimi/skills/solidspec.specify");
        assert!(!skill.exists());
    }

    #[test]
    fn register_all_with_specific_agent() {
        let dir = TempDir::new().unwrap();
        let registered = register_all(dir.path(), Some("claude"), "spec-driven").unwrap();
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
        let result = register_all(dir.path(), Some("nonexistent"), "spec-driven");
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

        let registered = register_all(dir.path(), None, "spec-driven").unwrap();
        assert!(registered.contains(&"claude".to_string()));
        assert!(registered.contains(&"cursor".to_string()));
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
        unregister_commands(dir.path(), opencode, None).unwrap();

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
        register_all(dir.path(), Some("claude"), "spec-driven").unwrap();

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

        unregister_commands(dir.path(), claude, None).unwrap();
        assert!(
            !apex_dir.exists(),
            "APEX dir should be removed by unregister_commands"
        );
    }

    // ── spcx meta-command registration ─────────────────────────────────────
    //
    // Only `/spcx:explore` is schema-independent and flat (`register_commands`,
    // via the static `COMMANDS` table). Every schema-dependent meta command is
    // 3-segment (`/spcx:<short>:<phase>`) — see the "per-schema" block below.

    #[test]
    fn claude_gets_namespaced_explore_command_not_a_flat_file() {
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();

        assert!(dir.path().join(".claude/commands/spcx/explore.md").exists());
        // The generic flat naming must NOT also be written for Claude.
        assert!(
            !dir.path()
                .join(".claude/commands/solidspec-spcx-explore.md")
                .exists()
        );
    }

    #[test]
    fn other_agents_get_flat_explore_command() {
        let dir = TempDir::new().unwrap();
        let cursor = find_agent("cursor").unwrap();
        register_commands(dir.path(), cursor).unwrap();

        assert!(
            dir.path()
                .join(".cursor/commands/solidspec-spcx-explore.md")
                .exists()
        );
    }

    #[test]
    fn unregister_removes_claude_spcx_explore_file() {
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();
        unregister_commands(dir.path(), claude, None).unwrap();

        assert!(!dir.path().join(".claude/commands/spcx/explore.md").exists());
    }

    // ── per-schema spcx meta-command registration ───────────────────────────
    //
    // `/spcx:<short>:<phase>` — reduced-schema-name : phase, one directory
    // level per colon segment for Claude (`.claude/commands/spcx/<short>/<phase>.md`).

    #[test]
    fn register_all_schema_spcx_commands_covers_every_builtin_schema() {
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_all_schema_spcx_commands(dir.path(), claude, "spec-driven").unwrap();

        for schema_name in crate::core::schema::builtin::names() {
            let short = crate::agents::spcx::schema_short_name(schema_name);
            for phase in ["new", "apply", "finalise"] {
                let path = dir
                    .path()
                    .join(format!(".claude/commands/spcx/{short}/{phase}.md"));
                assert!(path.exists(), "missing /spcx:{short}:{phase} ({path:?})");
            }
        }
    }

    #[test]
    fn per_schema_spcx_bodies_actually_differ_by_schema() {
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_all_schema_spcx_commands(dir.path(), claude, "spec-driven").unwrap();

        let minimal_new =
            std::fs::read_to_string(dir.path().join(".claude/commands/spcx/min/new.md")).unwrap();
        let spec_driven_new =
            std::fs::read_to_string(dir.path().join(".claude/commands/spcx/sdd/new.md")).unwrap();
        assert!(minimal_new.contains("--schema minimal"));
        assert!(spec_driven_new.contains("--schema spec-driven"));
        assert_ne!(minimal_new, spec_driven_new);

        let security_first_new =
            std::fs::read_to_string(dir.path().join(".claude/commands/spcx/sec/new.md")).unwrap();
        assert!(security_first_new.contains("security-review"));

        let tdd_apply =
            std::fs::read_to_string(dir.path().join(".claude/commands/spcx/tdd/apply.md")).unwrap();
        assert!(tdd_apply.to_lowercase().contains("red"));
    }

    #[test]
    fn other_agents_get_flat_per_schema_spcx_commands() {
        let dir = TempDir::new().unwrap();
        let cursor = find_agent("cursor").unwrap();
        register_all_schema_spcx_commands(dir.path(), cursor, "spec-driven").unwrap();

        let flat = dir
            .path()
            .join(".cursor/commands/solidspec-spcx-tdd-new.md");
        assert!(flat.exists());
    }

    #[test]
    fn register_all_registers_per_schema_spcx_commands_too() {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
        register_all(dir.path(), None, "spec-driven").unwrap();

        assert!(dir.path().join(".claude/commands/spcx/min/new.md").exists());
        assert!(
            dir.path()
                .join(".claude/commands/spcx/apex/apply.md")
                .exists()
        );
    }

    #[test]
    fn unregister_removes_per_schema_spcx_commands() {
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();
        register_all_schema_spcx_commands(dir.path(), claude, "spec-driven").unwrap();
        unregister_commands(dir.path(), claude, None).unwrap();

        for schema_name in crate::core::schema::builtin::names() {
            let short = crate::agents::spcx::schema_short_name(schema_name);
            for phase in ["new", "apply", "finalise"] {
                assert!(
                    !dir.path()
                        .join(format!(".claude/commands/spcx/{short}/{phase}.md"))
                        .exists()
                );
            }
            // The now-empty per-schema directory is cleaned up too.
            assert!(
                !dir.path()
                    .join(format!(".claude/commands/spcx/{short}"))
                    .exists()
            );
        }
    }

    #[test]
    fn default_schema_named_outside_the_7_builtins_still_gets_spcx_commands() {
        // A fully custom-named project-local schema (`.solidspec/workflows/<name>/`)
        // is not one of the 7 built-ins, so it wouldn't be covered by
        // `all_schema_spcx_commands` iterating `builtin::names()` alone —
        // it must still get its own /spcx:<name>:* commands since there is no
        // flagless /spcx:new fallback left to catch it. Its schema.yaml's own
        // `name:` field (spec-driven's, unedited — see `write_custom_schema`)
        // deliberately does NOT match "my-flow": registration must key off
        // the identifier (the directory name / `--schema` value), not that
        // label, or this would silently register nothing.
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        write_custom_schema(dir.path(), "my-flow");

        register_all_schema_spcx_commands(dir.path(), claude, "my-flow").unwrap();

        for phase in ["new", "apply", "finalise"] {
            let path = dir
                .path()
                .join(format!(".claude/commands/spcx/my-flow/{phase}.md"));
            assert!(path.exists(), "missing /spcx:my-flow:{phase}");
        }
    }

    #[test]
    fn default_schema_short_name_colliding_with_a_builtin_errors_instead_of_overwriting() {
        // A custom schema literally named "tdd" would reduce to the same
        // short name as the built-in tdd-driven's "tdd" — silently writing
        // both to .claude/commands/spcx/tdd/*.md would let the second one
        // clobber the first with no warning. Must fail loudly instead.
        let dir = TempDir::new().unwrap();
        write_custom_schema(dir.path(), "tdd");

        let result = all_schema_spcx_commands(dir.path(), "tdd");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("tdd"),
            "error should name the collision: {err}"
        );
    }

    #[test]
    fn default_schema_short_name_colliding_with_a_builtin_case_insensitively_also_errors() {
        // .claude/commands/spcx/<short>/ is a real directory, and several
        // common filesystems (macOS APFS, Windows NTFS) are case-insensitive
        // by default, so "TDD" and "tdd" would collide on disk even though
        // they compare unequal as plain strings — the guard must catch this
        // case too, not just an exact-string match.
        let dir = TempDir::new().unwrap();
        write_custom_schema(dir.path(), "TDD");

        let result = all_schema_spcx_commands(dir.path(), "TDD");
        assert!(result.is_err());
    }

    #[test]
    fn unregister_removes_custom_named_default_schema_spcx_commands_too() {
        // Symmetric with `default_schema_named_outside_the_7_builtins_still_gets_spcx_commands`:
        // whatever register_all wrote for a custom-named default schema,
        // unregister_commands (given the same schema) must clean up too —
        // otherwise those files linger as orphans forever.
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();
        write_custom_schema(dir.path(), "my-flow");
        register_all_schema_spcx_commands(dir.path(), claude, "my-flow").unwrap();

        let custom_dir = dir.path().join(".claude/commands/spcx/my-flow");
        assert!(custom_dir.join("new.md").exists());

        unregister_commands(dir.path(), claude, Some("my-flow")).unwrap();

        assert!(!custom_dir.join("new.md").exists());
        assert!(
            !custom_dir.exists(),
            "empty my-flow/ dir should be cleaned up too"
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
