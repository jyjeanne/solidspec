---
type: Rust Function
title: enabled_hooks_skips_disabled
resource: src/extensions/registry.rs#L334-L345
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/extensions/registry/ExtensionRegistry/disable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/registry/ExtensionRegistry/enabled_hooks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn enabled_hooks_skips_disabled()`

# Calls

- [disable](../../../../functions/src/extensions/registry/ExtensionRegistry/disable.md)
- [enabled_hooks](../../../../functions/src/extensions/registry/ExtensionRegistry/enabled_hooks.md)