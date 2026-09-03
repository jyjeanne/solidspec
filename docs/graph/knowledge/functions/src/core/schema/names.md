---
type: Rust Function
title: names
resource: src/core/schema.rs#L56-L66
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
  - target: functions/src/agents/spcx/every_builtin_schema_generates_without_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/list_available_schemas
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn names() -> Vec<&'static str>`

# Called by

- [all_schema_spcx_commands](../../../../functions/src/agents/registry/all_schema_spcx_commands.md)
- [unregister_all_schema_spcx_commands](../../../../functions/src/agents/registry/unregister_all_schema_spcx_commands.md)
- [register_all_schema_spcx_commands_covers_every_builtin_schema](../../../../functions/src/agents/registry/register_all_schema_spcx_commands_covers_every_builtin_schema.md)
- [unregister_removes_per_schema_spcx_commands](../../../../functions/src/agents/registry/unregister_removes_per_schema_spcx_commands.md)
- [every_builtin_schema_generates_without_error](../../../../functions/src/agents/spcx/every_builtin_schema_generates_without_error.md)
- [list_available_schemas](../../../../functions/src/core/schema/list_available_schemas.md)