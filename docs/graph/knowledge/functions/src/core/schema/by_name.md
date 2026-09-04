---
type: Rust Function
title: by_name
resource: src/core/schema.rs#L68-L79
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/spcx/schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
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

- [schema](../../../../functions/src/agents/spcx/schema.md)
- [run](../../../../functions/src/cli/init/run.md)
- [resolve_schema](../../../../functions/src/core/schema/resolve_schema.md)
- [list_available_schemas](../../../../functions/src/core/schema/list_available_schemas.md)