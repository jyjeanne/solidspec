---
type: Rust Function
title: merge_preserves_existing_keys
resource: src/core/vscode.rs#L72-L86
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/vscode/merge_settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/IntentStatus/from_str
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn merge_preserves_existing_keys()`

# Calls

- [merge_settings](../../../../functions/src/core/vscode/merge_settings.md)
- [from_str](../../../../functions/src/core/intent_parser/IntentStatus/from_str.md)