---
type: Rust Function
title: append_continues_from_last_id
resource: src/cli/checklist.rs#L127-L133
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/cli/checklist/find_last_chk_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/checklist/generate_append_items
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn append_continues_from_last_id()`

# Calls

- [find_last_chk_id](../../../../functions/src/cli/checklist/find_last_chk_id.md)
- [generate_append_items](../../../../functions/src/cli/checklist/generate_append_items.md)