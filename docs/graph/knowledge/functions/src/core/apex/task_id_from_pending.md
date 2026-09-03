---
type: Rust Function
title: task_id_from_pending
resource: src/core/apex.rs#L414-L423
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/apex/sync_tasks_from_apex_log
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn task_id_from_pending(line: &str) -> Option<String>`

# Called by

- [sync_tasks_from_apex_log](../../../../functions/src/core/apex/sync_tasks_from_apex_log.md)