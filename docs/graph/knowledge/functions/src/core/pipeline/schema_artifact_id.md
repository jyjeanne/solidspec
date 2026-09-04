---
type: Rust Function
title: schema_artifact_id
resource: src/core/pipeline.rs#L125-L127
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/pipeline/should_skip
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn schema_artifact_id(phase: &str) -> &str`

# Called by

- [should_skip](../../../../functions/src/core/pipeline/should_skip.md)