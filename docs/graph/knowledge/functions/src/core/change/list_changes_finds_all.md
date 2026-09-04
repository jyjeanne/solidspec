---
type: Rust Function
title: list_changes_finds_all
resource: src/core/change.rs#L595-L606
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

`fn list_changes_finds_all()`

# Calls

- [create_change](../../../../functions/src/core/change/create_change.md)
- [list_changes](../../../../functions/src/core/change/list_changes.md)