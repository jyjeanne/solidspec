---
type: Rust Function
title: remove_preset
resource: src/presets/manager.rs#L49-L63
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/preset/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/remove_deletes_files_and_registry
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn remove_preset(project_root: &Path, preset_id: &str) -> Result<()>`

# Called by

- [run](../../../../functions/src/cli/preset/run.md)
- [remove_deletes_files_and_registry](../../../../functions/src/presets/manager/remove_deletes_files_and_registry.md)