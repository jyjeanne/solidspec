---
type: Rust Function
title: two_root_graph
resource: src/core/artifact_graph.rs#L639-L664
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/artifact_graph/topological_order_is_deterministic_across_multiple_roots
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/first_ready_is_deterministic_across_multiple_roots
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn two_root_graph() -> ArtifactGraph`

# Called by

- [topological_order_is_deterministic_across_multiple_roots](../../../../functions/src/core/artifact_graph/topological_order_is_deterministic_across_multiple_roots.md)
- [first_ready_is_deterministic_across_multiple_roots](../../../../functions/src/core/artifact_graph/first_ready_is_deterministic_across_multiple_roots.md)