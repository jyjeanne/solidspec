---
type: Rust Function
title: first_ready_advances_as_artifacts_complete
resource: src/core/artifact_graph.rs#L631-L646
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
  - target: functions/src/core/artifact_graph/ArtifactGraph/first_ready
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn first_ready_advances_as_artifacts_complete()`

# Calls

- [spec_driven_graph](../../../../functions/src/core/artifact_graph/spec_driven_graph.md)
- [compute_states](../../../../functions/src/core/artifact_graph/ArtifactGraph/compute_states.md)
- [first_ready](../../../../functions/src/core/artifact_graph/ArtifactGraph/first_ready.md)