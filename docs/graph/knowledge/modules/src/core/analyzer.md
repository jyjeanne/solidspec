---
type: Rust Module
title: analyzer
resource: src/core/analyzer.rs#L1-L1086
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
  - target: external/super-artifact-graph-self-tracegraph
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-constitution
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-errors-solidspecerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-intent-parser-self-intentdrift
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-okf-bundleindex-default-bundle-dir
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-spec-parser
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

- [Severity](../../../classes/src/core/analyzer/Severity.md)
- [fmt](../../../functions/src/core/analyzer/Severity/std-fmt-display/fmt.md)
- [Finding](../../../classes/src/core/analyzer/Finding.md)
- [AnalysisReport](../../../classes/src/core/analyzer/AnalysisReport.md)
- [analyze_feature](../../../functions/src/core/analyzer/analyze_feature.md)
- [structural_cross_check](../../../functions/src/core/analyzer/structural_cross_check.md)
- [extract_symbol_name](../../../functions/src/core/analyzer/extract_symbol_name.md)
- [extract_file_path](../../../functions/src/core/analyzer/extract_file_path.md)
- [compute_drift](../../../functions/src/core/analyzer/compute_drift.md)
- [format_report](../../../functions/src/core/analyzer/format_report.md)
- [setup_feature](../../../functions/src/core/analyzer/setup_feature.md)
- [setup_constitution](../../../functions/src/core/analyzer/setup_constitution.md)
- [fully_traced_artifacts_high_score](../../../functions/src/core/analyzer/fully_traced_artifacts_high_score.md)
- [missing_plan_is_high_finding](../../../functions/src/core/analyzer/missing_plan_is_high_finding.md)
- [orphan_tasks_medium_finding](../../../functions/src/core/analyzer/orphan_tasks_medium_finding.md)
- [constitution_violation_is_critical](../../../functions/src/core/analyzer/constitution_violation_is_critical.md)
- [missing_spec_returns_error](../../../functions/src/core/analyzer/missing_spec_returns_error.md)
- [max_findings_enforced](../../../functions/src/core/analyzer/max_findings_enforced.md)
- [analyze_does_not_modify_files](../../../functions/src/core/analyzer/analyze_does_not_modify_files.md)
- [remediation_suggestions_present](../../../functions/src/core/analyzer/remediation_suggestions_present.md)
- [write_intent](../../../functions/src/core/analyzer/write_intent.md)
- [write_test](../../../functions/src/core/analyzer/write_test.md)
- [drift_none_when_no_intent_md](../../../functions/src/core/analyzer/drift_none_when_no_intent_md.md)
- [drift_zero_at_baseline_all_not_implemented](../../../functions/src/core/analyzer/drift_zero_at_baseline_all_not_implemented.md)
- [drift_zero_when_no_test_files](../../../functions/src/core/analyzer/drift_zero_when_no_test_files.md)
- [drift_detects_unsatisfied_criteria](../../../functions/src/core/analyzer/drift_detects_unsatisfied_criteria.md)
- [drift_score_100_when_all_criteria_uncovered](../../../functions/src/core/analyzer/drift_score_100_when_all_criteria_uncovered.md)
- [extract_symbol_name_strips_call_syntax_and_qualification](../../../functions/src/core/analyzer/extract_symbol_name_strips_call_syntax_and_qualification.md)
- [extract_symbol_name_rejects_non_identifiers](../../../functions/src/core/analyzer/extract_symbol_name_rejects_non_identifiers.md)
- [extract_file_path_accepts_recognized_source_extensions](../../../functions/src/core/analyzer/extract_file_path_accepts_recognized_source_extensions.md)
- [extract_file_path_rejects_non_paths](../../../functions/src/core/analyzer/extract_file_path_rejects_non_paths.md)
- [generate_bundle_for](../../../functions/src/core/analyzer/generate_bundle_for.md)
- [structural_cross_check_is_none_without_a_bundle](../../../functions/src/core/analyzer/structural_cross_check_is_none_without_a_bundle.md)
- [structural_cross_check_flags_unknown_backtick_symbol](../../../functions/src/core/analyzer/structural_cross_check_flags_unknown_backtick_symbol.md)
- [structural_cross_check_accepts_a_real_symbol](../../../functions/src/core/analyzer/structural_cross_check_accepts_a_real_symbol.md)
- [structural_cross_check_flags_existing_file_missing_from_a_stale_bundle](../../../functions/src/core/analyzer/structural_cross_check_flags_existing_file_missing_from_a_stale_bundle.md)
- [structural_cross_check_ignores_files_that_do_not_exist_yet](../../../functions/src/core/analyzer/structural_cross_check_ignores_files_that_do_not_exist_yet.md)
- [format_report_omits_structural_section_without_a_bundle](../../../functions/src/core/analyzer/format_report_omits_structural_section_without_a_bundle.md)
- [format_report_includes_structural_section_with_a_bundle](../../../functions/src/core/analyzer/format_report_includes_structural_section_with_a_bundle.md)

# Imports

- `std::path::Path`
- `std::sync::LazyLock`
- `anyhow::Result`
- `regex::Regex`
- `super::artifact_graph::{self, TraceGraph}`
- `super::constitution`
- `super::errors::SolidSpecError`
- `super::intent_parser::{self, IntentDrift}`
- `super::okf::{BundleIndex, DEFAULT_BUNDLE_DIR}`
- `super::spec_parser`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)