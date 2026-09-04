---
type: Rust Module
title: tdd
resource: tests/tdd.rs#L1-L1416
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
  - target: external/common-first-feature-dir-init-project-solidspec
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [create_feature](../../functions/tests/tdd/create_feature.md)
- [tdd_tests_hidden_from_top_level_help_but_still_registered](../../functions/tests/tdd/tdd_tests_hidden_from_top_level_help_but_still_registered.md)
- [tdd_refactor_hidden_from_top_level_help_but_still_registered](../../functions/tests/tdd/tdd_refactor_hidden_from_top_level_help_but_still_registered.md)
- [tdd_tests_help_shows_dry_run_flag](../../functions/tests/tdd/tdd_tests_help_shows_dry_run_flag.md)
- [tdd_refactor_help_shows_dry_run_flag](../../functions/tests/tdd/tdd_refactor_help_shows_dry_run_flag.md)
- [tdd_tests_fails_outside_project](../../functions/tests/tdd/tdd_tests_fails_outside_project.md)
- [tdd_refactor_fails_outside_project](../../functions/tests/tdd/tdd_refactor_fails_outside_project.md)
- [tdd_tests_generates_red_report](../../functions/tests/tdd/tdd_tests_generates_red_report.md)
- [tdd_tests_creates_tests_directory](../../functions/tests/tdd/tdd_tests_creates_tests_directory.md)
- [tdd_tests_report_contains_acceptance_criteria](../../functions/tests/tdd/tdd_tests_report_contains_acceptance_criteria.md)
- [tdd_tests_report_has_coverage_section](../../functions/tests/tdd/tdd_tests_report_has_coverage_section.md)
- [tdd_tests_dry_run_prints_report_without_writing](../../functions/tests/tdd/tdd_tests_dry_run_prints_report_without_writing.md)
- [tdd_refactor_dry_run_prints_report_without_writing](../../functions/tests/tdd/tdd_refactor_dry_run_prints_report_without_writing.md)
- [tdd_refactor_fails_without_red_report](../../functions/tests/tdd/tdd_refactor_fails_without_red_report.md)
- [tdd_refactor_succeeds_with_red_report](../../functions/tests/tdd/tdd_refactor_succeeds_with_red_report.md)
- [tdd_tests_is_idempotent](../../functions/tests/tdd/tdd_tests_is_idempotent.md)
- [tdd_refactor_is_idempotent](../../functions/tests/tdd/tdd_refactor_is_idempotent.md)
- [tdd_driven_schema_listed_by_pipeline_dry_run](../../functions/tests/tdd/tdd_driven_schema_listed_by_pipeline_dry_run.md)
- [pipeline_tdd_skips_tdd_tests_when_red_report_exists](../../functions/tests/tdd/pipeline_tdd_skips_tdd_tests_when_red_report_exists.md)
- [pipeline_tdd_skips_tdd_refactor_when_refactor_report_exists](../../functions/tests/tdd/pipeline_tdd_skips_tdd_refactor_when_refactor_report_exists.md)
- [init_registers_tdd_tests_command_for_claude](../../functions/tests/tdd/init_registers_tdd_tests_command_for_claude.md)
- [init_registers_tdd_refactor_command_for_claude](../../functions/tests/tdd/init_registers_tdd_refactor_command_for_claude.md)
- [tdd_tests_command_body_mentions_red_phase](../../functions/tests/tdd/tdd_tests_command_body_mentions_red_phase.md)
- [tdd_refactor_command_body_mentions_green_phase](../../functions/tests/tdd/tdd_refactor_command_body_mentions_green_phase.md)
- [upgrade_registers_tdd_tests_command](../../functions/tests/tdd/upgrade_registers_tdd_tests_command.md)
- [status_tdd_driven_shows_tdd_phases](../../functions/tests/tdd/status_tdd_driven_shows_tdd_phases.md)
- [red_report_has_interface_design_section](../../functions/tests/tdd/red_report_has_interface_design_section.md)
- [red_report_tracer_bullet_contains_first_ac](../../functions/tests/tdd/red_report_tracer_bullet_contains_first_ac.md)
- [red_report_has_test_quality_checklist](../../functions/tests/tdd/red_report_has_test_quality_checklist.md)
- [red_report_has_unexpectedly_passing_field](../../functions/tests/tdd/red_report_has_unexpectedly_passing_field.md)
- [refactor_report_has_candidates_checklist](../../functions/tests/tdd/refactor_report_has_candidates_checklist.md)
- [refactor_report_changes_table_has_type_column](../../functions/tests/tdd/refactor_report_changes_table_has_type_column.md)
- [red_report_single_ac_shows_no_remaining_cycles](../../functions/tests/tdd/red_report_single_ac_shows_no_remaining_cycles.md)
- [tdd_tests_with_explicit_feature_id](../../functions/tests/tdd/tdd_tests_with_explicit_feature_id.md)
- [tdd_refactor_with_explicit_feature_id](../../functions/tests/tdd/tdd_refactor_with_explicit_feature_id.md)
- [tdd_tests_fails_gracefully_when_feature_dir_missing](../../functions/tests/tdd/tdd_tests_fails_gracefully_when_feature_dir_missing.md)
- [red_report_uses_given_when_then_as_ac_fallback](../../functions/tests/tdd/red_report_uses_given_when_then_as_ac_fallback.md)
- [red_report_captures_criteria_after_subsection_header](../../functions/tests/tdd/red_report_captures_criteria_after_subsection_header.md)
- [red_report_graceful_when_spec_missing](../../functions/tests/tdd/red_report_graceful_when_spec_missing.md)
- [pipeline_dry_run_from_tdd_tests_skips_earlier_phases](../../functions/tests/tdd/pipeline_dry_run_from_tdd_tests_skips_earlier_phases.md)
- [pipeline_dry_run_only_tdd_tests_shows_one_phase](../../functions/tests/tdd/pipeline_dry_run_only_tdd_tests_shows_one_phase.md)
- [pipeline_force_reruns_tdd_tests_when_red_report_exists](../../functions/tests/tdd/pipeline_force_reruns_tdd_tests_when_red_report_exists.md)
- [pipeline_dry_run_shows_handoff_label_for_tdd_phases](../../functions/tests/tdd/pipeline_dry_run_shows_handoff_label_for_tdd_phases.md)
- [pipeline_tdd_phase_numbers_are_correct](../../functions/tests/tdd/pipeline_tdd_phase_numbers_are_correct.md)
- [pipeline_new_tdd_driven_scaffolds_both_reports](../../functions/tests/tdd/pipeline_new_tdd_driven_scaffolds_both_reports.md)
- [status_tdd_tests_shows_ready_when_no_artifacts](../../functions/tests/tdd/status_tdd_tests_shows_ready_when_no_artifacts.md)
- [status_tdd_tests_shows_done_when_tests_dir_nonempty_and_report_exists](../../functions/tests/tdd/status_tdd_tests_shows_done_when_tests_dir_nonempty_and_report_exists.md)
- [status_tdd_tests_not_done_when_tests_dir_is_empty](../../functions/tests/tdd/status_tdd_tests_not_done_when_tests_dir_is_empty.md)
- [status_tdd_refactor_shows_done_when_report_exists](../../functions/tests/tdd/status_tdd_refactor_shows_done_when_report_exists.md)
- [status_tdd_driven_schema_shows_correct_artifact_count](../../functions/tests/tdd/status_tdd_driven_schema_shows_correct_artifact_count.md)
- [tdd_tests_command_mentions_tracer_bullet](../../functions/tests/tdd/tdd_tests_command_mentions_tracer_bullet.md)
- [tdd_tests_command_mentions_interface_design](../../functions/tests/tdd/tdd_tests_command_mentions_interface_design.md)
- [tdd_tests_command_mentions_mock_boundaries](../../functions/tests/tdd/tdd_tests_command_mentions_mock_boundaries.md)
- [tdd_refactor_command_lists_specific_candidates](../../functions/tests/tdd/tdd_refactor_command_lists_specific_candidates.md)
- [tdd_refactor_command_warns_about_interface_growth](../../functions/tests/tdd/tdd_refactor_command_warns_about_interface_growth.md)
- [tdd_tests_resolves_to_second_feature_when_explicitly_given](../../functions/tests/tdd/tdd_tests_resolves_to_second_feature_when_explicitly_given.md)
- [full_tdd_workflow_scaffold_is_consistent](../../functions/tests/tdd/full_tdd_workflow_scaffold_is_consistent.md)
- [first_feature_dir_after_specify](../../functions/tests/tdd/first_feature_dir_after_specify.md)
- [seed_tdd_artifacts](../../functions/tests/tdd/seed_tdd_artifacts.md)

# Imports

- `predicates::prelude::*`
- `tempfile::TempDir`
- `common::{first_feature_dir, init_project, solidspec}`

# Member of

- [solidspec](../../packages/solidspec.md)