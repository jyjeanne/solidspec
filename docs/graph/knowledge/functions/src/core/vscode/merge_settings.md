---
type: Rust Function
title: merge_settings
resource: src/core/vscode.rs#L13-L32
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/from_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/vscode/deep_merge
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/vscode/merge_into_empty_creates_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/vscode/merge_preserves_existing_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/vscode/deep_merge_nested_objects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/vscode/arrays_replaced_not_merged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/vscode/non_json_existing_file_errors
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn merge_settings(project_dir: &Path, new_settings: &Value) -> Result<()>`

# Calls

- [from_str](../../../../functions/src/core/intent_parser/IntentStatus/from_str.md)
- [deep_merge](../../../../functions/src/core/vscode/deep_merge.md)

# Called by

- [merge_into_empty_creates_file](../../../../functions/src/core/vscode/merge_into_empty_creates_file.md)
- [merge_preserves_existing_keys](../../../../functions/src/core/vscode/merge_preserves_existing_keys.md)
- [deep_merge_nested_objects](../../../../functions/src/core/vscode/deep_merge_nested_objects.md)
- [arrays_replaced_not_merged](../../../../functions/src/core/vscode/arrays_replaced_not_merged.md)
- [non_json_existing_file_errors](../../../../functions/src/core/vscode/non_json_existing_file_errors.md)