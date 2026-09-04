---
type: Rust Function
title: all_schema_spcx_commands
resource: src/agents/registry.rs#L205-L249
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/schema/names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/schema_short_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/resolve_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/push_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/register_all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/default_schema_short_name_colliding_with_a_builtin_errors_instead_of_overwriting
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/default_schema_short_name_colliding_with_a_builtin_case_insensitively_also_errors
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn all_schema_spcx_commands( project_root: &Path, default_schema_name: &str, ) -> Result<Vec<(String, &'static str, String)>>`

# Calls

- [names](../../../../functions/src/core/schema/names.md)
- [schema_short_name](../../../../functions/src/agents/spcx/schema_short_name.md)
- [resolve_schema](../../../../functions/src/core/schema/resolve_schema.md)
- [push_spcx_commands](../../../../functions/src/agents/registry/push_spcx_commands.md)

# Called by

- [register_all](../../../../functions/src/agents/registry/register_all.md)
- [register_all_schema_spcx_commands](../../../../functions/src/agents/registry/register_all_schema_spcx_commands.md)
- [default_schema_short_name_colliding_with_a_builtin_errors_instead_of_overwriting](../../../../functions/src/agents/registry/default_schema_short_name_colliding_with_a_builtin_errors_instead_of_overwriting.md)
- [default_schema_short_name_colliding_with_a_builtin_case_insensitively_also_errors](../../../../functions/src/agents/registry/default_schema_short_name_colliding_with_a_builtin_case_insensitively_also_errors.md)