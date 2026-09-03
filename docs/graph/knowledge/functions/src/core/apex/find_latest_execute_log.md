---
type: Rust Function
title: find_latest_execute_log
resource: src/core/apex.rs#L427-L441
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/apex/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/find_latest_execute_log_finds_log_in_subdir
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn find_latest_execute_log(apex_output_dir: &Path) -> Option<PathBuf>`

# Called by

- [run](../../../../functions/src/cli/apex/run.md)
- [find_latest_execute_log_finds_log_in_subdir](../../../../functions/src/core/apex/find_latest_execute_log_finds_log_in_subdir.md)