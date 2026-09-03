---
type: Rust Function
title: check_requirement_quality
resource: src/core/review/checks.rs#L177-L217
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/empty_spec_means_no_requirements
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn check_requirement_quality(spec: &spec_parser::ParsedSpec) -> Vec<ReviewFinding>`

# Called by

- [preflight_review](../../../../../functions/src/core/review/preflight_review.md)
- [empty_spec_means_no_requirements](../../../../../functions/src/core/review/checks/empty_spec_means_no_requirements.md)