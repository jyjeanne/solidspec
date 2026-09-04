---
type: Rust Function
title: check_cross_references
resource: src/core/review/checks.rs#L255-L288
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/cross_reference_gaps_found
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn check_cross_references( spec: &spec_parser::ParsedSpec, plan_content: &str, file_name: &str, ) -> Vec<ReviewFinding>`

# Called by

- [preflight_review](../../../../../functions/src/core/review/preflight_review.md)
- [cross_reference_gaps_found](../../../../../functions/src/core/review/checks/cross_reference_gaps_found.md)