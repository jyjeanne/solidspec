---
type: Rust Function
title: generate_append_items
resource: src/cli/checklist.rs#L87-L101
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/checklist/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/checklist/append_items_start_from_given_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/checklist/append_continues_from_last_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/checklist/checklist_items_match_format
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn generate_append_items(start_id: u32) -> String`

# Called by

- [run](../../../../functions/src/cli/checklist/run.md)
- [append_items_start_from_given_id](../../../../functions/src/cli/checklist/append_items_start_from_given_id.md)
- [append_continues_from_last_id](../../../../functions/src/cli/checklist/append_continues_from_last_id.md)
- [checklist_items_match_format](../../../../functions/src/cli/checklist/checklist_items_match_format.md)