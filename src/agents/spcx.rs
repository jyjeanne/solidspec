//! Schema-aware generation of the 4 `/spcx:*` meta-command bodies
//! (`new`/`apply`/`finalise` — each namespaced per schema as
//! `/spcx:<short>:<phase>`, see `schema_short_name` below; `explore` is
//! generic, schema-independent, and stays flat as `/spcx:explore`).
//!
//! Each built-in and project-local schema already carries, per artifact, a
//! human-readable `instruction` field (`schemas/*/schema.yaml`) that was
//! parsed but never consulted by anything (see the `#[allow(dead_code)]` it
//! used to carry in `artifact_graph.rs`) — this module is that consumer.
//! Walking the schema's phases in `core::pipeline::phases_for_schema`'s
//! narrative/execution order (the same order `solidspec pipeline` actually
//! runs them in) and classifying each `Auto` vs `Handoff`
//! (`core::pipeline::phase_type`) splits it into three segments with no
//! per-schema special-casing needed:
//!
//! - a leading run of `Auto` phases → `/spcx:<short>:new` (scaffold + fill in)
//! - the following run of `Handoff` phases → `/spcx:<short>:apply`
//!   (implement/apex/...)
//! - everything after that (analyze/review, or nothing at all for a schema
//!   like `minimal` that ends at `implement`), plus `ship` when the schema's
//!   artifact graph has one → `/spcx:<short>:finalise`
//!
//! This is why `minimal`'s `/spcx:min:finalise` correctly says "nothing more
//! to run" instead of referencing analyze/review/ship it doesn't have, and
//! why `security-first`'s `/spcx:sec:new` correctly includes
//! `security-review` without this module knowing that schema exists.
//!
//! Note this deliberately does NOT use `ArtifactGraph::topological_order`:
//! that order is only DAG-valid, not narrative — e.g. `spec-driven`'s
//! `analyze`/`review` artifacts declare `requires: ["spec"]` only (for
//! DAG-gating/status purposes), so a topological sort is free to place them
//! before `tasks`/`implement`. `phases_for_schema` is the same
//! already-correct source `filter_phases`/`execute_phase` trust for run
//! order.

use anyhow::Result;

use crate::core::artifact_graph::ArtifactNode;
use crate::core::pipeline;
use crate::core::pipeline::PhaseType;
use crate::core::schema::WorkflowSchema;

pub struct SpcxBodies {
    pub new: String,
    pub apply: String,
    pub finalise: String,
}

/// Map an artifact-graph id to the CLI subcommand that produces it. Every
/// artifact id matches its CLI command name except `spec`, whose command is
/// `specify` (mirrors `core::pipeline::schema_artifact_id`'s inverse).
fn cli_command_for(artifact_id: &str) -> &str {
    if artifact_id == "spec" {
        "specify"
    } else {
        artifact_id
    }
}

/// Inverse of `cli_command_for`: map a `phases_for_schema` phase name back
/// to its schema artifact-graph id (mirrors
/// `core::pipeline::schema_artifact_id`, which is private to that module).
fn artifact_id_for_phase(phase: &str) -> &str {
    if phase == "specify" { "spec" } else { phase }
}

/// Reduced (short) schema name used as the middle segment of the
/// `/spcx:<short>:<phase>` slash-command pattern (`registry.rs`'s
/// `all_schema_spcx_commands`/`write_command_file`): a schema variant is a
/// namespace segment of its own, not text glued onto the action name, so
/// `/spcx:tdd:new` reads as `<namespace>:<domain>:<action>` instead of
/// `/spcx:tdd-driven-new`'s single hyphenated blob. Every phase name
/// (`new`/`apply`/`finalise`) is hyphen-free, so `write_command_file` can
/// always round-trip the internal `spcx-<short>-<phase>` name back into its
/// two parts with `rsplit_once('-')` on the *last* hyphen — safe even when
/// `<short>` itself contains hyphens (a passed-through custom schema name,
/// below).
///
/// A project-local schema whose name isn't one of the 7 built-ins (custom
/// `.solidspec/workflows/<name>/schema.yaml`) falls through unchanged: it's
/// already the name a project author picked.
pub fn schema_short_name(schema_name: &str) -> String {
    match schema_name {
        "minimal" => "min",
        "spec-driven" => "sdd",
        "security-first" => "sec",
        "tdd-driven" => "tdd",
        "intent-driven" => "intent",
        "apex-driven" => "apex",
        "intent-apex" => "iapex",
        other => other,
    }
    .to_string()
}

/// Generate the `new`/`apply`/`finalise` bodies for `schema`. Each still
/// contains the canonical `$ARGUMENTS` placeholder, translated per-agent by
/// `formats::translate_placeholder` the same way the static command bodies
/// are — this module only decides *what* the body says, not which agent it
/// ends up written for.
pub fn generate_bodies(schema: &WorkflowSchema) -> Result<SpcxBodies> {
    let schema_name = schema.name.clone();
    let graph = schema
        .clone()
        .into_graph()
        .map_err(|e| anyhow::anyhow!("Invalid schema '{schema_name}': {e}"))?;

    // Narrative order, not topological order — see module doc.
    let mut new_phases: Vec<&ArtifactNode> = Vec::new();
    let mut apply_phases: Vec<&ArtifactNode> = Vec::new();
    let mut finalise_phases: Vec<&ArtifactNode> = Vec::new();
    let mut seen_handoff = false;
    let mut seen_post_handoff = false;
    for phase in pipeline::phases_for_schema(&schema_name) {
        let artifact_id = artifact_id_for_phase(phase);
        let Some(node) = graph.get(artifact_id) else {
            // A phase this schema doesn't actually declare an artifact for
            // (shouldn't happen for any built-in schema, but a project-local
            // override could omit one) — skip rather than fail generation.
            continue;
        };
        let is_handoff = pipeline::phase_type(phase) == PhaseType::Handoff;
        if is_handoff {
            seen_handoff = true;
            if seen_post_handoff {
                // A second, later handoff run (not seen in any built-in
                // schema) — still handled: goes wherever we already are
                // rather than panicking or silently dropping it.
                finalise_phases.push(node);
            } else {
                apply_phases.push(node);
            }
        } else if !seen_handoff {
            new_phases.push(node);
        } else {
            seen_post_handoff = true;
            finalise_phases.push(node);
        }
    }

    // `ship` is deliberately excluded from every `phases_for_schema` list
    // (see `next_step_hint`'s doc comment) but IS a real artifact-graph node
    // with a useful `.instruction` for schemas that declare one — append it
    // to finalise when present instead of silently dropping it.
    if let Some(ship) = graph.get("ship") {
        finalise_phases.push(ship);
    }

    let short = schema_short_name(&schema_name);
    Ok(SpcxBodies {
        new: render_new(&schema_name, &short, &new_phases),
        apply: render_apply(&short, &apply_phases, finalise_phases.is_empty()),
        finalise: render_finalise(&schema_name, &finalise_phases),
    })
}

fn render_new(schema_name: &str, short: &str, phases: &[&ArtifactNode]) -> String {
    let chain: Vec<&str> = phases.iter().map(|n| n.id.as_str()).collect();
    let last_cli_phase = phases
        .last()
        .map(|n| cli_command_for(&n.id))
        .unwrap_or("tasks");

    let mut out = format!(
        "Read the project context from .solidspec/AGENT.md.\n\n\
         Feature description: $ARGUMENTS\n\n\
         This is the full \"start a feature\" flow for the {schema_name} schema — \
         {} — run in one pass instead of running each phase's command separately.\n\n\
         1. Run: `solidspec pipeline --new \"$ARGUMENTS\" --to {last_cli_phase} --schema {schema_name} --no-agent --auto`\n   \
         This scaffolds every artifact through {last_cli_phase} with placeholder content under the \
         new specs/NNN-slug/ directory it creates and prints. Note that directory name — you need it \
         for every step below.\n",
        chain.join(" → "),
    );

    for (i, node) in phases.iter().enumerate() {
        if node.instruction.is_empty() {
            continue;
        }
        out.push_str(&format!("{}. {}\n", i + 2, node.instruction));
    }

    out.push_str(&format!(
        "\nNext: /spcx:{short}:apply to implement, or /spcx:explore to discuss the plan first.\n"
    ));
    out
}

fn render_apply(short: &str, phases: &[&ArtifactNode], nothing_after: bool) -> String {
    let mut out = "Read the project context from .solidspec/AGENT.md.\n\n\
         Feature: $ARGUMENTS (auto-detected from the current git branch or latest spec \
         if left empty).\n\n\
         1. Run: `solidspec status $ARGUMENTS`\n   \
         Confirms tasks.md exists and shows which artifacts are ready.\n"
        .to_string();

    for (i, node) in phases.iter().enumerate() {
        if node.instruction.is_empty() {
            continue;
        }
        out.push_str(&format!("{}. {}\n", i + 2, node.instruction));
    }

    if nothing_after {
        out.push_str(
            "\nThis schema ends here — run `solidspec status` to confirm everything is done.\n",
        );
    } else {
        out.push_str(&format!(
            "\nNext: /spcx:{short}:finalise to validate, review, and get a SHIP/HOLD decision.\n"
        ));
    }
    out
}

fn render_finalise(schema_name: &str, phases: &[&ArtifactNode]) -> String {
    if phases.is_empty() {
        return format!(
            "Read the project context from .solidspec/AGENT.md.\n\n\
             Feature: $ARGUMENTS (auto-detected if left empty).\n\n\
             The {schema_name} schema ends at the implement step — there's nothing further to \
             run automatically. Run `solidspec status $ARGUMENTS` to confirm everything is done.\n"
        );
    }

    let mut out = "Read the project context from .solidspec/AGENT.md.\n\n\
         Feature: $ARGUMENTS (auto-detected if left empty).\n\n"
        .to_string();

    for (i, node) in phases.iter().enumerate() {
        let cli_name = cli_command_for(&node.id);
        out.push_str(&format!(
            "{}. Run: `solidspec {cli_name} $ARGUMENTS`\n   {}\n",
            i + 1,
            node.instruction
        ));
    }

    out.push_str(
        "\nReport the final result (including any SHIP/HOLD decision) plainly at the end.\n",
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::schema::builtin;

    fn schema(name: &str) -> WorkflowSchema {
        WorkflowSchema::parse(builtin::by_name(name).unwrap()).unwrap()
    }

    #[test]
    fn minimal_new_covers_specify_plan_tasks_and_stops_before_implement() {
        let bodies = generate_bodies(&schema("minimal")).unwrap();
        assert!(bodies.new.contains("--to tasks --schema minimal"));
        assert!(bodies.new.contains("$ARGUMENTS"));
    }

    #[test]
    fn minimal_finalise_has_nothing_to_run() {
        let bodies = generate_bodies(&schema("minimal")).unwrap();
        assert!(bodies.finalise.contains("nothing further to run"));
        assert!(!bodies.finalise.contains("solidspec ship"));
    }

    #[test]
    fn minimal_apply_says_schema_ends_here() {
        let bodies = generate_bodies(&schema("minimal")).unwrap();
        assert!(bodies.apply.contains("This schema ends here"));
    }

    #[test]
    fn spec_driven_new_stops_before_implement_includes_tests() {
        let bodies = generate_bodies(&schema("spec-driven")).unwrap();
        assert!(bodies.new.contains("--to tests --schema spec-driven"));
    }

    #[test]
    fn spec_driven_finalise_includes_analyze_review_and_ship() {
        let bodies = generate_bodies(&schema("spec-driven")).unwrap();
        assert!(bodies.finalise.contains("solidspec analyze"));
        assert!(bodies.finalise.contains("solidspec review"));
        assert!(bodies.finalise.contains("solidspec ship"));
    }

    #[test]
    fn security_first_new_includes_security_review_step() {
        let bodies = generate_bodies(&schema("security-first")).unwrap();
        assert!(bodies.new.contains("security-review"));
        assert!(bodies.new.contains("--to tasks --schema security-first"));
    }

    #[test]
    fn security_first_finalise_has_nothing_to_run() {
        // security-first has no analyze/review/ship artifacts.
        let bodies = generate_bodies(&schema("security-first")).unwrap();
        assert!(bodies.finalise.contains("nothing further to run"));
    }

    #[test]
    fn tdd_driven_apply_covers_all_three_handoff_phases() {
        let bodies = generate_bodies(&schema("tdd-driven")).unwrap();
        assert!(bodies.apply.contains("tdd-tests") || bodies.apply.to_lowercase().contains("red"));
        assert!(bodies.apply.contains("Next: /spcx:tdd:finalise"));
    }

    #[test]
    fn intent_driven_new_uses_intent_cli_command_first() {
        let bodies = generate_bodies(&schema("intent-driven")).unwrap();
        // "intent" schema artifact id maps 1:1 to the `intent` CLI command.
        assert!(bodies.new.contains("intent"));
    }

    #[test]
    fn schema_short_name_maps_every_builtin_and_is_hyphen_free() {
        let expected = [
            ("minimal", "min"),
            ("spec-driven", "sdd"),
            ("security-first", "sec"),
            ("tdd-driven", "tdd"),
            ("intent-driven", "intent"),
            ("apex-driven", "apex"),
            ("intent-apex", "iapex"),
        ];
        for (schema_name, short) in expected {
            assert_eq!(schema_short_name(schema_name), short);
            assert!(
                !short.contains('-'),
                "{short} must be hyphen-free so spcx-{{short}}-{{phase}} round-trips"
            );
        }
    }

    #[test]
    fn schema_short_name_passes_through_unknown_names() {
        assert_eq!(schema_short_name("my-custom-flow"), "my-custom-flow");
    }

    #[test]
    fn every_builtin_schema_generates_without_error() {
        for name in builtin::names() {
            let s = schema(name);
            let result = generate_bodies(&s);
            assert!(result.is_ok(), "{name} failed: {:?}", result.err());
        }
    }
}
