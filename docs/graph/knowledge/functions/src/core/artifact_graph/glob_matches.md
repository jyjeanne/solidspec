---
type: Rust Function
title: glob_matches
resource: src/core/artifact_graph.rs#L219-L247
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/artifact_graph/ArtifactGraph/generates_present
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn glob_matches(name: &str, pattern: &str) -> bool`

# Called by

- [generates_present](../../../../functions/src/core/artifact_graph/ArtifactGraph/generates_present.md)