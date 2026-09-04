---
type: Rust Function
title: minimal_apply_says_schema_ends_here
resource: src/agents/spcx.rs#L285-L288
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn minimal_apply_says_schema_ends_here()`

# Calls

- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)
- [schema](../../../../functions/src/agents/spcx/schema.md)