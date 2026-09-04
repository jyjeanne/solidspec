---
type: Rust Function
title: make_tasks_md
resource: src/core/apex.rs#L744-L746
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
  - target: functions/src/core/apex/sync_empty_log_returns_zero_counts
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_tasks_md(dir: &Path, lines: &[&str])`

# Called by

- [sync_marks_uppercase_checkbox_pattern](../../../../functions/src/core/apex/sync_marks_uppercase_checkbox_pattern.md)
- [sync_marks_checkmark_pattern](../../../../functions/src/core/apex/sync_marks_checkmark_pattern.md)
- [sync_marks_checkbox_pattern](../../../../functions/src/core/apex/sync_marks_checkbox_pattern.md)
- [sync_leaves_unlisted_tasks_unchanged](../../../../functions/src/core/apex/sync_leaves_unlisted_tasks_unchanged.md)
- [sync_is_idempotent](../../../../functions/src/core/apex/sync_is_idempotent.md)
- [sync_empty_log_returns_zero_counts](../../../../functions/src/core/apex/sync_empty_log_returns_zero_counts.md)