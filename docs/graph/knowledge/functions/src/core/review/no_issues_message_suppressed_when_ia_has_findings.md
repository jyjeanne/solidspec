---
type: Rust Function
title: no_issues_message_suppressed_when_ia_has_findings
resource: src/core/review.rs#L487-L506
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/report/format_review_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn no_issues_message_suppressed_when_ia_has_findings()`

# Calls

- [preflight_review](../../../../functions/src/core/review/preflight_review.md)
- [format_review_report](../../../../functions/src/core/review/report/format_review_report.md)