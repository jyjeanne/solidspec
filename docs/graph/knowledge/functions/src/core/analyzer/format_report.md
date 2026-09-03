---
type: Rust Function
title: format_report
resource: src/core/analyzer.rs#L381-L438
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
---

# Signature

`pub fn format_report(report: &AnalysisReport) -> String`

# Calls

- [format_tree](../../../../functions/src/core/artifact_graph/TraceGraph/format_tree.md)

# Called by

- [run](../../../../functions/src/cli/analyze/run.md)