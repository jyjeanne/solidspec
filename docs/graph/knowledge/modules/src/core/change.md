---
type: Rust Module
title: change
resource: src/core/change.rs#L1-L634
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-lazylock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result-bail
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [DeltaSpec](../../../classes/src/core/change/DeltaSpec.md)
- [DeltaRequirement](../../../classes/src/core/change/DeltaRequirement.md)
- [DeltaModification](../../../classes/src/core/change/DeltaModification.md)
- [ChangeMetadata](../../../classes/src/core/change/ChangeMetadata.md)
- [ChangeStatus](../../../classes/src/core/change/ChangeStatus.md)
- [new](../../../functions/src/core/change/ChangeMetadata/new.md)
- [load](../../../functions/src/core/change/ChangeMetadata/load.md)
- [save](../../../functions/src/core/change/ChangeMetadata/save.md)
- [parse_delta_spec](../../../functions/src/core/change/parse_delta_spec.md)
- [extract_added](../../../functions/src/core/change/extract_added.md)
- [extract_modified](../../../functions/src/core/change/extract_modified.md)
- [extract_removed](../../../functions/src/core/change/extract_removed.md)
- [extract_section](../../../functions/src/core/change/extract_section.md)
- [merge_deltas](../../../functions/src/core/change/merge_deltas.md)
- [list_changes](../../../functions/src/core/change/list_changes.md)
- [ChangeInfo](../../../classes/src/core/change/ChangeInfo.md)
- [slugify](../../../functions/src/core/change/slugify.md)
- [create_change](../../../functions/src/core/change/create_change.md)
- [archive_change](../../../functions/src/core/change/archive_change.md)
- [parse_added_requirements](../../../functions/src/core/change/parse_added_requirements.md)
- [parse_modified_requirements](../../../functions/src/core/change/parse_modified_requirements.md)
- [parse_modified_with_multibyte_text_and_uppercase_was](../../../functions/src/core/change/parse_modified_with_multibyte_text_and_uppercase_was.md)
- [parse_removed_requirements](../../../functions/src/core/change/parse_removed_requirements.md)
- [merge_deltas_adds_new_requirements](../../../functions/src/core/change/merge_deltas_adds_new_requirements.md)
- [merge_deltas_removes_requirements](../../../functions/src/core/change/merge_deltas_removes_requirements.md)
- [merge_deltas_modifies_existing](../../../functions/src/core/change/merge_deltas_modifies_existing.md)
- [empty_delta_parsed](../../../functions/src/core/change/empty_delta_parsed.md)
- [slugify_produces_valid_slugs](../../../functions/src/core/change/slugify_produces_valid_slugs.md)
- [create_and_archive_change_roundtrip](../../../functions/src/core/change/create_and_archive_change_roundtrip.md)
- [list_changes_finds_all](../../../functions/src/core/change/list_changes_finds_all.md)
- [list_changes_skips_archive_directory](../../../functions/src/core/change/list_changes_skips_archive_directory.md)
- [archive_nonexistent_change_errors](../../../functions/src/core/change/archive_nonexistent_change_errors.md)

# Imports

- `std::path::{Path, PathBuf}`
- `std::sync::LazyLock`
- `anyhow::{Context, Result, bail}`
- `regex::Regex`
- `serde::{Deserialize, Serialize}`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)