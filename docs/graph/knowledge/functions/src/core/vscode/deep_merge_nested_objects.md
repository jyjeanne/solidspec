---
type: Rust Function
title: deep_merge_nested_objects
resource: src/core/vscode.rs#L89-L108
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

`fn deep_merge_nested_objects()`

# Calls

- [merge_settings](../../../../functions/src/core/vscode/merge_settings.md)
- [from_str](../../../../functions/src/core/intent_parser/IntentStatus/from_str.md)