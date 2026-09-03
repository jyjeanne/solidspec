---
type: Rust Function
title: run
resource: src/cli/extension.rs#L49-L117
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/add_extension_dev
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/remove_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/enable_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/disable_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/list_extensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/search_extensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/info_extension
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(cmd: ExtensionCommands) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [add_extension_dev](../../../../functions/src/extensions/manager/add_extension_dev.md)
- [remove_extension](../../../../functions/src/extensions/manager/remove_extension.md)
- [enable_extension](../../../../functions/src/extensions/manager/enable_extension.md)
- [disable_extension](../../../../functions/src/extensions/manager/disable_extension.md)
- [list_extensions](../../../../functions/src/extensions/manager/list_extensions.md)
- [search_extensions](../../../../functions/src/extensions/manager/search_extensions.md)
- [info_extension](../../../../functions/src/extensions/manager/info_extension.md)