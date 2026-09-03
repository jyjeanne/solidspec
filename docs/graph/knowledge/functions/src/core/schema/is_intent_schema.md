---
type: Rust Function
title: is_intent_schema
resource: src/core/schema.rs#L143-L145
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/write_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/status/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_intent_schema(name: &str) -> bool`

# Called by

- [run](../../../../functions/src/cli/plan/run.md)
- [run](../../../../functions/src/cli/specify/run.md)
- [write_spec](../../../../functions/src/cli/specify/write_spec.md)
- [run](../../../../functions/src/cli/status/run.md)