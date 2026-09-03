---
type: Rust Function
title: check_scenario_coverage
resource: src/core/review/checks.rs#L220-L252
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn check_scenario_coverage(spec: &spec_parser::ParsedSpec) -> Vec<ReviewFinding>`

# Called by

- [preflight_review](../../../../../functions/src/core/review/preflight_review.md)