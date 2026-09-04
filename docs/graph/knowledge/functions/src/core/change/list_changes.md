---
type: Rust Function
title: list_changes
resource: src/core/change.rs#L234-L283
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/change/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/list_changes_finds_all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/list_changes_skips_archive_directory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn list_changes(feature_dir: &Path) -> Result<Vec<ChangeInfo>>`

# Called by

- [run](../../../../functions/src/cli/change/run.md)
- [list_changes_finds_all](../../../../functions/src/core/change/list_changes_finds_all.md)
- [list_changes_skips_archive_directory](../../../../functions/src/core/change/list_changes_skips_archive_directory.md)