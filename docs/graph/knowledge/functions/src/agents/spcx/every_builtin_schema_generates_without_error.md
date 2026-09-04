---
type: Rust Function
title: every_builtin_schema_generates_without_error
resource: src/agents/spcx.rs#L280-L286
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/schema/names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn every_builtin_schema_generates_without_error()`

# Calls

- [names](../../../../functions/src/core/schema/names.md)
- [schema](../../../../functions/src/agents/spcx/schema.md)
- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)