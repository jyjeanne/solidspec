---
type: Rust Function
title: sync_preserves_trailing_newline
resource: src/core/apex.rs#L828-L842
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/apex/make_execute_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_tasks_from_apex_log
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sync_preserves_trailing_newline()`

# Calls

- [make_execute_log](../../../../functions/src/core/apex/make_execute_log.md)
- [sync_tasks_from_apex_log](../../../../functions/src/core/apex/sync_tasks_from_apex_log.md)