---
type: Rust Function
title: first_ready_is_deterministic_across_multiple_roots
resource: src/core/artifact_graph.rs#L681-L687
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/artifact_graph/two_root_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/compute_states
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn first_ready_is_deterministic_across_multiple_roots()`

# Calls

- [two_root_graph](../../../../functions/src/core/artifact_graph/two_root_graph.md)
- [compute_states](../../../../functions/src/core/artifact_graph/ArtifactGraph/compute_states.md)