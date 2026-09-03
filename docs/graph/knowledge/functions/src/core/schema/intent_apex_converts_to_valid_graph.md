---
type: Rust Function
title: intent_apex_converts_to_valid_graph
resource: src/core/schema.rs#L466-L471
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/schema/WorkflowSchema/into_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/topological_order
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn intent_apex_converts_to_valid_graph()`

# Calls

- [into_graph](../../../../functions/src/core/schema/WorkflowSchema/into_graph.md)
- [topological_order](../../../../functions/src/core/artifact_graph/ArtifactGraph/topological_order.md)