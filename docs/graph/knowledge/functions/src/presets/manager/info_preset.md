---
type: Rust Function
title: info_preset
resource: src/presets/manager.rs#L80-L84
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/preset/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/info_returns_entry
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn info_preset(project_root: &Path, preset_id: &str) -> Result<Option<PresetEntry>>`

# Called by

- [run](../../../../functions/src/cli/preset/run.md)
- [info_returns_entry](../../../../functions/src/presets/manager/info_returns_entry.md)