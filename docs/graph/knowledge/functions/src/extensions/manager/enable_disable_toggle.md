---
type: Rust Function
title: enable_disable_toggle
resource: src/extensions/manager.rs#L260-L274
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/extensions/manager/create_ext_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/add_extension_dev
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/disable_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/load_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/enable_extension
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn enable_disable_toggle()`

# Calls

- [create_ext_source](../../../../functions/src/extensions/manager/create_ext_source.md)
- [add_extension_dev](../../../../functions/src/extensions/manager/add_extension_dev.md)
- [disable_extension](../../../../functions/src/extensions/manager/disable_extension.md)
- [load_registry](../../../../functions/src/extensions/manager/load_registry.md)
- [enable_extension](../../../../functions/src/extensions/manager/enable_extension.md)