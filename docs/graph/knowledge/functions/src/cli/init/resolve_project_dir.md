---
type: Rust Function
title: resolve_project_dir
resource: src/cli/init.rs#L209-L220
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

`fn resolve_project_dir(name: Option<&str>, here: bool) -> Result<PathBuf>`

# Called by

- [run](../../../../functions/src/cli/init/run.md)