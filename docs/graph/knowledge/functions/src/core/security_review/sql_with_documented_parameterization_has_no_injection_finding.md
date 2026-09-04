---
type: Rust Function
title: sql_with_documented_parameterization_has_no_injection_finding
resource: src/core/security_review.rs#L389-L408
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

`fn sql_with_documented_parameterization_has_no_injection_finding()`

# Calls

- [write_feature](../../../../functions/src/core/security_review/write_feature.md)
- [run_security_review](../../../../functions/src/core/security_review/run_security_review.md)