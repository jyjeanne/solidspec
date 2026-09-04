---
type: Rust Method
title: first_ready
resource: src/core/artifact_graph.rs#L148-L153
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/artifact_graph/ArtifactGraph/topological_order
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/artifact_graph/first_ready_advances_as_artifacts_complete
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn first_ready(&self, states: &HashMap<String, ArtifactState>) -> Option<&ArtifactNode>`

# Calls

- [topological_order](../../../../../functions/src/core/artifact_graph/ArtifactGraph/topological_order.md)

# Called by

- [first_ready_advances_as_artifacts_complete](../../../../../functions/src/core/artifact_graph/first_ready_advances_as_artifacts_complete.md)