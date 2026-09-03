---
type: Rust Function
title: list_changes_skips_archive_directory
resource: src/core/change.rs#L609-L625
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/change/create_change
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/list_changes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_changes_skips_archive_directory()`

# Calls

- [create_change](../../../../functions/src/core/change/create_change.md)
- [list_changes](../../../../functions/src/core/change/list_changes.md)