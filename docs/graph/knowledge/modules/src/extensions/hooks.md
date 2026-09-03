---
type: Rust Module
title: hooks
resource: src/extensions/hooks.rs#L1-L138
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-registry-extensionregistry
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-extensions-registry-extensionentry-hookentry
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
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

- [fire_hooks](../../../functions/src/extensions/hooks/fire_hooks.md)
- [fire_hooks_skips_missing_file](../../../functions/src/extensions/hooks/fire_hooks_skips_missing_file.md)
- [fire_hooks_skips_disabled_extensions](../../../functions/src/extensions/hooks/fire_hooks_skips_disabled_extensions.md)

# Imports

- `std::path::Path`
- `std::process::Command`
- `super::registry::ExtensionRegistry`
- `super::*`
- `crate::extensions::registry::{ExtensionEntry, HookEntry}`
- `std::collections::HashMap`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)