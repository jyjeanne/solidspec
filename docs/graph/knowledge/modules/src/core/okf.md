---
type: Rust Module
title: okf
resource: src/core/okf.rs#L1-L151
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-collections-btreemap
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
- [validation_should_fail](../../../functions/src/core/okf/validation_should_fail.md)
- [write_sample_project](../../../functions/src/core/okf/write_sample_project.md)
- [generate_writes_a_bundle_with_expected_concepts](../../../functions/src/core/okf/generate_writes_a_bundle_with_expected_concepts.md)
- [generate_is_incremental_on_a_second_run](../../../functions/src/core/okf/generate_is_incremental_on_a_second_run.md)
- [validate_reports_no_issues_for_a_freshly_generated_bundle](../../../functions/src/core/okf/validate_reports_no_issues_for_a_freshly_generated_bundle.md)

# Imports

- `std::collections::BTreeMap`
- `std::path::Path`
- `anyhow::{Context, Result}`
- `okf_parser::ConceptKind`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)