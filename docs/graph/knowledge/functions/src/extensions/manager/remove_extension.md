---
type: Rust Function
title: remove_extension
resource: src/extensions/manager.rs#L41-L54
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/extension/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/remove_cleans_up
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn remove_extension(project_root: &Path, ext_id: &str) -> Result<()>`

# Called by

- [run](../../../../functions/src/cli/extension/run.md)
- [remove_cleans_up](../../../../functions/src/extensions/manager/remove_cleans_up.md)