---
type: Rust Function
title: auth_with_documented_mitigation_has_no_authn_finding
resource: src/core/security_review.rs#L335-L350
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

`fn auth_with_documented_mitigation_has_no_authn_finding()`

# Calls

- [write_feature](../../../../functions/src/core/security_review/write_feature.md)
- [run_security_review](../../../../functions/src/core/security_review/run_security_review.md)