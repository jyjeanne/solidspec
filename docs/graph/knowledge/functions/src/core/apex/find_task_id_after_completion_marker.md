---
type: Rust Function
title: find_task_id_after_completion_marker
resource: src/core/apex.rs#L398-L411
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/apex/extract_completed_task_ids
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn find_task_id_after_completion_marker(line: &str) -> Option<String>`

# Called by

- [extract_completed_task_ids](../../../../functions/src/core/apex/extract_completed_task_ids.md)