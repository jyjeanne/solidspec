---
type: Rust Function
title: topological_order_is_deterministic_across_multiple_roots
resource: src/core/artifact_graph.rs#L667-L678
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/artifact_graph/two_root_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/topological_order
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn topological_order_is_deterministic_across_multiple_roots()`

# Calls

- [two_root_graph](../../../../functions/src/core/artifact_graph/two_root_graph.md)
- [topological_order](../../../../functions/src/core/artifact_graph/ArtifactGraph/topological_order.md)