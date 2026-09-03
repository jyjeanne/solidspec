---
type: Rust Function
title: create_change
resource: src/core/change.rs#L314-L347
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/change/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/create_and_archive_change_roundtrip
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

`pub fn create_change(feature_dir: &Path, title: &str) -> Result<(String, PathBuf)>`

# Called by

- [run](../../../../functions/src/cli/change/run.md)
- [create_and_archive_change_roundtrip](../../../../functions/src/core/change/create_and_archive_change_roundtrip.md)
- [list_changes_finds_all](../../../../functions/src/core/change/list_changes_finds_all.md)
- [list_changes_skips_archive_directory](../../../../functions/src/core/change/list_changes_skips_archive_directory.md)