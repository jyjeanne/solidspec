---
type: Rust Module
title: registry
resource: src/presets/registry.rs#L1-L219
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result-bail
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
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

- [PresetEntry](../../../classes/src/presets/registry/PresetEntry.md)
- [PresetRegistry](../../../classes/src/presets/registry/PresetRegistry.md)
- [load](../../../functions/src/presets/registry/PresetRegistry/load.md)
- [save](../../../functions/src/presets/registry/PresetRegistry/save.md)
- [add](../../../functions/src/presets/registry/PresetRegistry/add.md)
- [remove](../../../functions/src/presets/registry/PresetRegistry/remove.md)
- [get](../../../functions/src/presets/registry/PresetRegistry/get.md)
- [list](../../../functions/src/presets/registry/PresetRegistry/list.md)
- [sorted_priorities](../../../functions/src/presets/registry/PresetRegistry/sorted_priorities.md)
- [search](../../../functions/src/presets/registry/PresetRegistry/search.md)
- [sample_entry](../../../functions/src/presets/registry/sample_entry.md)
- [add_appears_in_registry](../../../functions/src/presets/registry/add_appears_in_registry.md)
- [duplicate_id_errors](../../../functions/src/presets/registry/duplicate_id_errors.md)
- [remove_gone_from_registry](../../../functions/src/presets/registry/remove_gone_from_registry.md)
- [remove_nonexistent_errors](../../../functions/src/presets/registry/remove_nonexistent_errors.md)
- [list_sorted_by_priority](../../../functions/src/presets/registry/list_sorted_by_priority.md)
- [sorted_priorities_for_resolver](../../../functions/src/presets/registry/sorted_priorities_for_resolver.md)
- [save_and_reload](../../../functions/src/presets/registry/save_and_reload.md)
- [load_empty_file](../../../functions/src/presets/registry/load_empty_file.md)
- [load_missing_file](../../../functions/src/presets/registry/load_missing_file.md)
- [search_by_name](../../../functions/src/presets/registry/search_by_name.md)
- [get_returns_deep_copy](../../../functions/src/presets/registry/get_returns_deep_copy.md)

# Imports

- `std::collections::HashMap`
- `std::path::Path`
- `anyhow::{Result, bail}`
- `serde::{Deserialize, Serialize}`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)