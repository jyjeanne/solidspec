---
type: Rust Module
title: change
resource: tests/change.rs#L1-L116
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/predicates-prelude
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  - target: external/common-solidspec
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [init_project_with_feature](../../functions/tests/change/init_project_with_feature.md)
- [change_propose_creates_directory_and_files](../../functions/tests/change/change_propose_creates_directory_and_files.md)
- [change_list_shows_active_changes](../../functions/tests/change/change_list_shows_active_changes.md)
- [change_archive_merges_deltas_and_moves_to_archive](../../functions/tests/change/change_archive_merges_deltas_and_moves_to_archive.md)

# Imports

- `predicates::prelude::*`
- `tempfile::TempDir`
- `common::solidspec`

# Member of

- [solidspec](../../packages/solidspec.md)