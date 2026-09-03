---
type: Rust Function
title: list_presets
resource: src/presets/manager.rs#L66-L70
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/preset/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/list_returns_sorted_by_priority
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn list_presets(project_root: &Path) -> Result<Vec<PresetEntry>>`

# Called by

- [run](../../../../functions/src/cli/preset/run.md)
- [list_returns_sorted_by_priority](../../../../functions/src/presets/manager/list_returns_sorted_by_priority.md)