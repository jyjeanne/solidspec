---
type: Rust Function
title: topological_order_starts_with_no_dependency_artifacts
resource: src/core/artifact_graph.rs#L604-L608
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/artifact_graph/spec_driven_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/topological_order
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn topological_order_starts_with_no_dependency_artifacts()`

# Calls

- [spec_driven_graph](../../../../functions/src/core/artifact_graph/spec_driven_graph.md)
- [topological_order](../../../../functions/src/core/artifact_graph/ArtifactGraph/topological_order.md)