---
type: Rust Function
title: sync_tasks_from_apex_log
resource: src/core/apex.rs#L334-L380
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/apex/extract_completed_task_ids
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/is_pending_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/task_id_from_pending
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/apex/run
    resolved_by: tree-sitter
    confidence: exact
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

`pub fn sync_tasks_from_apex_log(apex_log: &Path, tasks_md: &Path) -> Result<SyncReport>`

# Calls

- [extract_completed_task_ids](../../../../functions/src/core/apex/extract_completed_task_ids.md)
- [is_pending_task](../../../../functions/src/core/apex/is_pending_task.md)
- [task_id_from_pending](../../../../functions/src/core/apex/task_id_from_pending.md)

# Called by

- [run](../../../../functions/src/cli/apex/run.md)
- [sync_marks_uppercase_checkbox_pattern](../../../../functions/src/core/apex/sync_marks_uppercase_checkbox_pattern.md)
- [sync_marks_checkmark_pattern](../../../../functions/src/core/apex/sync_marks_checkmark_pattern.md)
- [sync_marks_checkbox_pattern](../../../../functions/src/core/apex/sync_marks_checkbox_pattern.md)
- [sync_leaves_unlisted_tasks_unchanged](../../../../functions/src/core/apex/sync_leaves_unlisted_tasks_unchanged.md)
- [sync_is_idempotent](../../../../functions/src/core/apex/sync_is_idempotent.md)
- [sync_preserves_trailing_newline](../../../../functions/src/core/apex/sync_preserves_trailing_newline.md)
- [sync_empty_log_returns_zero_counts](../../../../functions/src/core/apex/sync_empty_log_returns_zero_counts.md)