---
type: Rust Module
title: manager
resource: src/extensions/manager.rs#L1-L328
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
  - target: external/super-manifest-extensionmanifest
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-registry-extensionentry-extensionregistry-hookentry
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

- [add_extension_dev](../../../functions/src/extensions/manager/add_extension_dev.md)
- [remove_extension](../../../functions/src/extensions/manager/remove_extension.md)
- [enable_extension](../../../functions/src/extensions/manager/enable_extension.md)
- [disable_extension](../../../functions/src/extensions/manager/disable_extension.md)
- [list_extensions](../../../functions/src/extensions/manager/list_extensions.md)
- [search_extensions](../../../functions/src/extensions/manager/search_extensions.md)
- [info_extension](../../../functions/src/extensions/manager/info_extension.md)
- [load_registry](../../../functions/src/extensions/manager/load_registry.md)
- [build_entry](../../../functions/src/extensions/manager/build_entry.md)
- [copy_dir_safe](../../../functions/src/extensions/manager/copy_dir_safe.md)
- [setup_project](../../../functions/src/extensions/manager/setup_project.md)
- [create_ext_source](../../../functions/src/extensions/manager/create_ext_source.md)
- [add_dev_installs_extension](../../../functions/src/extensions/manager/add_dev_installs_extension.md)
- [add_without_manifest_errors](../../../functions/src/extensions/manager/add_without_manifest_errors.md)
- [dev_flag_set_in_registry](../../../functions/src/extensions/manager/dev_flag_set_in_registry.md)
- [remove_cleans_up](../../../functions/src/extensions/manager/remove_cleans_up.md)
- [remove_nonexistent_errors](../../../functions/src/extensions/manager/remove_nonexistent_errors.md)
- [enable_disable_toggle](../../../functions/src/extensions/manager/enable_disable_toggle.md)
- [disable_already_disabled_noop](../../../functions/src/extensions/manager/disable_already_disabled_noop.md)
- [list_shows_all](../../../functions/src/extensions/manager/list_shows_all.md)
- [search_filters](../../../functions/src/extensions/manager/search_filters.md)
- [info_by_id](../../../functions/src/extensions/manager/info_by_id.md)
- [info_missing_returns_none](../../../functions/src/extensions/manager/info_missing_returns_none.md)

# Imports

- `std::collections::HashMap`
- `std::path::Path`
- `anyhow::{Result, bail}`
- `super::manifest::ExtensionManifest`
- `super::registry::{ExtensionEntry, ExtensionRegistry, HookEntry}`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)