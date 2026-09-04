---
type: Rust Function
title: powershell_scripts
resource: src/templates/mod.rs#L65-L73
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/templates/copy_embedded_scripts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/all_powershell_scripts_are_nonempty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/copy_embedded_scripts_creates_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn powershell_scripts() -> Vec<(&'static str, &'static str)>`

# Called by

- [copy_embedded_scripts](../../../functions/src/templates/copy_embedded_scripts.md)
- [all_powershell_scripts_are_nonempty](../../../functions/src/templates/all_powershell_scripts_are_nonempty.md)
- [copy_embedded_scripts_creates_files](../../../functions/src/templates/copy_embedded_scripts_creates_files.md)