---
type: Rust Function
title: tokenized_payment_data_has_no_cryptographic_finding
resource: src/core/security_review.rs#L431-L448
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

`fn tokenized_payment_data_has_no_cryptographic_finding()`

# Calls

- [write_feature](../../../../functions/src/core/security_review/write_feature.md)
- [run_security_review](../../../../functions/src/core/security_review/run_security_review.md)