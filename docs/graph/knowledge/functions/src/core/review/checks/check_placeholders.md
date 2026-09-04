---
type: Rust Function
title: check_placeholders
resource: src/core/review/checks.rs#L62-L80
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/placeholder_detection
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn check_placeholders(content: &str, file_name: &str) -> Vec<ReviewFinding>`

# Called by

- [preflight_review](../../../../../functions/src/core/review/preflight_review.md)
- [placeholder_detection](../../../../../functions/src/core/review/checks/placeholder_detection.md)