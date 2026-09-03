---
type: Rust Function
title: merge_deltas
resource: src/core/change.rs#L195-L231
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/change/archive_change
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/merge_deltas_adds_new_requirements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/merge_deltas_removes_requirements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/merge_deltas_modifies_existing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn merge_deltas(main_spec: &str, delta: &DeltaSpec) -> Result<String>`

# Called by

- [archive_change](../../../../functions/src/core/change/archive_change.md)
- [merge_deltas_adds_new_requirements](../../../../functions/src/core/change/merge_deltas_adds_new_requirements.md)
- [merge_deltas_removes_requirements](../../../../functions/src/core/change/merge_deltas_removes_requirements.md)
- [merge_deltas_modifies_existing](../../../../functions/src/core/change/merge_deltas_modifies_existing.md)