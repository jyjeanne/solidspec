---
type: Rust Function
title: first_ready_is_none_when_everything_is_done
resource: src/core/artifact_graph.rs#L649-L654
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

`fn first_ready_is_none_when_everything_is_done()`

# Calls

- [spec_driven_graph](../../../../functions/src/core/artifact_graph/spec_driven_graph.md)
- [compute_states](../../../../functions/src/core/artifact_graph/ArtifactGraph/compute_states.md)