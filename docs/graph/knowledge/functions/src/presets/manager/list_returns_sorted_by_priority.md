---
type: Rust Function
title: list_returns_sorted_by_priority
resource: src/presets/manager.rs#L210-L222
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/presets/manager/create_preset_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/add_preset
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/list_presets
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_returns_sorted_by_priority()`

# Calls

- [create_preset_source](../../../../functions/src/presets/manager/create_preset_source.md)
- [add_preset](../../../../functions/src/presets/manager/add_preset.md)
- [list_presets](../../../../functions/src/presets/manager/list_presets.md)