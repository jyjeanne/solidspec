---
type: Rust Function
title: render_finalise
resource: src/agents/spcx.rs#L232-L259
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/spcx/cli_command_for
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_finalise(schema_name: &str, phases: &[&ArtifactNode]) -> String`

# Calls

- [cli_command_for](../../../../functions/src/agents/spcx/cli_command_for.md)

# Called by

- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)