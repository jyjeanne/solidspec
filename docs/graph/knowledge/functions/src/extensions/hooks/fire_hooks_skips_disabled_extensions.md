---
type: Rust Function
title: fire_hooks_skips_disabled_extensions
resource: src/extensions/hooks.rs#L114-L137
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/extensions/registry/ExtensionRegistry/enabled_hooks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/hooks/fire_hooks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fire_hooks_skips_disabled_extensions()`

# Calls

- [enabled_hooks](../../../../functions/src/extensions/registry/ExtensionRegistry/enabled_hooks.md)
- [fire_hooks](../../../../functions/src/extensions/hooks/fire_hooks.md)