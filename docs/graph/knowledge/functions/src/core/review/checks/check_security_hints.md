---
type: Rust Function
title: check_security_hints
resource: src/core/review/checks.rs#L398-L424
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn check_security_hints(plan_content: &str, spec_content: &str) -> Vec<ReviewFinding>`

# Called by

- [preflight_review](../../../../../functions/src/core/review/preflight_review.md)