---
type: Rust Function
title: create_and_archive_change_roundtrip
resource: src/core/change.rs#L557-L592
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/change/create_change
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/archive_change
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_and_archive_change_roundtrip()`

# Calls

- [create_change](../../../../functions/src/core/change/create_change.md)
- [archive_change](../../../../functions/src/core/change/archive_change.md)