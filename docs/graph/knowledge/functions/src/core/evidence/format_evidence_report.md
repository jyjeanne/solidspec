---
type: Rust Function
title: format_evidence_report
resource: src/core/evidence.rs#L168-L201
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/evidence/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/format_report_contains_table_and_header
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_evidence_report(report: &EvidenceReport) -> String`

# Called by

- [run](../../../../functions/src/cli/evidence/run.md)
- [format_report_contains_table_and_header](../../../../functions/src/core/evidence/format_report_contains_table_and_header.md)