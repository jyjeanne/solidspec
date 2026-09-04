---
type: Rust Method
title: format_tree
resource: src/core/artifact_graph.rs#L305-L377
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/TraceGraph/tasks_for_req
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/TraceGraph/tests_for_task
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/analyzer/format_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/format_tree_contains_fr_ids
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_tree(&self) -> String`

# Calls

- [as_str](../../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)
- [tasks_for_req](../../../../../functions/src/core/artifact_graph/TraceGraph/tasks_for_req.md)
- [tests_for_task](../../../../../functions/src/core/artifact_graph/TraceGraph/tests_for_task.md)

# Called by

- [format_report](../../../../../functions/src/core/analyzer/format_report.md)
- [format_tree_contains_fr_ids](../../../../../functions/src/core/artifact_graph/format_tree_contains_fr_ids.md)