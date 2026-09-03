---
type: Rust Module
title: security_review
resource: src/core/security_review.rs#L1-L510
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-lazylock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-errors-solidspecerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-review-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [SecurityFinding](../../../classes/src/core/security_review/SecurityFinding.md)
- [SecurityReviewReport](../../../classes/src/core/security_review/SecurityReviewReport.md)
- [OwaspCheck](../../../classes/src/core/security_review/OwaspCheck.md)
- [run_security_review](../../../functions/src/core/security_review/run_security_review.md)
- [format_security_review](../../../functions/src/core/security_review/format_security_review.md)
- [write_feature](../../../functions/src/core/security_review/write_feature.md)
- [missing_plan_returns_error](../../../functions/src/core/security_review/missing_plan_returns_error.md)
- [clean_plan_with_no_sensitive_topics_has_no_findings](../../../functions/src/core/security_review/clean_plan_with_no_sensitive_topics_has_no_findings.md)
- [auth_without_mitigation_raises_high_finding](../../../functions/src/core/security_review/auth_without_mitigation_raises_high_finding.md)
- [auth_with_documented_mitigation_has_no_authn_finding](../../../functions/src/core/security_review/auth_with_documented_mitigation_has_no_authn_finding.md)
- [sql_without_parameterization_raises_critical_finding](../../../functions/src/core/security_review/sql_without_parameterization_raises_critical_finding.md)
- [payment_data_without_encryption_raises_critical_finding](../../../functions/src/core/security_review/payment_data_without_encryption_raises_critical_finding.md)
- [sql_with_documented_parameterization_has_no_injection_finding](../../../functions/src/core/security_review/sql_with_documented_parameterization_has_no_injection_finding.md)
- [sqlite_mention_alone_does_not_trigger_injection_finding](../../../functions/src/core/security_review/sqlite_mention_alone_does_not_trigger_injection_finding.md)
- [tokenized_payment_data_has_no_cryptographic_finding](../../../functions/src/core/security_review/tokenized_payment_data_has_no_cryptographic_finding.md)
- [structured_logging_documented_has_no_logging_finding](../../../functions/src/core/security_review/structured_logging_documented_has_no_logging_finding.md)
- [spec_missing_is_tolerated_plan_only_still_runs](../../../functions/src/core/security_review/spec_missing_is_tolerated_plan_only_still_runs.md)
- [format_report_renders_markdown_with_severity_sections](../../../functions/src/core/security_review/format_report_renders_markdown_with_severity_sections.md)
- [format_report_clean_says_no_concerns_detected](../../../functions/src/core/security_review/format_report_clean_says_no_concerns_detected.md)

# Imports

- `std::path::Path`
- `std::sync::LazyLock`
- `anyhow::Result`
- `regex::Regex`
- `super::errors::SolidSpecError`
- `super::review::Severity`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)