---
type: Rust Function
title: compute_states_shows_blocked_when_deps_missing
resource: src/core/artifact_graph.rs#L665-L677
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/artifact_graph/spec_driven_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/compute_states
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn compute_states_shows_blocked_when_deps_missing()`

# Calls

- [spec_driven_graph](../../../../functions/src/core/artifact_graph/spec_driven_graph.md)
- [compute_states](../../../../functions/src/core/artifact_graph/ArtifactGraph/compute_states.md)