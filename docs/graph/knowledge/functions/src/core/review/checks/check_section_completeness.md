---
type: Rust Function
title: check_section_completeness
resource: src/core/review/checks.rs#L83-L115
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/missing_sections_detected
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn check_section_completeness(content: &str) -> Vec<ReviewFinding>`

# Called by

- [preflight_review](../../../../../functions/src/core/review/preflight_review.md)
- [missing_sections_detected](../../../../../functions/src/core/review/checks/missing_sections_detected.md)