---
type: Rust Module
title: okf
resource: src/core/okf.rs#L1-L281
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-collections-btreemap-hashset
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/okf-parser-conceptkind
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

- [GenerateReport](../../../classes/src/core/okf/GenerateReport.md)
- [generate](../../../functions/src/core/okf/generate.md)
- [validate](../../../functions/src/core/okf/validate.md)
- [refresh_if_present](../../../functions/src/core/okf/refresh_if_present.md)
- [validation_should_fail](../../../functions/src/core/okf/validation_should_fail.md)
- [BundleIndex](../../../classes/src/core/okf/BundleIndex.md)
- [load](../../../functions/src/core/okf/BundleIndex/load.md)
- [has_file](../../../functions/src/core/okf/BundleIndex/has_file.md)
- [has_symbol](../../../functions/src/core/okf/BundleIndex/has_symbol.md)
- [write_sample_project](../../../functions/src/core/okf/write_sample_project.md)
- [bundle_index_load_returns_none_when_bundle_missing](../../../functions/src/core/okf/bundle_index_load_returns_none_when_bundle_missing.md)
- [bundle_index_knows_generated_files_and_symbols](../../../functions/src/core/okf/bundle_index_knows_generated_files_and_symbols.md)
- [refresh_if_present_is_none_without_a_bundle](../../../functions/src/core/okf/refresh_if_present_is_none_without_a_bundle.md)
- [refresh_if_present_regenerates_an_existing_bundle_in_place](../../../functions/src/core/okf/refresh_if_present_regenerates_an_existing_bundle_in_place.md)
- [generate_writes_a_bundle_with_expected_concepts](../../../functions/src/core/okf/generate_writes_a_bundle_with_expected_concepts.md)
- [generate_is_incremental_on_a_second_run](../../../functions/src/core/okf/generate_is_incremental_on_a_second_run.md)
- [validate_reports_no_issues_for_a_freshly_generated_bundle](../../../functions/src/core/okf/validate_reports_no_issues_for_a_freshly_generated_bundle.md)

# Imports

- `std::collections::{BTreeMap, HashSet}`
- `std::path::Path`
- `anyhow::{Context, Result}`
- `okf_parser::ConceptKind`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)