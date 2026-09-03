---
type: Rust Method
title: tasks_for_req
resource: src/core/artifact_graph.rs#L265-L271
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/artifact_graph/TraceGraph/format_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/req_to_task_links_created
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn tasks_for_req(&self, req_id: &str) -> Vec<&str>`

# Calls

- [as_str](../../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [format_tree](../../../../../functions/src/core/artifact_graph/TraceGraph/format_tree.md)
- [req_to_task_links_created](../../../../../functions/src/core/artifact_graph/req_to_task_links_created.md)