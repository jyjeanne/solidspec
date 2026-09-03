//! Integration tests for `solidspec init`'s 3-point overhaul:
//! 1. `--schema` selects the workflow and drives schema-aware `/spcx:*`
//!    command generation (see `src/agents/spcx.rs`).
//! 2. Omitting `--schema` defaults to `minimal` and persists it as the
//!    project's default (`solidspec.toml`'s `[pipeline].schema`).
//! 3. An existing codebase gets a native OKF knowledge-graph bundle and an
//!    `okf` MCP server registered in `.mcp.json`; an empty directory gets
//!    neither.

use predicates::prelude::*;
use tempfile::TempDir;

mod common;
use common::solidspec;

/// Pre-create `.claude/` so the claude agent is always detected regardless
/// of whether the `claude` CLI binary is on `PATH` — same reasoning as
/// `common::init_project`, but this file needs to vary the `init` args
/// themselves (schema, existing files) so it builds the command directly.
fn with_claude_dir(dir: &std::path::Path) {
    std::fs::create_dir_all(dir.join(".claude")).unwrap();
}

#[test]
fn init_without_schema_defaults_to_minimal_and_persists_it() {
    let dir = TempDir::new().unwrap();
    with_claude_dir(dir.path());

    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("schema: minimal"));

    let toml = std::fs::read_to_string(dir.path().join("solidspec.toml")).unwrap();
    assert!(
        toml.contains("schema = \"minimal\""),
        "solidspec.toml should record minimal as the project default: {toml}"
    );
}

#[test]
fn init_with_schema_flag_persists_the_chosen_schema() {
    let dir = TempDir::new().unwrap();
    with_claude_dir(dir.path());

    solidspec()
        .args(["init", "--here", "--no-git", "--schema", "spec-driven"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("schema: spec-driven"));

    let toml = std::fs::read_to_string(dir.path().join("solidspec.toml")).unwrap();
    assert!(toml.contains("schema = \"spec-driven\""));
}

#[test]
fn init_with_unknown_schema_fails_before_writing_any_files() {
    let dir = TempDir::new().unwrap();
    with_claude_dir(dir.path());

    solidspec()
        .args(["init", "--here", "--no-git", "--schema", "bogus"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown schema 'bogus'"));

    assert!(
        !dir.path().join("solidspec.toml").exists(),
        "a rejected --schema must fail before any file is written"
    );
}

#[test]
fn init_registers_spcx_commands_matching_the_chosen_schema() {
    let dir = TempDir::new().unwrap();
    with_claude_dir(dir.path());

    solidspec()
        .args(["init", "--here", "--no-git", "--schema", "spec-driven"])
        .current_dir(dir.path())
        .assert()
        .success();

    // spec-driven's /spcx:new stops before implement (tests is its last Auto
    // phase) and /spcx:finalise covers analyze/review/ship — see
    // src/agents/spcx.rs's module doc for why this needs no schema-specific
    // test cases beyond checking the right schema was actually used.
    let new_body =
        std::fs::read_to_string(dir.path().join(".claude/commands/spcx/new.md")).unwrap();
    assert!(new_body.contains("--schema spec-driven"));
    assert!(new_body.contains("--to tests"));

    let finalise_body =
        std::fs::read_to_string(dir.path().join(".claude/commands/spcx/finalise.md")).unwrap();
    assert!(finalise_body.contains("solidspec analyze"));
    assert!(finalise_body.contains("solidspec review"));
    assert!(finalise_body.contains("solidspec ship"));
}

#[test]
fn init_registers_namespaced_spcx_commands_for_every_builtin_schema() {
    let dir = TempDir::new().unwrap();
    with_claude_dir(dir.path());

    // Regardless of --schema, init also registers /spcx:<schema>-{new,apply,
    // finalise} for every other built-in workflow, so an agent can run any
    // of them explicitly without switching the project's stored default —
    // see src/agents/registry.rs's register_all_schema_spcx_commands.
    solidspec()
        .args(["init", "--here", "--no-git", "--schema", "minimal"])
        .current_dir(dir.path())
        .assert()
        .success();

    for schema in [
        "minimal",
        "spec-driven",
        "security-first",
        "tdd-driven",
        "intent-driven",
        "apex-driven",
        "intent-apex",
    ] {
        for phase in ["new", "apply", "finalise"] {
            let path = dir
                .path()
                .join(format!(".claude/commands/spcx/{schema}-{phase}.md"));
            assert!(path.exists(), "missing /spcx:{schema}-{phase}");
        }
    }

    // The flagless /spcx:new (the project's minimal default) still exists
    // alongside the namespaced /spcx:minimal-new — both should say the same
    // thing since minimal is this project's default schema.
    let flagless =
        std::fs::read_to_string(dir.path().join(".claude/commands/spcx/new.md")).unwrap();
    let namespaced =
        std::fs::read_to_string(dir.path().join(".claude/commands/spcx/minimal-new.md")).unwrap();
    assert_eq!(flagless, namespaced);

    // A different schema's content actually differs.
    let tdd = std::fs::read_to_string(dir.path().join(".claude/commands/spcx/tdd-driven-apply.md"))
        .unwrap();
    assert!(tdd.to_lowercase().contains("red"));
}

#[test]
fn init_on_empty_directory_skips_knowledge_graph_generation() {
    let dir = TempDir::new().unwrap();
    with_claude_dir(dir.path());

    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated knowledge graph").not());

    assert!(!dir.path().join(".solidspec/knowledge").exists());
    assert!(!dir.path().join(".mcp.json").exists());
}

#[test]
fn init_on_existing_codebase_generates_knowledge_graph_and_mcp_config() {
    let dir = TempDir::new().unwrap();
    with_claude_dir(dir.path());
    // A non-hidden file already present is what makes this "an existing
    // codebase" per has_existing_codebase (src/cli/init.rs) — the `.claude/`
    // dir created above is hidden and deliberately doesn't count on its own.
    std::fs::write(
        dir.path().join("main.rs"),
        "fn greet(name: &str) -> String { format!(\"hi {name}\") }\nfn main() { println!(\"{}\", greet(\"x\")); }\n",
    )
    .unwrap();

    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated knowledge graph"))
        .stdout(predicate::str::contains("Registered okf MCP server"));

    assert!(dir.path().join(".solidspec/knowledge/index.md").exists());

    let mcp: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.path().join(".mcp.json")).unwrap())
            .unwrap();
    assert_eq!(
        mcp["mcpServers"]["okf"]["command"],
        serde_json::Value::String("okf-mcp".to_string())
    );
    assert_eq!(mcp["mcpServers"]["okf"]["args"][0], ".solidspec/knowledge");
}

#[test]
fn init_on_existing_codebase_preserves_other_mcp_servers_already_configured() {
    let dir = TempDir::new().unwrap();
    with_claude_dir(dir.path());
    std::fs::write(dir.path().join("main.rs"), "fn main() {}\n").unwrap();
    std::fs::write(
        dir.path().join(".mcp.json"),
        r#"{"mcpServers": {"other": {"command": "other-server", "args": []}}}"#,
    )
    .unwrap();

    solidspec()
        .args(["init", "--here", "--no-git"])
        .current_dir(dir.path())
        .assert()
        .success();

    let mcp: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.path().join(".mcp.json")).unwrap())
            .unwrap();
    assert_eq!(
        mcp["mcpServers"]["other"]["command"],
        serde_json::Value::String("other-server".to_string())
    );
    assert_eq!(
        mcp["mcpServers"]["okf"]["command"],
        serde_json::Value::String("okf-mcp".to_string())
    );
}

#[test]
fn go_and_continue_use_the_projects_stored_default_schema() {
    let dir = TempDir::new().unwrap();
    with_claude_dir(dir.path());

    solidspec()
        .args(["init", "--here", "--no-git", "--schema", "minimal"])
        .current_dir(dir.path())
        .assert()
        .success();

    // minimal has no clarify/tests/analyze/review artifacts — `go` running
    // on any other (implicit) schema would scaffold files this assertion
    // doesn't expect.
    solidspec()
        .args(["go", "Minimal schema smoke test", "--no-agent"])
        .current_dir(dir.path())
        .assert()
        .success();

    let feature_dir = common::first_feature_dir(dir.path());
    assert!(feature_dir.join("spec.md").exists());
    assert!(feature_dir.join("plan.md").exists());
    assert!(feature_dir.join("tasks.md").exists());
    assert!(!feature_dir.join("tests").exists());
}
