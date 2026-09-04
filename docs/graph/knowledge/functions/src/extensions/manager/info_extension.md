---
type: Rust Function
title: info_extension
resource: src/extensions/manager.rs#L86-L90
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/extension/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/info_by_id
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn info_extension(project_root: &Path, name: &str) -> Result<Option<ExtensionEntry>>`

# Called by

- [run](../../../../functions/src/cli/extension/run.md)
- [info_by_id](../../../../functions/src/extensions/manager/info_by_id.md)