---
type: Rust Function
title: list_available_schemas
resource: src/core/schema.rs#L153-L201
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/schema/names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/by_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/schemas/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/list_available_schemas_includes_builtins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/list_available_schemas_includes_apex_schemas
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn list_available_schemas(project_root: &Path) -> Vec<SchemaInfo>`

# Calls

- [names](../../../../functions/src/core/schema/names.md)
- [by_name](../../../../functions/src/core/schema/by_name.md)

# Called by

- [run](../../../../functions/src/cli/schemas/run.md)
- [list_available_schemas_includes_builtins](../../../../functions/src/core/schema/list_available_schemas_includes_builtins.md)
- [list_available_schemas_includes_apex_schemas](../../../../functions/src/core/schema/list_available_schemas_includes_apex_schemas.md)