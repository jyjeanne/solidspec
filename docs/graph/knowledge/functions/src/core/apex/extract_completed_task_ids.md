---
type: Rust Function
title: extract_completed_task_ids
resource: src/core/apex.rs#L383-L394
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/apex/find_task_id_after_completion_marker
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/apex/sync_tasks_from_apex_log
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_completed_task_ids(log: &str) -> Vec<String>`

# Calls

- [find_task_id_after_completion_marker](../../../../functions/src/core/apex/find_task_id_after_completion_marker.md)

# Called by

- [sync_tasks_from_apex_log](../../../../functions/src/core/apex/sync_tasks_from_apex_log.md)