---
type: Rust Function
title: apex_driven_converts_to_valid_graph
resource: src/core/schema.rs#L408-L413
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

`fn apex_driven_converts_to_valid_graph()`

# Calls

- [into_graph](../../../../functions/src/core/schema/WorkflowSchema/into_graph.md)
- [topological_order](../../../../functions/src/core/artifact_graph/ArtifactGraph/topological_order.md)