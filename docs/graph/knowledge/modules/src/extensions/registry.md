---
type: Rust Module
title: registry
resource: src/extensions/registry.rs#L1-L346
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

- [ExtensionEntry](../../../classes/src/extensions/registry/ExtensionEntry.md)
- [HookEntry](../../../classes/src/extensions/registry/HookEntry.md)
- [ExtensionRegistry](../../../classes/src/extensions/registry/ExtensionRegistry.md)
- [load](../../../functions/src/extensions/registry/ExtensionRegistry/load.md)
- [save](../../../functions/src/extensions/registry/ExtensionRegistry/save.md)
- [add](../../../functions/src/extensions/registry/ExtensionRegistry/add.md)
- [remove](../../../functions/src/extensions/registry/ExtensionRegistry/remove.md)
- [get](../../../functions/src/extensions/registry/ExtensionRegistry/get.md)
- [update](../../../functions/src/extensions/registry/ExtensionRegistry/update.md)
- [enable](../../../functions/src/extensions/registry/ExtensionRegistry/enable.md)
- [disable](../../../functions/src/extensions/registry/ExtensionRegistry/disable.md)
- [list](../../../functions/src/extensions/registry/ExtensionRegistry/list.md)
- [search](../../../functions/src/extensions/registry/ExtensionRegistry/search.md)
- [resolve](../../../functions/src/extensions/registry/ExtensionRegistry/resolve.md)
- [enabled_hooks](../../../functions/src/extensions/registry/ExtensionRegistry/enabled_hooks.md)
- [sample_entry](../../../functions/src/extensions/registry/sample_entry.md)
- [add_and_get_deep_copy](../../../functions/src/extensions/registry/add_and_get_deep_copy.md)
- [duplicate_add_errors](../../../functions/src/extensions/registry/duplicate_add_errors.md)
- [update_preserves_timestamp](../../../functions/src/extensions/registry/update_preserves_timestamp.md)
- [update_nonexistent_errors](../../../functions/src/extensions/registry/update_nonexistent_errors.md)
- [enable_disable_toggle](../../../functions/src/extensions/registry/enable_disable_toggle.md)
- [disable_already_disabled_is_noop](../../../functions/src/extensions/registry/disable_already_disabled_is_noop.md)
- [remove_deletes_entry](../../../functions/src/extensions/registry/remove_deletes_entry.md)
- [remove_nonexistent_errors](../../../functions/src/extensions/registry/remove_nonexistent_errors.md)
- [corrupted_registry_starts_fresh](../../../functions/src/extensions/registry/corrupted_registry_starts_fresh.md)
- [load_empty_registry](../../../functions/src/extensions/registry/load_empty_registry.md)
- [save_and_reload](../../../functions/src/extensions/registry/save_and_reload.md)
- [search_by_name](../../../functions/src/extensions/registry/search_by_name.md)
- [resolve_by_id](../../../functions/src/extensions/registry/resolve_by_id.md)
- [resolve_by_display_name](../../../functions/src/extensions/registry/resolve_by_display_name.md)
- [resolve_missing_returns_none](../../../functions/src/extensions/registry/resolve_missing_returns_none.md)
- [enabled_hooks_skips_disabled](../../../functions/src/extensions/registry/enabled_hooks_skips_disabled.md)

# Imports

- `std::collections::HashMap`
- `std::path::Path`
- `anyhow::{Result, bail}`
- `serde::{Deserialize, Serialize}`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)