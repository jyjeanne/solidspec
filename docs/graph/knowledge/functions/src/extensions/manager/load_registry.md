---
type: Rust Function
title: load_registry
resource: src/extensions/manager.rs#L92-L95
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/implement/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tasks/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/dev_flag_set_in_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/remove_cleans_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/enable_disable_toggle
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_registry(project_root: &Path) -> Result<ExtensionRegistry>`

# Called by

- [run](../../../../functions/src/cli/implement/run.md)
- [run](../../../../functions/src/cli/init/run.md)
- [run](../../../../functions/src/cli/tasks/run.md)
- [dev_flag_set_in_registry](../../../../functions/src/extensions/manager/dev_flag_set_in_registry.md)
- [remove_cleans_up](../../../../functions/src/extensions/manager/remove_cleans_up.md)
- [enable_disable_toggle](../../../../functions/src/extensions/manager/enable_disable_toggle.md)