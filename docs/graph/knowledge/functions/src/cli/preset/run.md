---
type: Rust Function
title: run
resource: src/cli/preset.rs#L39-L98
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/add_preset
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/remove_preset
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/list_presets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/search_presets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/info_preset
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(cmd: PresetCommands) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [add_preset](../../../../functions/src/presets/manager/add_preset.md)
- [remove_preset](../../../../functions/src/presets/manager/remove_preset.md)
- [list_presets](../../../../functions/src/presets/manager/list_presets.md)
- [search_presets](../../../../functions/src/presets/manager/search_presets.md)
- [info_preset](../../../../functions/src/presets/manager/info_preset.md)