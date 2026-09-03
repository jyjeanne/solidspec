---
type: Rust Function
title: first_ready_on_fresh_project_is_the_root_artifact
resource: src/core/artifact_graph.rs#L624-L628
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

`fn first_ready_on_fresh_project_is_the_root_artifact()`

# Calls

- [spec_driven_graph](../../../../functions/src/core/artifact_graph/spec_driven_graph.md)
- [compute_states](../../../../functions/src/core/artifact_graph/ArtifactGraph/compute_states.md)