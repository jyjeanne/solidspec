---
type: Rust Function
title: next_feature_number
resource: src/core/feature.rs#L17-L50
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

`pub fn next_feature_number(specs_dir: &Path) -> Result<u32>`

# Called by

- [run](../../../../functions/src/cli/intent/run.md)
- [run](../../../../functions/src/cli/pipeline/run.md)
- [run](../../../../functions/src/cli/specify/run.md)