---
type: Rust Function
title: remove_cleans_up
resource: src/extensions/manager.rs#L240-L250
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
  - target: functions/src/extensions/manager/remove_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/load_registry
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn remove_cleans_up()`

# Calls

- [create_ext_source](../../../../functions/src/extensions/manager/create_ext_source.md)
- [add_extension_dev](../../../../functions/src/extensions/manager/add_extension_dev.md)
- [remove_extension](../../../../functions/src/extensions/manager/remove_extension.md)
- [load_registry](../../../../functions/src/extensions/manager/load_registry.md)