---
type: Rust Function
title: merge_into_empty_creates_file
resource: src/core/vscode.rs#L59-L69
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

`fn merge_into_empty_creates_file()`

# Calls

- [merge_settings](../../../../functions/src/core/vscode/merge_settings.md)
- [from_str](../../../../functions/src/core/intent_parser/IntentStatus/from_str.md)