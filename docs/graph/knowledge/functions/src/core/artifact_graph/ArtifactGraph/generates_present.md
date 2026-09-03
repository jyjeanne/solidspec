---
type: Rust Method
title: generates_present
resource: src/core/artifact_graph.rs#L173-L201
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/templates/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/glob_matches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/artifact_graph/ArtifactGraph/detect_completion
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generates_present(node: &ArtifactNode, feature_dir: &Path) -> bool`

# Calls

- [all](../../../../../functions/src/templates/all.md)
- [as_str](../../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)
- [glob_matches](../../../../../functions/src/core/artifact_graph/glob_matches.md)

# Called by

- [detect_completion](../../../../../functions/src/core/artifact_graph/ArtifactGraph/detect_completion.md)
- [should_skip](../../../../../functions/src/core/pipeline/should_skip.md)