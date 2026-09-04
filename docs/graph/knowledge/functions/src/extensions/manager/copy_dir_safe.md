---
type: Rust Function
title: copy_dir_safe
resource: src/extensions/manager.rs#L138-L165
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/extensions/manager/add_extension_dev
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn copy_dir_safe(src: &Path, dst: &Path) -> Result<()>`

# Called by

- [add_extension_dev](../../../../functions/src/extensions/manager/add_extension_dev.md)