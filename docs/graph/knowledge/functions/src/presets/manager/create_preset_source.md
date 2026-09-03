---
type: Rust Function
title: create_preset_source
resource: src/presets/manager.rs#L134-L149
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/presets/manager/add_preset_copies_files_and_registers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/add_same_preset_twice_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/remove_deletes_files_and_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/list_returns_sorted_by_priority
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/search_filters_by_keyword
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/info_returns_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/priorities_for_resolver
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_preset_source(dir: &Path, id: &str) -> std::path::PathBuf`

# Called by

- [add_preset_copies_files_and_registers](../../../../functions/src/presets/manager/add_preset_copies_files_and_registers.md)
- [add_same_preset_twice_errors](../../../../functions/src/presets/manager/add_same_preset_twice_errors.md)
- [remove_deletes_files_and_registry](../../../../functions/src/presets/manager/remove_deletes_files_and_registry.md)
- [list_returns_sorted_by_priority](../../../../functions/src/presets/manager/list_returns_sorted_by_priority.md)
- [search_filters_by_keyword](../../../../functions/src/presets/manager/search_filters_by_keyword.md)
- [info_returns_entry](../../../../functions/src/presets/manager/info_returns_entry.md)
- [priorities_for_resolver](../../../../functions/src/presets/manager/priorities_for_resolver.md)