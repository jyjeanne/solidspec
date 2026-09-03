---
type: Rust Function
title: detect_completion_nonempty_trailing_slash_dir_is_complete
resource: src/core/artifact_graph.rs#L774-L787
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/artifact_graph/spec_driven_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/detect_completion
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detect_completion_nonempty_trailing_slash_dir_is_complete()`

# Calls

- [spec_driven_graph](../../../../functions/src/core/artifact_graph/spec_driven_graph.md)
- [detect_completion](../../../../functions/src/core/artifact_graph/ArtifactGraph/detect_completion.md)