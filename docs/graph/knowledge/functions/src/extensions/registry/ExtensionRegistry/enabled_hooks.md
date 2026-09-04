---
type: Rust Method
title: enabled_hooks
resource: src/extensions/registry.rs#L154-L165
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/extensions/hooks/fire_hooks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/hooks/fire_hooks_skips_disabled_extensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/registry/enabled_hooks_skips_disabled
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn enabled_hooks(&self, trigger: &str) -> Vec<(String, HookEntry)>`

# Called by

- [fire_hooks](../../../../../functions/src/extensions/hooks/fire_hooks.md)
- [fire_hooks_skips_disabled_extensions](../../../../../functions/src/extensions/hooks/fire_hooks_skips_disabled_extensions.md)
- [enabled_hooks_skips_disabled](../../../../../functions/src/extensions/registry/enabled_hooks_skips_disabled.md)