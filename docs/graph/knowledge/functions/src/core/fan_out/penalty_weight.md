---
type: Rust Function
title: penalty_weight
resource: src/core/fan_out.rs#L183-L191
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/fan_out/apply_penalty_formula
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/derive_score_from_keywords
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn penalty_weight(severity: &Severity) -> f64`

# Called by

- [apply_penalty_formula](../../../../functions/src/core/fan_out/apply_penalty_formula.md)
- [derive_score_from_keywords](../../../../functions/src/core/fan_out/derive_score_from_keywords.md)