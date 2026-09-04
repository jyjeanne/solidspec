---
type: Rust Function
title: copy_dir_recursive
resource: src/presets/manager.rs#L93-L122
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/presets/manager/add_preset
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()>`

# Called by

- [add_preset](../../../../functions/src/presets/manager/add_preset.md)