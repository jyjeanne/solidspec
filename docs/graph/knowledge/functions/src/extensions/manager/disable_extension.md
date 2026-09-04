---
type: Rust Function
title: disable_extension
resource: src/extensions/manager.rs#L66-L72
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/extensions/registry/ExtensionRegistry/disable
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/extension/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/enable_disable_toggle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/disable_already_disabled_noop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn disable_extension(project_root: &Path, ext_id: &str) -> Result<()>`

# Calls

- [disable](../../../../functions/src/extensions/registry/ExtensionRegistry/disable.md)

# Called by

- [run](../../../../functions/src/cli/extension/run.md)
- [enable_disable_toggle](../../../../functions/src/extensions/manager/enable_disable_toggle.md)
- [disable_already_disabled_noop](../../../../functions/src/extensions/manager/disable_already_disabled_noop.md)