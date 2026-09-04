---
type: Rust Function
title: init_project_with_feature
resource: tests/change.rs#L9-L20
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/change/change_propose_creates_directory_and_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/change/change_list_shows_active_changes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/change/change_archive_merges_deltas_and_moves_to_archive
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn init_project_with_feature(dir: &std::path::Path)`

# Called by

- [change_propose_creates_directory_and_files](../../../functions/tests/change/change_propose_creates_directory_and_files.md)
- [change_list_shows_active_changes](../../../functions/tests/change/change_list_shows_active_changes.md)
- [change_archive_merges_deltas_and_moves_to_archive](../../../functions/tests/change/change_archive_merges_deltas_and_moves_to_archive.md)