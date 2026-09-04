---
type: Rust Function
title: add_dev_installs_extension
resource: src/extensions/manager.rs#L204-L216
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
---

# Signature

`fn add_dev_installs_extension()`

# Calls

- [create_ext_source](../../../../functions/src/extensions/manager/create_ext_source.md)
- [add_extension_dev](../../../../functions/src/extensions/manager/add_extension_dev.md)