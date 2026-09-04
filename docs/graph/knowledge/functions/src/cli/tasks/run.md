---
type: Rust Function
title: run
resource: src/cli/tasks.rs#L8-L75
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
  - target: functions/src/core/spec_parser/parse_spec
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
  - target: functions/src/core/task_generator/generate_tasks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/format_task_list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/load_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/hooks/fire_hooks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_id: Option<&str>, schema_name: &str) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [parse_spec](../../../../functions/src/core/spec_parser/parse_spec.md)
- [load_graph](../../../../functions/src/core/schema/load_graph.md)
- [detect_completion](../../../../functions/src/core/artifact_graph/ArtifactGraph/detect_completion.md)
- [compute_states](../../../../functions/src/core/artifact_graph/ArtifactGraph/compute_states.md)
- [generate_tasks](../../../../functions/src/core/task_generator/generate_tasks.md)
- [format_task_list](../../../../functions/src/core/task_generator/format_task_list.md)
- [load_registry](../../../../functions/src/extensions/manager/load_registry.md)
- [fire_hooks](../../../../functions/src/extensions/hooks/fire_hooks.md)