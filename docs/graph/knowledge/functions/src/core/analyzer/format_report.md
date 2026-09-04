---
type: Rust Function
title: format_report
resource: src/core/analyzer.rs#L541-L620
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/artifact_graph/TraceGraph/format_tree
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/analyze/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/format_report_omits_structural_section_without_a_bundle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/format_report_includes_structural_section_with_a_bundle
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_report(report: &AnalysisReport) -> String`

# Calls

- [format_tree](../../../../functions/src/core/artifact_graph/TraceGraph/format_tree.md)

# Called by

- [run](../../../../functions/src/cli/analyze/run.md)
- [format_report_omits_structural_section_without_a_bundle](../../../../functions/src/core/analyzer/format_report_omits_structural_section_without_a_bundle.md)
- [format_report_includes_structural_section_with_a_bundle](../../../../functions/src/core/analyzer/format_report_includes_structural_section_with_a_bundle.md)