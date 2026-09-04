---
type: Rust Function
title: default_schema_short_name_colliding_with_a_builtin_errors_instead_of_overwriting
resource: src/agents/registry.rs#L1260-L1275
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/registry/write_custom_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn default_schema_short_name_colliding_with_a_builtin_errors_instead_of_overwriting()`

# Calls

- [write_custom_schema](../../../../functions/src/agents/registry/write_custom_schema.md)
- [all_schema_spcx_commands](../../../../functions/src/agents/registry/all_schema_spcx_commands.md)