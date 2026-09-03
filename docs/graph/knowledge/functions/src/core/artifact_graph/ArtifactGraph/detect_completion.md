---
type: Rust Method
title: detect_completion
resource: src/core/artifact_graph.rs#L174-L180
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/artifact_graph/ArtifactGraph/generates_present
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/core/artifact_graph/detect_completion_finds_existing_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/detect_completion_empty_trailing_slash_dir_is_incomplete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/detect_completion_nonempty_trailing_slash_dir_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/detect_completion_glob_requires_matching_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn detect_completion(&self, feature_dir: &Path) -> HashSet<String>`

# Calls

- [generates_present](../../../../../functions/src/core/artifact_graph/ArtifactGraph/generates_present.md)

# Called by

- [run](../../../../../functions/src/cli/pipeline/run.md)
- [run](../../../../../functions/src/cli/status/run.md)
- [run](../../../../../functions/src/cli/tasks/run.md)
- [detect_completion_finds_existing_files](../../../../../functions/src/core/artifact_graph/detect_completion_finds_existing_files.md)
- [detect_completion_empty_trailing_slash_dir_is_incomplete](../../../../../functions/src/core/artifact_graph/detect_completion_empty_trailing_slash_dir_is_incomplete.md)
- [detect_completion_nonempty_trailing_slash_dir_is_complete](../../../../../functions/src/core/artifact_graph/detect_completion_nonempty_trailing_slash_dir_is_complete.md)
- [detect_completion_glob_requires_matching_file](../../../../../functions/src/core/artifact_graph/detect_completion_glob_requires_matching_file.md)