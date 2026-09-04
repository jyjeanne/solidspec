---
type: Rust Function
title: skip_reason
resource: src/cli/pipeline.rs#L572-L589
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn skip_reason(phase: &str, _feature_dir: &std::path::Path) -> String`

# Called by

- [run](../../../../functions/src/cli/pipeline/run.md)