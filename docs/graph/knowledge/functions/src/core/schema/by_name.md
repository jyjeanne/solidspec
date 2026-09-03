---
type: Rust Function
title: by_name
resource: src/core/schema.rs#L64-L75
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/schema/resolve_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/list_available_schemas
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn by_name(name: &str) -> Option<&'static str>`

# Called by

- [resolve_schema](../../../../functions/src/core/schema/resolve_schema.md)
- [list_available_schemas](../../../../functions/src/core/schema/list_available_schemas.md)