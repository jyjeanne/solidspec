---
type: Rust Function
title: build_entry
resource: src/extensions/manager.rs#L97-L136
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/extensions/manager/add_extension_dev
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_entry(manifest: &ExtensionManifest, dev: bool) -> ExtensionEntry`

# Called by

- [add_extension_dev](../../../../functions/src/extensions/manager/add_extension_dev.md)