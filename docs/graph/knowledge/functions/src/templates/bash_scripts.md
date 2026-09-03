---
type: Rust Function
title: bash_scripts
resource: src/templates/mod.rs#L55-L63
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/templates/copy_embedded_scripts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/all_bash_scripts_are_nonempty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/copy_embedded_scripts_creates_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/setup_script_project
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn bash_scripts() -> Vec<(&'static str, &'static str)>`

# Called by

- [copy_embedded_scripts](../../../functions/src/templates/copy_embedded_scripts.md)
- [all_bash_scripts_are_nonempty](../../../functions/src/templates/all_bash_scripts_are_nonempty.md)
- [copy_embedded_scripts_creates_files](../../../functions/src/templates/copy_embedded_scripts_creates_files.md)
- [setup_script_project](../../../functions/src/templates/setup_script_project.md)