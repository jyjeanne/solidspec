---
type: Rust Function
title: list_extensions
resource: src/extensions/manager.rs#L74-L78
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/extension/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/list_shows_all
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn list_extensions(project_root: &Path) -> Result<Vec<ExtensionEntry>>`

# Called by

- [run](../../../../functions/src/cli/extension/run.md)
- [list_shows_all](../../../../functions/src/extensions/manager/list_shows_all.md)