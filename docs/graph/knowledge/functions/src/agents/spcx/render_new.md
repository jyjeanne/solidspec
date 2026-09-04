---
type: Rust Function
title: render_new
resource: src/agents/spcx.rs#L124-L152
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/cli_command_for
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_new(schema_name: &str, phases: &[&ArtifactNode]) -> String`

# Calls

- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)
- [cli_command_for](../../../../functions/src/agents/spcx/cli_command_for.md)

# Called by

- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)