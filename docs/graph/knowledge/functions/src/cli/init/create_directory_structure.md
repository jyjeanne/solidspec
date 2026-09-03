---
type: Rust Function
title: create_directory_structure
resource: src/cli/init.rs#L103-L128
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_directory_structure(project_dir: &Path) -> Result<()>`

# Called by

- [run](../../../../functions/src/cli/init/run.md)