---
type: Rust Function
title: format_review_report
resource: src/core/review/report.rs#L6-L127
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/review/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/format_report_contains_intent_alignment_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/intent_alignment_section_shows_traced_when_all_good
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/no_issues_message_suppressed_when_ia_has_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/report/format_report_renders_markdown
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_review_report(report: &ReviewReport) -> String`

# Called by

- [run](../../../../../functions/src/cli/review/run.md)
- [format_report_contains_intent_alignment_section](../../../../../functions/src/core/review/format_report_contains_intent_alignment_section.md)
- [intent_alignment_section_shows_traced_when_all_good](../../../../../functions/src/core/review/intent_alignment_section_shows_traced_when_all_good.md)
- [no_issues_message_suppressed_when_ia_has_findings](../../../../../functions/src/core/review/no_issues_message_suppressed_when_ia_has_findings.md)
- [format_report_renders_markdown](../../../../../functions/src/core/review/report/format_report_renders_markdown.md)