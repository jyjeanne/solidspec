---
type: Rust Function
title: write_sample_project
resource: src/core/okf.rs#L149-L155
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/okf/bundle_index_knows_generated_files_and_symbols
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/okf/generate_writes_a_bundle_with_expected_concepts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/okf/generate_is_incremental_on_a_second_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/okf/validate_reports_no_issues_for_a_freshly_generated_bundle
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_sample_project(dir: &Path)`

# Called by

- [bundle_index_knows_generated_files_and_symbols](../../../../functions/src/core/okf/bundle_index_knows_generated_files_and_symbols.md)
- [generate_writes_a_bundle_with_expected_concepts](../../../../functions/src/core/okf/generate_writes_a_bundle_with_expected_concepts.md)
- [generate_is_incremental_on_a_second_run](../../../../functions/src/core/okf/generate_is_incremental_on_a_second_run.md)
- [validate_reports_no_issues_for_a_freshly_generated_bundle](../../../../functions/src/core/okf/validate_reports_no_issues_for_a_freshly_generated_bundle.md)