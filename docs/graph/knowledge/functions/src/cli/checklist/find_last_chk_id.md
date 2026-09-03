---
type: Rust Function
title: find_last_chk_id
resource: src/cli/checklist.rs#L77-L85
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/checklist/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/checklist/append_continues_from_last_id
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn find_last_chk_id(content: &str) -> u32`

# Called by

- [run](../../../../functions/src/cli/checklist/run.md)
- [append_continues_from_last_id](../../../../functions/src/cli/checklist/append_continues_from_last_id.md)