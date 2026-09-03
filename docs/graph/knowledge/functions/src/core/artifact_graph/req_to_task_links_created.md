---
type: Rust Function
title: req_to_task_links_created
resource: src/core/artifact_graph.rs#L855-L865
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/artifact_graph/build_trace_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/TraceGraph/tasks_for_req
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn req_to_task_links_created()`

# Calls

- [build_trace_graph](../../../../functions/src/core/artifact_graph/build_trace_graph.md)
- [tasks_for_req](../../../../functions/src/core/artifact_graph/TraceGraph/tasks_for_req.md)