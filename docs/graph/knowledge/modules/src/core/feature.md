---
type: Rust Module
title: feature
resource: src/core/feature.rs#L1-L396
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-lazylock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-errors-solidspecerror
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

- [next_feature_number](../../../functions/src/core/feature/next_feature_number.md)
- [format_feature_id](../../../functions/src/core/feature/format_feature_id.md)
- [generate_branch_name](../../../functions/src/core/feature/generate_branch_name.md)
- [sanitize](../../../functions/src/core/feature/sanitize.md)
- [is_valid_feature_branch](../../../functions/src/core/feature/is_valid_feature_branch.md)
- [resolve_feature](../../../functions/src/core/feature/resolve_feature.md)
- [find_feature_dir_by_prefix](../../../functions/src/core/feature/find_feature_dir_by_prefix.md)
- [latest_feature_dir](../../../functions/src/core/feature/latest_feature_dir.md)
- [empty_specs_returns_001](../../../functions/src/core/feature/empty_specs_returns_001.md)
- [nonexistent_specs_returns_001](../../../functions/src/core/feature/nonexistent_specs_returns_001.md)
- [existing_001_002_returns_003](../../../functions/src/core/feature/existing_001_002_returns_003.md)
- [non_sequential_gaps_use_global_max_plus_one](../../../functions/src/core/feature/non_sequential_gaps_use_global_max_plus_one.md)
- [ignores_non_matching_dirs](../../../functions/src/core/feature/ignores_non_matching_dirs.md)
- [format_feature_id_zero_pads](../../../functions/src/core/feature/format_feature_id_zero_pads.md)
- [generate_branch_name_from_description](../../../functions/src/core/feature/generate_branch_name_from_description.md)
- [empty_description_returns_error](../../../functions/src/core/feature/empty_description_returns_error.md)
- [is_valid_feature_branch_matches_pattern](../../../functions/src/core/feature/is_valid_feature_branch_matches_pattern.md)
- [resolve_explicit_arg_wins](../../../functions/src/core/feature/resolve_explicit_arg_wins.md)
- [resolve_env_var_and_latest_fallback](../../../functions/src/core/feature/resolve_env_var_and_latest_fallback.md)
- [resolve_empty_specs_returns_error](../../../functions/src/core/feature/resolve_empty_specs_returns_error.md)
- [find_feature_dir_single_match](../../../functions/src/core/feature/find_feature_dir_single_match.md)
- [find_feature_dir_no_match](../../../functions/src/core/feature/find_feature_dir_no_match.md)
- [find_feature_dir_multiple_picks_latest](../../../functions/src/core/feature/find_feature_dir_multiple_picks_latest.md)

# Imports

- `std::path::Path`
- `std::sync::LazyLock`
- `anyhow::Result`
- `regex::Regex`
- `super::errors::SolidSpecError`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)