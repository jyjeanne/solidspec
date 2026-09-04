use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::agents::registry;
use crate::config::{InitOptions, ProjectInternalConfig, RootConfig};
use crate::core::git;
use crate::core::schema;
use crate::extensions;
use crate::templates;

/// Bundle location `init`'s OKF auto-generation writes to (task 15/16) —
/// same convention the `okf` extension (`extensions/okf/`) already uses,
/// so the two never disagree about where the project's knowledge graph
/// lives.
const OKF_BUNDLE_DIR: &str = ".solidspec/knowledge";

pub fn run(
    name: Option<String>,
    here: bool,
    no_git: bool,
    _force: bool,
    agent: Option<String>,
    schema_name: Option<String>,
) -> Result<()> {
    let project_dir = resolve_project_dir(name.as_deref(), here)?;
    let project_name = project_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "my_project".to_string());

    // Snapshot before writing anything of our own: any non-hidden entry
    // already present means this is an existing codebase, not an empty
    // directory — decides whether to auto-generate a knowledge graph below
    // (requirement 3: generate when there's something to index, leave it to
    // 'solidspec okf generate' on demand otherwise).
    let has_existing_codebase = has_existing_codebase(&project_dir);

    // Resolve the workflow schema before anything else: an invalid name
    // should fail loudly, before any file gets written, not silently fall
    // back to a default deep inside schema resolution.
    let schema_name = schema_name.unwrap_or_else(|| "minimal".to_string());
    if schema::builtin::by_name(&schema_name).is_none() {
        anyhow::bail!(
            "Unknown schema '{schema_name}'. Available: {}. Run 'solidspec schemas' for details on each.",
            schema::builtin::names().join(", ")
        );
    }
    let (workflow_schema, _) = schema::resolve_schema(&schema_name, &project_dir)?;

    println!("Initializing SolidSpec project: {project_name} (schema: {schema_name})");

    // Create directory structure
    create_directory_structure(&project_dir)?;

    // Save root config — records schema_name as the project's default, so
    // status/pipeline/go/continue pick it up without repeating --schema.
    let mut root_config = RootConfig::new(&project_name);
    root_config.pipeline.schema = schema_name;
    root_config.save(&project_dir.join("solidspec.toml"))?;

    // Save internal config
    let internal_config = ProjectInternalConfig::default();
    internal_config.save(&project_dir.join(".solidspec/config.toml"))?;

    // Copy embedded templates (preserves existing)
    templates::copy_embedded_templates(&project_dir.join(".solidspec/templates"))?;

    // Copy embedded scripts (always overwrite)
    templates::copy_embedded_scripts(&project_dir.join(".solidspec"))?;

    // Generate constitution from template (preserves existing)
    generate_constitution(&project_dir, &project_name)?;

    // Generate AGENT.md
    generate_agent_file(&project_dir, &project_name)?;

    // Detect and register AI agent commands (per-phase commands +
    // schema-aware /spcx:* meta commands — see src/agents/spcx.rs). Per-agent
    // mechanics (which directory, Markdown vs Toml, one file vs a skill
    // directory, ...) are real but not something a user needs to see at init
    // time — see docs/simplification-study-openspec.md item #8. Full detail
    // is still one 'solidspec check' away.
    let registered = registry::register_all(&project_dir, agent.as_deref(), &workflow_schema)?;
    if registered.is_empty() {
        println!("  No AI agent detected — run 'solidspec check' for setup details");
    } else {
        println!("  AI agent commands ready");
    }

    // Save init options (use first detected agent, detect script type from OS)
    let ai_assistant = registered.first().map(|s| s.as_str()).unwrap_or("claude");
    let script_type = if cfg!(windows) { "ps" } else { "sh" };
    let init_options = InitOptions {
        ai_assistant: ai_assistant.into(),
        script_type: script_type.into(),
        installed_at: chrono::Utc::now().to_rfc3339(),
    };
    init_options.save(&project_dir.join(".solidspec/init-options.json"))?;

    // Git init
    if !no_git && !git::is_git_repo(&project_dir) {
        println!("  Initializing Git repository...");
        git::init_repo(&project_dir)?;
    } else if no_git {
        println!("  Skipping Git initialization (--no-git)");
    } else {
        println!("  Git repository already exists");
    }

    // Existing codebase: generate an OKF knowledge graph natively (no
    // external okf-rs binary — see src/core/okf.rs) and register it as an
    // MCP server for agents that support one, so agents can query the
    // codebase instead of re-reading files cold during plan/analyze/review.
    // Best-effort: never fails init. A fresh/empty directory skips this —
    // 'solidspec okf generate' remains available on demand at any time.
    if has_existing_codebase {
        generate_knowledge_graph_and_mcp_config(&project_dir);
    }

    // Fire after_init hooks
    let ext_registry = extensions::manager::load_registry(&project_dir).unwrap_or_default();
    extensions::hooks::fire_hooks("after_init", &project_dir, &ext_registry);

    println!("  Project initialized at {}", project_dir.display());
    println!();
    println!(
        "Next: solidspec go \"<feature description>\"   (or 'solidspec schemas' to pick a different workflow)"
    );
    Ok(())
}

/// True when `project_dir` already has at least one non-hidden entry —
/// i.e. this is an existing codebase, not a fresh empty directory. Checked
/// before `init` writes anything of its own, so its own output never counts.
fn has_existing_codebase(project_dir: &Path) -> bool {
    std::fs::read_dir(project_dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .any(|e| !e.file_name().to_string_lossy().starts_with('.'))
        })
        .unwrap_or(false)
}

/// Best-effort: generate the OKF bundle and register it as an MCP server.
/// Never returns an error to the caller — every failure is a printed
/// warning, since none of this should ever block `solidspec init` itself.
fn generate_knowledge_graph_and_mcp_config(project_dir: &Path) {
    let bundle_dir = project_dir.join(OKF_BUNDLE_DIR);
    match crate::core::okf::generate(project_dir, &bundle_dir) {
        Ok(report) => {
            println!(
                "  Generated knowledge graph: {} concept(s) in {OKF_BUNDLE_DIR}/",
                report.total_concepts
            );
            if let Err(e) = write_okf_mcp_config(project_dir) {
                println!("  Warning: could not register the OKF MCP server: {e}");
            } else {
                println!(
                    "  Registered okf MCP server in .mcp.json (needs 'okf-mcp' on PATH — \
                     cargo install --git https://github.com/jyjeanne/okf-rs okf-mcp)"
                );
            }
        }
        Err(e) => {
            println!("  Warning: knowledge graph generation skipped: {e}");
            println!("    Run 'solidspec okf generate' manually once the issue is resolved.");
        }
    }
}

/// Merge an `okf` entry into `.mcp.json`'s `mcpServers` (Claude Code's
/// project-scoped MCP config) without disturbing any other server already
/// configured there. Claude Code only for now — it's the one MCP config
/// format this project is sure of; other MCP-capable agents are a follow-up.
fn write_okf_mcp_config(project_dir: &Path) -> Result<()> {
    let path = project_dir.join(".mcp.json");
    let mut root: serde_json::Value = if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    if !root.is_object() {
        root = serde_json::json!({});
    }
    let obj = root.as_object_mut().expect("just ensured object above");
    let servers = obj
        .entry("mcpServers")
        .or_insert_with(|| serde_json::json!({}));
    if !servers.is_object() {
        *servers = serde_json::json!({});
    }
    servers.as_object_mut().unwrap().insert(
        "okf".to_string(),
        serde_json::json!({
            "command": "okf-mcp",
            "args": [OKF_BUNDLE_DIR]
        }),
    );

    let content = serde_json::to_string_pretty(&root)?;
    std::fs::write(&path, content)?;
    Ok(())
}

fn resolve_project_dir(name: Option<&str>, here: bool) -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;

    match name {
        Some(n) if !here => {
            let dir = cwd.join(n);
            std::fs::create_dir_all(&dir)?;
            Ok(dir)
        }
        _ => Ok(cwd),
    }
}

fn create_directory_structure(project_dir: &Path) -> Result<()> {
    let dirs = [
        ".solidspec/templates/overrides",
        ".solidspec/presets",
        ".solidspec/extensions/.cache/catalogs",
        "specs",
    ];

    for dir in &dirs {
        std::fs::create_dir_all(project_dir.join(dir))?;
    }

    // Create empty registry files
    let registries = [
        ".solidspec/presets/.registry",
        ".solidspec/extensions/.registry",
    ];
    for reg in &registries {
        let path = project_dir.join(reg);
        if !path.exists() {
            std::fs::write(&path, "{}")?;
        }
    }

    Ok(())
}

fn generate_constitution(project_dir: &Path, project_name: &str) -> Result<()> {
    let path = project_dir.join(".solidspec/constitution.md");
    if path.exists() {
        println!("  Constitution already exists, preserving");
        return Ok(());
    }

    let mut vars = HashMap::new();
    vars.insert("project_name".into(), project_name.to_string());
    vars.insert(
        "date".into(),
        chrono::Local::now().format("%Y-%m-%d").to_string(),
    );

    let content = templates::render(templates::embedded::CONSTITUTION_TEMPLATE, &vars)?;
    std::fs::write(&path, content)?;
    println!("  Generated constitution.md");
    Ok(())
}

fn generate_agent_file(project_dir: &Path, project_name: &str) -> Result<()> {
    let path = project_dir.join(".solidspec/AGENT.md");

    let mut vars = HashMap::new();
    vars.insert("project_name".into(), project_name.to_string());
    vars.insert(
        "date".into(),
        chrono::Local::now().format("%Y-%m-%d").to_string(),
    );

    let content = templates::render(templates::embedded::AGENT_FILE_TEMPLATE, &vars)?;
    std::fs::write(&path, content)?;
    Ok(())
}
