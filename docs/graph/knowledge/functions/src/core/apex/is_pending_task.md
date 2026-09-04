---
type: Rust Function
title: is_pending_task
resource: src/core/apex.rs#L294-L302
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/apex/build_solidspec_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_tasks_from_apex_log
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_pending_task(line: &str) -> bool`

# Called by

- [build_solidspec_context](../../../../functions/src/core/apex/build_solidspec_context.md)
- [sync_tasks_from_apex_log](../../../../functions/src/core/apex/sync_tasks_from_apex_log.md)