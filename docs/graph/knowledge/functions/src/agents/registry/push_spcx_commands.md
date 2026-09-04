---
type: Rust Function
title: push_spcx_commands
resource: src/agents/registry.rs#L221-L237
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn push_spcx_commands( commands: &mut Vec<(String, &'static str, String)>, short: &str, schema: &crate::core::schema::WorkflowSchema, ) -> Result<()>`

# Calls

- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)

# Called by

- [all_schema_spcx_commands](../../../../functions/src/agents/registry/all_schema_spcx_commands.md)