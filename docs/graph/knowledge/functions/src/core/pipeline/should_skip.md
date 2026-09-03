---
type: Rust Function
title: should_skip
resource: src/core/pipeline.rs#L137-L194
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/spec_parser/parse_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/schema_artifact_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/generates_present
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn should_skip(phase: &str, feature_dir: &Path, force: bool, graph: &ArtifactGraph) -> bool`

# Calls

- [parse_spec](../../../../functions/src/core/spec_parser/parse_spec.md)
- [schema_artifact_id](../../../../functions/src/core/pipeline/schema_artifact_id.md)
- [generates_present](../../../../functions/src/core/artifact_graph/ArtifactGraph/generates_present.md)

# Called by

- [run](../../../../functions/src/cli/pipeline/run.md)