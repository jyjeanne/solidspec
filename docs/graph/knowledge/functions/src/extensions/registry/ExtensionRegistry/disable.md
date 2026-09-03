---
type: Rust Method
title: disable
resource: src/extensions/registry.rs#L101-L108
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/extensions/manager/disable_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/registry/enable_disable_toggle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/registry/disable_already_disabled_is_noop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/registry/enabled_hooks_skips_disabled
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn disable(&mut self, id: &str) -> Result<()>`

# Called by

- [disable_extension](../../../../../functions/src/extensions/manager/disable_extension.md)
- [enable_disable_toggle](../../../../../functions/src/extensions/registry/enable_disable_toggle.md)
- [disable_already_disabled_is_noop](../../../../../functions/src/extensions/registry/disable_already_disabled_is_noop.md)
- [enabled_hooks_skips_disabled](../../../../../functions/src/extensions/registry/enabled_hooks_skips_disabled.md)