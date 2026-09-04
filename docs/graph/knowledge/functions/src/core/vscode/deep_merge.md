---
type: Rust Function
title: deep_merge
resource: src/core/vscode.rs#L34-L50
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/vscode/merge_settings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn deep_merge(base: Value, overlay: Value) -> Value`

# Called by

- [merge_settings](../../../../functions/src/core/vscode/merge_settings.md)