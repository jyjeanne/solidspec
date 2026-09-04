---
type: Rust Function
title: copy_embedded_scripts_creates_files
resource: src/templates/mod.rs#L248-L265
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/templates/copy_embedded_scripts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/bash_scripts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/powershell_scripts
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn copy_embedded_scripts_creates_files()`

# Calls

- [copy_embedded_scripts](../../../functions/src/templates/copy_embedded_scripts.md)
- [bash_scripts](../../../functions/src/templates/bash_scripts.md)
- [powershell_scripts](../../../functions/src/templates/powershell_scripts.md)