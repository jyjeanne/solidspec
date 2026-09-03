---
type: Rust Method
title: compute_states
resource: src/core/artifact_graph.rs#L146-L169
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/status/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tasks/run
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
  - target: functions/src/core/artifact_graph/compute_states_shows_blocked_when_deps_missing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn compute_states(&self, completed: &HashSet<String>) -> HashMap<String, ArtifactState>`

# Called by

- [run](../../../../../functions/src/cli/pipeline/run.md)
- [run](../../../../../functions/src/cli/status/run.md)
- [run](../../../../../functions/src/cli/tasks/run.md)
- [first_ready_on_fresh_project_is_the_root_artifact](../../../../../functions/src/core/artifact_graph/first_ready_on_fresh_project_is_the_root_artifact.md)
- [first_ready_advances_as_artifacts_complete](../../../../../functions/src/core/artifact_graph/first_ready_advances_as_artifacts_complete.md)
- [first_ready_is_none_when_everything_is_done](../../../../../functions/src/core/artifact_graph/first_ready_is_none_when_everything_is_done.md)
- [compute_states_shows_blocked_when_deps_missing](../../../../../functions/src/core/artifact_graph/compute_states_shows_blocked_when_deps_missing.md)