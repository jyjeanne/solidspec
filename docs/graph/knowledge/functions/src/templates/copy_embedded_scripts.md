---
type: Rust Function
title: copy_embedded_scripts
resource: src/templates/mod.rs#L118-L132
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/templates/bash_scripts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/powershell_scripts
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/upgrade/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/copy_embedded_scripts_creates_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/copy_embedded_scripts_overwrites_existing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn copy_embedded_scripts(solidspec_dir: &Path) -> Result<()>`

# Calls

- [bash_scripts](../../../functions/src/templates/bash_scripts.md)
- [powershell_scripts](../../../functions/src/templates/powershell_scripts.md)

# Called by

- [run](../../../functions/src/cli/init/run.md)
- [run](../../../functions/src/cli/upgrade/run.md)
- [copy_embedded_scripts_creates_files](../../../functions/src/templates/copy_embedded_scripts_creates_files.md)
- [copy_embedded_scripts_overwrites_existing](../../../functions/src/templates/copy_embedded_scripts_overwrites_existing.md)