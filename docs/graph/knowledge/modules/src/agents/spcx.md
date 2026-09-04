---
type: Rust Module
title: spcx
resource: src/agents/spcx.rs#L1-L287
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-artifact-graph-artifactnode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-pipeline
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-pipeline-phasetype
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-schema-workflowschema
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-schema-builtin
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [SpcxBodies](../../../classes/src/agents/spcx/SpcxBodies.md)
- [cli_command_for](../../../functions/src/agents/spcx/cli_command_for.md)
- [artifact_id_for_phase](../../../functions/src/agents/spcx/artifact_id_for_phase.md)
- [generate_bodies](../../../functions/src/agents/spcx/generate_bodies.md)
- [render_new](../../../functions/src/agents/spcx/render_new.md)
- [render_apply](../../../functions/src/agents/spcx/render_apply.md)
- [render_finalise](../../../functions/src/agents/spcx/render_finalise.md)
- [schema](../../../functions/src/agents/spcx/schema.md)
- [minimal_new_covers_specify_plan_tasks_and_stops_before_implement](../../../functions/src/agents/spcx/minimal_new_covers_specify_plan_tasks_and_stops_before_implement.md)
- [minimal_finalise_has_nothing_to_run](../../../functions/src/agents/spcx/minimal_finalise_has_nothing_to_run.md)
- [minimal_apply_says_schema_ends_here](../../../functions/src/agents/spcx/minimal_apply_says_schema_ends_here.md)
- [spec_driven_new_stops_before_implement_includes_tests](../../../functions/src/agents/spcx/spec_driven_new_stops_before_implement_includes_tests.md)
- [spec_driven_finalise_includes_analyze_review_and_ship](../../../functions/src/agents/spcx/spec_driven_finalise_includes_analyze_review_and_ship.md)
- [security_first_new_includes_security_review_step](../../../functions/src/agents/spcx/security_first_new_includes_security_review_step.md)
- [security_first_finalise_has_nothing_to_run](../../../functions/src/agents/spcx/security_first_finalise_has_nothing_to_run.md)
- [tdd_driven_apply_covers_all_three_handoff_phases](../../../functions/src/agents/spcx/tdd_driven_apply_covers_all_three_handoff_phases.md)
- [intent_driven_new_uses_intent_cli_command_first](../../../functions/src/agents/spcx/intent_driven_new_uses_intent_cli_command_first.md)
- [every_builtin_schema_generates_without_error](../../../functions/src/agents/spcx/every_builtin_schema_generates_without_error.md)

# Imports

- `anyhow::Result`
- `crate::core::artifact_graph::ArtifactNode`
- `crate::core::pipeline`
- `crate::core::pipeline::PhaseType`
- `crate::core::schema::WorkflowSchema`
- `super::*`
- `crate::core::schema::builtin`

# Member of

- [solidspec](../../../packages/solidspec.md)