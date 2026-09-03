---
type: Rust Module
title: vscode
resource: src/core/vscode.rs#L1-L141
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-json
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

- [merge_settings](../../../functions/src/core/vscode/merge_settings.md)
- [deep_merge](../../../functions/src/core/vscode/deep_merge.md)
- [merge_into_empty_creates_file](../../../functions/src/core/vscode/merge_into_empty_creates_file.md)
- [merge_preserves_existing_keys](../../../functions/src/core/vscode/merge_preserves_existing_keys.md)
- [deep_merge_nested_objects](../../../functions/src/core/vscode/deep_merge_nested_objects.md)
- [arrays_replaced_not_merged](../../../functions/src/core/vscode/arrays_replaced_not_merged.md)
- [non_json_existing_file_errors](../../../functions/src/core/vscode/non_json_existing_file_errors.md)

# Imports

- `std::path::Path`
- `anyhow::Result`
- `serde_json::Value`
- `super::*`
- `serde_json::json`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)