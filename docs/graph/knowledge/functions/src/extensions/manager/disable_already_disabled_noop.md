---
type: Rust Function
title: disable_already_disabled_noop
resource: src/extensions/manager.rs#L277-L285
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
---

# Signature

`fn disable_already_disabled_noop()`

# Calls

- [create_ext_source](../../../../functions/src/extensions/manager/create_ext_source.md)
- [add_extension_dev](../../../../functions/src/extensions/manager/add_extension_dev.md)
- [disable_extension](../../../../functions/src/extensions/manager/disable_extension.md)