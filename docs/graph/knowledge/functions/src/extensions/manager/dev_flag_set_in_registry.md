---
type: Rust Function
title: dev_flag_set_in_registry
resource: src/extensions/manager.rs#L229-L237
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
  - target: functions/src/extensions/manager/load_registry
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn dev_flag_set_in_registry()`

# Calls

- [create_ext_source](../../../../functions/src/extensions/manager/create_ext_source.md)
- [add_extension_dev](../../../../functions/src/extensions/manager/add_extension_dev.md)
- [load_registry](../../../../functions/src/extensions/manager/load_registry.md)