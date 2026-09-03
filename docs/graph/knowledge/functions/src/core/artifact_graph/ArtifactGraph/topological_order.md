---
type: Rust Method
title: topological_order
resource: src/core/artifact_graph.rs#L95-L130
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/status/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/first_ready
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/topological_order_starts_with_no_dependency_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/all_artifacts_in_default_graph_are_reachable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/schema_converts_to_valid_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/apex_driven_converts_to_valid_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/intent_apex_converts_to_valid_graph
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn topological_order(&self) -> Result<Vec<&ArtifactNode>, String>`

# Calls

- [as_str](../../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [run](../../../../../functions/src/cli/status/run.md)
- [first_ready](../../../../../functions/src/core/artifact_graph/ArtifactGraph/first_ready.md)
- [topological_order_starts_with_no_dependency_artifacts](../../../../../functions/src/core/artifact_graph/topological_order_starts_with_no_dependency_artifacts.md)
- [all_artifacts_in_default_graph_are_reachable](../../../../../functions/src/core/artifact_graph/all_artifacts_in_default_graph_are_reachable.md)
- [schema_converts_to_valid_graph](../../../../../functions/src/core/schema/schema_converts_to_valid_graph.md)
- [apex_driven_converts_to_valid_graph](../../../../../functions/src/core/schema/apex_driven_converts_to_valid_graph.md)
- [intent_apex_converts_to_valid_graph](../../../../../functions/src/core/schema/intent_apex_converts_to_valid_graph.md)