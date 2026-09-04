---
type: Rust Function
title: default_schema_short_name_colliding_with_a_builtin_case_insensitively_also_errors
resource: src/agents/registry.rs#L1278-L1289
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

`fn default_schema_short_name_colliding_with_a_builtin_case_insensitively_also_errors()`

# Calls

- [write_custom_schema](../../../../functions/src/agents/registry/write_custom_schema.md)
- [all_schema_spcx_commands](../../../../functions/src/agents/registry/all_schema_spcx_commands.md)