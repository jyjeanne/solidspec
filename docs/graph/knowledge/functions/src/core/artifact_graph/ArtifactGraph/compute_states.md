---
type: Rust Method
title: compute_states
resource: src/core/artifact_graph.rs#L133-L156
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/status/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tasks/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/compute_states_shows_blocked_when_deps_missing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn compute_states(&self, completed: &HashSet<String>) -> HashMap<String, ArtifactState>`

# Called by

- [run](../../../../../functions/src/cli/status/run.md)
- [run](../../../../../functions/src/cli/tasks/run.md)
- [compute_states_shows_blocked_when_deps_missing](../../../../../functions/src/core/artifact_graph/compute_states_shows_blocked_when_deps_missing.md)