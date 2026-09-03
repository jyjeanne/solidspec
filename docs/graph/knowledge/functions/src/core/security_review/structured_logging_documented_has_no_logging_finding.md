---
type: Rust Function
title: structured_logging_documented_has_no_logging_finding
resource: src/core/security_review.rs#L451-L468
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/security_review/write_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/run_security_review
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn structured_logging_documented_has_no_logging_finding()`

# Calls

- [write_feature](../../../../functions/src/core/security_review/write_feature.md)
- [run_security_review](../../../../functions/src/core/security_review/run_security_review.md)