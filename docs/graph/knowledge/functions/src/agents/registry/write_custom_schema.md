---
type: Rust Function
title: write_custom_schema
resource: src/agents/registry.rs#L713-L721
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/registry/default_schema_named_outside_the_7_builtins_still_gets_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/default_schema_short_name_colliding_with_a_builtin_errors_instead_of_overwriting
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/default_schema_short_name_colliding_with_a_builtin_case_insensitively_also_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_custom_named_default_schema_spcx_commands_too
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_custom_schema(project_root: &Path, dir_name: &str)`

# Called by

- [default_schema_named_outside_the_7_builtins_still_gets_spcx_commands](../../../../functions/src/agents/registry/default_schema_named_outside_the_7_builtins_still_gets_spcx_commands.md)
- [default_schema_short_name_colliding_with_a_builtin_errors_instead_of_overwriting](../../../../functions/src/agents/registry/default_schema_short_name_colliding_with_a_builtin_errors_instead_of_overwriting.md)
- [default_schema_short_name_colliding_with_a_builtin_case_insensitively_also_errors](../../../../functions/src/agents/registry/default_schema_short_name_colliding_with_a_builtin_case_insensitively_also_errors.md)
- [unregister_removes_custom_named_default_schema_spcx_commands_too](../../../../functions/src/agents/registry/unregister_removes_custom_named_default_schema_spcx_commands_too.md)