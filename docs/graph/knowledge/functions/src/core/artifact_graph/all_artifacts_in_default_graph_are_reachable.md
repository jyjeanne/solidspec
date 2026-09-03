---
type: Rust Function
title: all_artifacts_in_default_graph_are_reachable
resource: src/core/artifact_graph.rs#L790-L803
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
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn all_artifacts_in_default_graph_are_reachable()`

# Calls

- [spec_driven_graph](../../../../functions/src/core/artifact_graph/spec_driven_graph.md)
- [topological_order](../../../../functions/src/core/artifact_graph/ArtifactGraph/topological_order.md)
- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)