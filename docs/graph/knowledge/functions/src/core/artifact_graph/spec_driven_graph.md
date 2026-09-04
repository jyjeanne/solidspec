---
type: Rust Function
title: spec_driven_graph
resource: src/core/artifact_graph.rs#L558-L618
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/artifact_graph/default_graph_has_eight_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/topological_order_starts_with_no_dependency_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/first_ready_on_fresh_project_is_the_root_artifact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/first_ready_advances_as_artifacts_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/first_ready_is_none_when_everything_is_done
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/tasks_requires_spec_and_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/compute_states_shows_blocked_when_deps_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/detect_completion_finds_existing_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/detect_completion_empty_trailing_slash_dir_is_incomplete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/detect_completion_nonempty_trailing_slash_dir_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/all_artifacts_in_default_graph_are_reachable
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn spec_driven_graph() -> ArtifactGraph`

# Called by

- [default_graph_has_eight_artifacts](../../../../functions/src/core/artifact_graph/default_graph_has_eight_artifacts.md)
- [topological_order_starts_with_no_dependency_artifacts](../../../../functions/src/core/artifact_graph/topological_order_starts_with_no_dependency_artifacts.md)
- [first_ready_on_fresh_project_is_the_root_artifact](../../../../functions/src/core/artifact_graph/first_ready_on_fresh_project_is_the_root_artifact.md)
- [first_ready_advances_as_artifacts_complete](../../../../functions/src/core/artifact_graph/first_ready_advances_as_artifacts_complete.md)
- [first_ready_is_none_when_everything_is_done](../../../../functions/src/core/artifact_graph/first_ready_is_none_when_everything_is_done.md)
- [tasks_requires_spec_and_plan](../../../../functions/src/core/artifact_graph/tasks_requires_spec_and_plan.md)
- [compute_states_shows_blocked_when_deps_missing](../../../../functions/src/core/artifact_graph/compute_states_shows_blocked_when_deps_missing.md)
- [detect_completion_finds_existing_files](../../../../functions/src/core/artifact_graph/detect_completion_finds_existing_files.md)
- [detect_completion_empty_trailing_slash_dir_is_incomplete](../../../../functions/src/core/artifact_graph/detect_completion_empty_trailing_slash_dir_is_incomplete.md)
- [detect_completion_nonempty_trailing_slash_dir_is_complete](../../../../functions/src/core/artifact_graph/detect_completion_nonempty_trailing_slash_dir_is_complete.md)
- [all_artifacts_in_default_graph_are_reachable](../../../../functions/src/core/artifact_graph/all_artifacts_in_default_graph_are_reachable.md)