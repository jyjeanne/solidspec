---
type: Rust Function
title: validation_should_fail
resource: src/core/okf.rs#L94-L96
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/okf/validate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validation_should_fail(report: &okf_validator::ValidationReport, ci: bool) -> bool`

# Called by

- [validate](../../../../functions/src/cli/okf/validate.md)