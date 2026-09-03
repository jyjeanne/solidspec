---
type: Rust Function
title: make_execute_log
resource: src/core/apex.rs#L748-L750
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/apex/sync_marks_uppercase_checkbox_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_marks_checkmark_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_marks_checkbox_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_leaves_unlisted_tasks_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_is_idempotent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_preserves_trailing_newline
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_empty_log_returns_zero_counts
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_execute_log(dir: &Path, content: &str)`

# Called by

- [sync_marks_uppercase_checkbox_pattern](../../../../functions/src/core/apex/sync_marks_uppercase_checkbox_pattern.md)
- [sync_marks_checkmark_pattern](../../../../functions/src/core/apex/sync_marks_checkmark_pattern.md)
- [sync_marks_checkbox_pattern](../../../../functions/src/core/apex/sync_marks_checkbox_pattern.md)
- [sync_leaves_unlisted_tasks_unchanged](../../../../functions/src/core/apex/sync_leaves_unlisted_tasks_unchanged.md)
- [sync_is_idempotent](../../../../functions/src/core/apex/sync_is_idempotent.md)
- [sync_preserves_trailing_newline](../../../../functions/src/core/apex/sync_preserves_trailing_newline.md)
- [sync_empty_log_returns_zero_counts](../../../../functions/src/core/apex/sync_empty_log_returns_zero_counts.md)