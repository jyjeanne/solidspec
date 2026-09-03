---
type: Rust Function
title: names
resource: src/core/schema.rs#L56-L66
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
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

- [every_builtin_schema_generates_without_error](../../../../functions/src/agents/spcx/every_builtin_schema_generates_without_error.md)
- [list_available_schemas](../../../../functions/src/core/schema/list_available_schemas.md)