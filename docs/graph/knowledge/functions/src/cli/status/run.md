---
type: Rust Function
title: run
resource: src/cli/status.rs#L8-L110
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/load_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/detect_completion
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/compute_states
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/topological_order
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/is_intent_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/compute_drift
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_id: Option<&str>, schema_name: &str) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [load_graph](../../../../functions/src/core/schema/load_graph.md)
- [detect_completion](../../../../functions/src/core/artifact_graph/ArtifactGraph/detect_completion.md)
- [compute_states](../../../../functions/src/core/artifact_graph/ArtifactGraph/compute_states.md)
- [topological_order](../../../../functions/src/core/artifact_graph/ArtifactGraph/topological_order.md)
- [is_intent_schema](../../../../functions/src/core/schema/is_intent_schema.md)
- [compute_drift](../../../../functions/src/core/analyzer/compute_drift.md)