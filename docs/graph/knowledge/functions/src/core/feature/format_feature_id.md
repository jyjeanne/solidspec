---
type: Rust Function
title: format_feature_id
resource: src/core/feature.rs#L53-L55
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/intent/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_feature_id(num: u32) -> String`

# Called by

- [run](../../../../functions/src/cli/intent/run.md)
- [run](../../../../functions/src/cli/pipeline/run.md)
- [run](../../../../functions/src/cli/specify/run.md)