---
type: Rust Method
title: has_symbol
resource: src/core/okf.rs#L162-L164
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/analyzer/structural_cross_check
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn has_symbol(&self, name: &str) -> bool`

# Called by

- [structural_cross_check](../../../../../functions/src/core/analyzer/structural_cross_check.md)