---
type: Rust Function
title: mark_task_done
resource: src/cli/implement.rs#L77-L82
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/implement/mark_task_done_updates_checkbox
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/implement/mark_nonexistent_task_is_noop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn mark_task_done(tasks_path: &Path, task_id: &str) -> Result<()>`

# Called by

- [mark_task_done_updates_checkbox](../../../../functions/src/cli/implement/mark_task_done_updates_checkbox.md)
- [mark_nonexistent_task_is_noop](../../../../functions/src/cli/implement/mark_nonexistent_task_is_noop.md)