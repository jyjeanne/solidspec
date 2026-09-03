---
type: Rust Function
title: check_test_coverage
resource: src/core/review/checks.rs#L326-L395
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn check_test_coverage( spec: &spec_parser::ParsedSpec, tests_dir: &Path, ) -> Vec<ReviewFinding>`

# Called by

- [preflight_review](../../../../../functions/src/core/review/preflight_review.md)