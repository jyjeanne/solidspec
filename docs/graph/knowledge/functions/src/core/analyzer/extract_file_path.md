---
type: Rust Function
title: extract_file_path
resource: src/core/analyzer.rs#L411-L421
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/analyzer/structural_cross_check
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_file_path(token: &str) -> Option<String>`

# Called by

- [structural_cross_check](../../../../functions/src/core/analyzer/structural_cross_check.md)