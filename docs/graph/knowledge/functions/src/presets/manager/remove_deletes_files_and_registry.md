---
type: Rust Function
title: remove_deletes_files_and_registry
resource: src/presets/manager.rs#L189-L200
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
  - target: functions/src/presets/manager/remove_preset
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn remove_deletes_files_and_registry()`

# Calls

- [create_preset_source](../../../../functions/src/presets/manager/create_preset_source.md)
- [add_preset](../../../../functions/src/presets/manager/add_preset.md)
- [remove_preset](../../../../functions/src/presets/manager/remove_preset.md)