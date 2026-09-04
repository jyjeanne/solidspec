---
type: Rust Function
title: extract_removed
resource: src/core/change.rs#L167-L177
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/change/parse_delta_spec
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_removed(content: &str) -> Vec<String>`

# Called by

- [parse_delta_spec](../../../../functions/src/core/change/parse_delta_spec.md)