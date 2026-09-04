---
type: Rust Function
title: schema_short_name
resource: src/agents/spcx.rs#L82-L94
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/registry/all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_all_schema_spcx_commands_covers_every_builtin_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_per_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn schema_short_name(schema_name: &str) -> String`

# Called by

- [all_schema_spcx_commands](../../../../functions/src/agents/registry/all_schema_spcx_commands.md)
- [unregister_all_schema_spcx_commands](../../../../functions/src/agents/registry/unregister_all_schema_spcx_commands.md)
- [register_all_schema_spcx_commands_covers_every_builtin_schema](../../../../functions/src/agents/registry/register_all_schema_spcx_commands_covers_every_builtin_schema.md)
- [unregister_removes_per_schema_spcx_commands](../../../../functions/src/agents/registry/unregister_removes_per_schema_spcx_commands.md)
- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)