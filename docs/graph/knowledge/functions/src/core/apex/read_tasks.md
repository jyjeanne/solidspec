---
type: Rust Function
title: read_tasks
resource: src/core/apex.rs#L752-L754
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/apex/sync_marks_checkmark_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_leaves_unlisted_tasks_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_is_idempotent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn read_tasks(dir: &Path) -> String`

# Called by

- [sync_marks_checkmark_pattern](../../../../functions/src/core/apex/sync_marks_checkmark_pattern.md)
- [sync_leaves_unlisted_tasks_unchanged](../../../../functions/src/core/apex/sync_leaves_unlisted_tasks_unchanged.md)
- [sync_is_idempotent](../../../../functions/src/core/apex/sync_is_idempotent.md)