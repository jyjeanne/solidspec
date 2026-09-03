---
type: Rust Function
title: extract_symbol_name
resource: src/core/analyzer.rs#L359-L380
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/templates/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/analyzer/structural_cross_check
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_symbol_name(raw: &str) -> Option<String>`

# Calls

- [all](../../../../functions/src/templates/all.md)

# Called by

- [structural_cross_check](../../../../functions/src/core/analyzer/structural_cross_check.md)