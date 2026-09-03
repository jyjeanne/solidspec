---
type: Rust Function
title: format_security_review
resource: src/core/security_review.rs#L216-L284
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/security_review/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/format_report_renders_markdown_with_severity_sections
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/format_report_clean_says_no_concerns_detected
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_security_review(report: &SecurityReviewReport) -> String`

# Called by

- [run](../../../../functions/src/cli/security_review/run.md)
- [format_report_renders_markdown_with_severity_sections](../../../../functions/src/core/security_review/format_report_renders_markdown_with_severity_sections.md)
- [format_report_clean_says_no_concerns_detected](../../../../functions/src/core/security_review/format_report_clean_says_no_concerns_detected.md)