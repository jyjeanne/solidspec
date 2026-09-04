---
type: Rust Method
title: has_file
resource: src/core/okf.rs#L157-L159
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/analyzer/structural_cross_check
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn has_file(&self, path: &str) -> bool`

# Called by

- [structural_cross_check](../../../../../functions/src/core/analyzer/structural_cross_check.md)