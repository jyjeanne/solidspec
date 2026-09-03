---
type: Rust Function
title: check_ambiguous_language
resource: src/core/review/checks.rs#L118-L174
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/ambiguous_language_flagged
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn check_ambiguous_language(content: &str) -> Vec<ReviewFinding>`

# Called by

- [preflight_review](../../../../../functions/src/core/review/preflight_review.md)
- [ambiguous_language_flagged](../../../../../functions/src/core/review/checks/ambiguous_language_flagged.md)