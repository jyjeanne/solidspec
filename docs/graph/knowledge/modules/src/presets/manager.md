---
type: Rust Module
title: manager
resource: src/presets/manager.rs#L1-L266
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result-bail
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-manifest-presetmanifest
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-registry-presetentry-presetregistry
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [add_preset](../../../functions/src/presets/manager/add_preset.md)
- [remove_preset](../../../functions/src/presets/manager/remove_preset.md)
- [list_presets](../../../functions/src/presets/manager/list_presets.md)
- [search_presets](../../../functions/src/presets/manager/search_presets.md)
- [info_preset](../../../functions/src/presets/manager/info_preset.md)
- [get_preset_priorities](../../../functions/src/presets/manager/get_preset_priorities.md)
- [copy_dir_recursive](../../../functions/src/presets/manager/copy_dir_recursive.md)
- [setup_project](../../../functions/src/presets/manager/setup_project.md)
- [create_preset_source](../../../functions/src/presets/manager/create_preset_source.md)
- [add_preset_copies_files_and_registers](../../../functions/src/presets/manager/add_preset_copies_files_and_registers.md)
- [add_same_preset_twice_errors](../../../functions/src/presets/manager/add_same_preset_twice_errors.md)
- [remove_deletes_files_and_registry](../../../functions/src/presets/manager/remove_deletes_files_and_registry.md)
- [remove_nonexistent_errors](../../../functions/src/presets/manager/remove_nonexistent_errors.md)
- [list_returns_sorted_by_priority](../../../functions/src/presets/manager/list_returns_sorted_by_priority.md)
- [search_filters_by_keyword](../../../functions/src/presets/manager/search_filters_by_keyword.md)
- [info_returns_entry](../../../functions/src/presets/manager/info_returns_entry.md)
- [info_missing_returns_none](../../../functions/src/presets/manager/info_missing_returns_none.md)
- [priorities_for_resolver](../../../functions/src/presets/manager/priorities_for_resolver.md)

# Imports

- `std::path::Path`
- `anyhow::{Result, bail}`
- `super::manifest::PresetManifest`
- `super::registry::{PresetEntry, PresetRegistry}`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)