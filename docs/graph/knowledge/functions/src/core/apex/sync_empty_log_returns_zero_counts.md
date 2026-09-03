---
type: Rust Function
title: sync_empty_log_returns_zero_counts
resource: src/core/apex.rs#L845-L860
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/apex/make_tasks_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/make_execute_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_tasks_from_apex_log
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sync_empty_log_returns_zero_counts()`

# Calls

- [make_tasks_md](../../../../functions/src/core/apex/make_tasks_md.md)
- [make_execute_log](../../../../functions/src/core/apex/make_execute_log.md)
- [sync_tasks_from_apex_log](../../../../functions/src/core/apex/sync_tasks_from_apex_log.md)