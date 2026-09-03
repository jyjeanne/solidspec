---
type: Rust Function
title: list_shows_all
resource: src/extensions/manager.rs#L288-L298
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
  - target: functions/src/extensions/manager/list_extensions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_shows_all()`

# Calls

- [create_ext_source](../../../../functions/src/extensions/manager/create_ext_source.md)
- [add_extension_dev](../../../../functions/src/extensions/manager/add_extension_dev.md)
- [list_extensions](../../../../functions/src/extensions/manager/list_extensions.md)