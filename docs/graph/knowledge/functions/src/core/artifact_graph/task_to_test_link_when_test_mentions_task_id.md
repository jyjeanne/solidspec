---
type: Rust Function
title: task_to_test_link_when_test_mentions_task_id
resource: src/core/artifact_graph.rs#L888-L903
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/artifact_graph/build_trace_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/TraceGraph/tests_for_task
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn task_to_test_link_when_test_mentions_task_id()`

# Calls

- [build_trace_graph](../../../../functions/src/core/artifact_graph/build_trace_graph.md)
- [tests_for_task](../../../../functions/src/core/artifact_graph/TraceGraph/tests_for_task.md)