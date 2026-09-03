---
type: Rust Function
title: write_feature
resource: src/core/security_review.rs#L291-L294
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/security_review/clean_plan_with_no_sensitive_topics_has_no_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/auth_without_mitigation_raises_high_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/auth_with_documented_mitigation_has_no_authn_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/sql_without_parameterization_raises_critical_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/payment_data_without_encryption_raises_critical_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/sql_with_documented_parameterization_has_no_injection_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/sqlite_mention_alone_does_not_trigger_injection_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/tokenized_payment_data_has_no_cryptographic_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/structured_logging_documented_has_no_logging_finding
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_feature(dir: &Path, spec: &str, plan: &str)`

# Called by

- [clean_plan_with_no_sensitive_topics_has_no_findings](../../../../functions/src/core/security_review/clean_plan_with_no_sensitive_topics_has_no_findings.md)
- [auth_without_mitigation_raises_high_finding](../../../../functions/src/core/security_review/auth_without_mitigation_raises_high_finding.md)
- [auth_with_documented_mitigation_has_no_authn_finding](../../../../functions/src/core/security_review/auth_with_documented_mitigation_has_no_authn_finding.md)
- [sql_without_parameterization_raises_critical_finding](../../../../functions/src/core/security_review/sql_without_parameterization_raises_critical_finding.md)
- [payment_data_without_encryption_raises_critical_finding](../../../../functions/src/core/security_review/payment_data_without_encryption_raises_critical_finding.md)
- [sql_with_documented_parameterization_has_no_injection_finding](../../../../functions/src/core/security_review/sql_with_documented_parameterization_has_no_injection_finding.md)
- [sqlite_mention_alone_does_not_trigger_injection_finding](../../../../functions/src/core/security_review/sqlite_mention_alone_does_not_trigger_injection_finding.md)
- [tokenized_payment_data_has_no_cryptographic_finding](../../../../functions/src/core/security_review/tokenized_payment_data_has_no_cryptographic_finding.md)
- [structured_logging_documented_has_no_logging_finding](../../../../functions/src/core/security_review/structured_logging_documented_has_no_logging_finding.md)