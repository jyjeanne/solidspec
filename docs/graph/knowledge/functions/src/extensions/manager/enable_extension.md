---
type: Rust Function
title: enable_extension
resource: src/extensions/manager.rs#L57-L63
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/extensions/registry/ExtensionRegistry/enable
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/extension/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/enable_disable_toggle
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn enable_extension(project_root: &Path, ext_id: &str) -> Result<()>`

# Calls

- [enable](../../../../functions/src/extensions/registry/ExtensionRegistry/enable.md)

# Called by

- [run](../../../../functions/src/cli/extension/run.md)
- [enable_disable_toggle](../../../../functions/src/extensions/manager/enable_disable_toggle.md)