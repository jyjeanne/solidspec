---
type: Rust Module
title: pipeline
resource: src/core/pipeline.rs#L1-L742
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
  - target: external/super-artifact-graph-artifactgraph
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

- [PhaseType](../../../classes/src/core/pipeline/PhaseType.md)
- [PhaseStatus](../../../classes/src/core/pipeline/PhaseStatus.md)
- [fmt](../../../functions/src/core/pipeline/PhaseStatus/std-fmt-display/fmt.md)
- [PhaseResult](../../../classes/src/core/pipeline/PhaseResult.md)
- [schema_artifact_id](../../../functions/src/core/pipeline/schema_artifact_id.md)
- [should_skip](../../../functions/src/core/pipeline/should_skip.md)
- [phase_type](../../../functions/src/core/pipeline/phase_type.md)
- [filter_phases](../../../functions/src/core/pipeline/filter_phases.md)
- [format_log_entry](../../../functions/src/core/pipeline/format_log_entry.md)
- [write_log](../../../functions/src/core/pipeline/write_log.md)
- [graph_for](../../../functions/src/core/pipeline/graph_for.md)
- [filter_all_phases](../../../functions/src/core/pipeline/filter_all_phases.md)
- [filter_idsd_phases_includes_intent_and_evidence](../../../functions/src/core/pipeline/filter_idsd_phases_includes_intent_and_evidence.md)
- [filter_minimal_phases_excludes_tests_and_review](../../../functions/src/core/pipeline/filter_minimal_phases_excludes_tests_and_review.md)
- [filter_security_first_phases_includes_security_review](../../../functions/src/core/pipeline/filter_security_first_phases_includes_security_review.md)
- [filter_from_plan_to_tasks](../../../functions/src/core/pipeline/filter_from_plan_to_tasks.md)
- [filter_only_one_phase](../../../functions/src/core/pipeline/filter_only_one_phase.md)
- [filter_from_after_to_errors](../../../functions/src/core/pipeline/filter_from_after_to_errors.md)
- [filter_invalid_phase_errors](../../../functions/src/core/pipeline/filter_invalid_phase_errors.md)
- [should_skip_specify_when_spec_exists](../../../functions/src/core/pipeline/should_skip_specify_when_spec_exists.md)
- [should_skip_clarify_when_no_markers](../../../functions/src/core/pipeline/should_skip_clarify_when_no_markers.md)
- [should_not_skip_clarify_when_markers_present](../../../functions/src/core/pipeline/should_not_skip_clarify_when_markers_present.md)
- [should_skip_implement_when_all_tasks_done](../../../functions/src/core/pipeline/should_skip_implement_when_all_tasks_done.md)
- [should_not_skip_implement_when_tasks_pending](../../../functions/src/core/pipeline/should_not_skip_implement_when_tasks_pending.md)
- [analyze_never_skipped](../../../functions/src/core/pipeline/analyze_never_skipped.md)
- [phase_types_correct](../../../functions/src/core/pipeline/phase_types_correct.md)
- [should_skip_ship_when_report_exists](../../../functions/src/core/pipeline/should_skip_ship_when_report_exists.md)
- [should_not_skip_ship_when_report_absent](../../../functions/src/core/pipeline/should_not_skip_ship_when_report_absent.md)
- [should_skip_evidence_when_report_exists](../../../functions/src/core/pipeline/should_skip_evidence_when_report_exists.md)
- [should_not_skip_evidence_when_absent](../../../functions/src/core/pipeline/should_not_skip_evidence_when_absent.md)
- [should_skip_intent_when_intent_exists](../../../functions/src/core/pipeline/should_skip_intent_when_intent_exists.md)
- [should_not_skip_intent_when_absent](../../../functions/src/core/pipeline/should_not_skip_intent_when_absent.md)
- [format_log_has_table_and_totals](../../../functions/src/core/pipeline/format_log_has_table_and_totals.md)
- [write_log_creates_file](../../../functions/src/core/pipeline/write_log_creates_file.md)
- [write_log_appends_to_existing](../../../functions/src/core/pipeline/write_log_appends_to_existing.md)
- [filter_apex_driven_has_apex_not_implement](../../../functions/src/core/pipeline/filter_apex_driven_has_apex_not_implement.md)
- [filter_apex_driven_apex_at_correct_position](../../../functions/src/core/pipeline/filter_apex_driven_apex_at_correct_position.md)
- [filter_intent_apex_has_all_idsd_phases_with_apex](../../../functions/src/core/pipeline/filter_intent_apex_has_all_idsd_phases_with_apex.md)
- [filter_intent_apex_apex_before_evidence](../../../functions/src/core/pipeline/filter_intent_apex_apex_before_evidence.md)
- [filter_apex_driven_from_tasks_to_analyze](../../../functions/src/core/pipeline/filter_apex_driven_from_tasks_to_analyze.md)
- [filter_apex_driven_only_apex](../../../functions/src/core/pipeline/filter_apex_driven_only_apex.md)
- [filter_existing_schemas_unchanged_by_apex_addition](../../../functions/src/core/pipeline/filter_existing_schemas_unchanged_by_apex_addition.md)
- [should_skip_apex_false_when_no_apex_dir](../../../functions/src/core/pipeline/should_skip_apex_false_when_no_apex_dir.md)
- [should_skip_apex_false_when_apex_dir_empty](../../../functions/src/core/pipeline/should_skip_apex_false_when_apex_dir_empty.md)
- [should_skip_apex_false_when_run_dir_has_no_finish](../../../functions/src/core/pipeline/should_skip_apex_false_when_run_dir_has_no_finish.md)
- [should_skip_apex_true_when_finish_exists](../../../functions/src/core/pipeline/should_skip_apex_true_when_finish_exists.md)
- [should_skip_apex_false_when_force](../../../functions/src/core/pipeline/should_skip_apex_false_when_force.md)
- [should_skip_apex_ignores_file_entries_in_apex_dir](../../../functions/src/core/pipeline/should_skip_apex_ignores_file_entries_in_apex_dir.md)
- [should_skip_tests_when_dir_nonempty_via_schema_generates](../../../functions/src/core/pipeline/should_skip_tests_when_dir_nonempty_via_schema_generates.md)
- [should_skip_unknown_phase_defaults_false_when_absent_from_schema](../../../functions/src/core/pipeline/should_skip_unknown_phase_defaults_false_when_absent_from_schema.md)
- [should_skip_security_review_when_report_exists](../../../functions/src/core/pipeline/should_skip_security_review_when_report_exists.md)
- [filter_phases_minimal_and_security_first_never_include_undeclared_phases](../../../functions/src/core/pipeline/filter_phases_minimal_and_security_first_never_include_undeclared_phases.md)

# Imports

- `std::path::Path`
- `anyhow::Result`
- `super::artifact_graph::ArtifactGraph`
- `super::spec_parser`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)