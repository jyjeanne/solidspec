---
type: Rust Function
title: drift_detects_unsatisfied_criteria
resource: src/core/analyzer.rs#L808-L827
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/analyzer/write_test
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/compute_drift
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn drift_detects_unsatisfied_criteria()`

# Calls

- [write_test](../../../../functions/src/core/analyzer/write_test.md)
- [compute_drift](../../../../functions/src/core/analyzer/compute_drift.md)