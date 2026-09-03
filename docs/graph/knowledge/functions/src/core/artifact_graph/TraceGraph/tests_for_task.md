---
type: Rust Method
title: tests_for_task
resource: src/core/artifact_graph.rs#L273-L279
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
  - target: functions/src/core/artifact_graph/task_to_test_link_when_test_mentions_task_id
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn tests_for_task(&self, task_id: &str) -> Vec<&str>`

# Calls

- [as_str](../../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [format_tree](../../../../../functions/src/core/artifact_graph/TraceGraph/format_tree.md)
- [task_to_test_link_when_test_mentions_task_id](../../../../../functions/src/core/artifact_graph/task_to_test_link_when_test_mentions_task_id.md)