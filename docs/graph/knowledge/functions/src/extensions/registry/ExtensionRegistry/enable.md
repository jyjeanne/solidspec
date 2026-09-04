---
type: Rust Method
title: enable
resource: src/extensions/registry.rs#L92-L99
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/extensions/manager/enable_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/registry/enable_disable_toggle
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn enable(&mut self, id: &str) -> Result<()>`

# Called by

- [enable_extension](../../../../../functions/src/extensions/manager/enable_extension.md)
- [enable_disable_toggle](../../../../../functions/src/extensions/registry/enable_disable_toggle.md)