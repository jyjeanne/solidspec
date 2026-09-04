---
type: Rust Function
title: first_feature_dir
resource: tests/common/mod.rs#L31-L39
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/apex/create_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/apex/pipeline_apex_driven_runs_apex_when_no_finish_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/apex/pipeline_apex_driven_skips_apex_when_finish_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/init/go_and_continue_use_the_projects_stored_default_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pipeline/full_pipeline_scaffold_generates_all_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pipeline/pipeline_idsd_generates_intent_before_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pipeline/pipeline_sdd_unchanged_no_intent_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pipeline/pipeline_dry_run_respects_custom_schema_generates_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/security_first_minimal/minimal_pipeline_no_agent_scaffolds_all_four_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/security_first_minimal/minimal_status_shows_only_four_artifacts_and_no_clarify_or_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/security_first_minimal/minimal_tasks_require_only_spec_and_plan_no_security_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/security_first_minimal/security_first_status_lists_security_review_between_plan_and_tasks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/security_first_minimal/security_first_tasks_blocked_until_security_review_md_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/security_first_minimal/tasks_command_itself_blocks_without_security_review_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/security_first_minimal/tasks_without_schema_flag_defaults_to_spec_driven_and_is_unaffected
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/security_first_minimal/security_first_pipeline_no_agent_scaffolds_all_five_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/security_first_minimal/security_review_command_is_idempotent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/security_first_minimal/security_review_dry_run_prints_without_writing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/security_first_minimal/security_review_fails_without_plan_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/ship/create_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/ship/ship_lane_filter_runs_subset
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/create_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/red_report_single_ac_shows_no_remaining_cycles
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/red_report_uses_given_when_then_as_ac_fallback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/red_report_captures_criteria_after_subsection_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/red_report_graceful_when_spec_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/pipeline_new_tdd_driven_scaffolds_both_reports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/first_feature_dir_after_specify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/traceability/idsd_pipeline_scaffold_creates_all_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/traceability/sdd_pipeline_produces_no_idsd_artifacts
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn first_feature_dir(dir: &std::path::Path) -> std::path::PathBuf`

# Called by

- [create_feature](../../../functions/tests/apex/create_feature.md)
- [pipeline_apex_driven_runs_apex_when_no_finish_file](../../../functions/tests/apex/pipeline_apex_driven_runs_apex_when_no_finish_file.md)
- [pipeline_apex_driven_skips_apex_when_finish_exists](../../../functions/tests/apex/pipeline_apex_driven_skips_apex_when_finish_exists.md)
- [go_and_continue_use_the_projects_stored_default_schema](../../../functions/tests/init/go_and_continue_use_the_projects_stored_default_schema.md)
- [full_pipeline_scaffold_generates_all_artifacts](../../../functions/tests/pipeline/full_pipeline_scaffold_generates_all_artifacts.md)
- [pipeline_idsd_generates_intent_before_spec](../../../functions/tests/pipeline/pipeline_idsd_generates_intent_before_spec.md)
- [pipeline_sdd_unchanged_no_intent_md](../../../functions/tests/pipeline/pipeline_sdd_unchanged_no_intent_md.md)
- [pipeline_dry_run_respects_custom_schema_generates_override](../../../functions/tests/pipeline/pipeline_dry_run_respects_custom_schema_generates_override.md)
- [minimal_pipeline_no_agent_scaffolds_all_four_artifacts](../../../functions/tests/security_first_minimal/minimal_pipeline_no_agent_scaffolds_all_four_artifacts.md)
- [minimal_status_shows_only_four_artifacts_and_no_clarify_or_review](../../../functions/tests/security_first_minimal/minimal_status_shows_only_four_artifacts_and_no_clarify_or_review.md)
- [minimal_tasks_require_only_spec_and_plan_no_security_review](../../../functions/tests/security_first_minimal/minimal_tasks_require_only_spec_and_plan_no_security_review.md)
- [security_first_status_lists_security_review_between_plan_and_tasks](../../../functions/tests/security_first_minimal/security_first_status_lists_security_review_between_plan_and_tasks.md)
- [security_first_tasks_blocked_until_security_review_md_exists](../../../functions/tests/security_first_minimal/security_first_tasks_blocked_until_security_review_md_exists.md)
- [tasks_command_itself_blocks_without_security_review_md](../../../functions/tests/security_first_minimal/tasks_command_itself_blocks_without_security_review_md.md)
- [tasks_without_schema_flag_defaults_to_spec_driven_and_is_unaffected](../../../functions/tests/security_first_minimal/tasks_without_schema_flag_defaults_to_spec_driven_and_is_unaffected.md)
- [security_first_pipeline_no_agent_scaffolds_all_five_artifacts](../../../functions/tests/security_first_minimal/security_first_pipeline_no_agent_scaffolds_all_five_artifacts.md)
- [security_review_command_is_idempotent](../../../functions/tests/security_first_minimal/security_review_command_is_idempotent.md)
- [security_review_dry_run_prints_without_writing](../../../functions/tests/security_first_minimal/security_review_dry_run_prints_without_writing.md)
- [security_review_fails_without_plan_md](../../../functions/tests/security_first_minimal/security_review_fails_without_plan_md.md)
- [create_feature](../../../functions/tests/ship/create_feature.md)
- [ship_lane_filter_runs_subset](../../../functions/tests/ship/ship_lane_filter_runs_subset.md)
- [create_feature](../../../functions/tests/tdd/create_feature.md)
- [red_report_single_ac_shows_no_remaining_cycles](../../../functions/tests/tdd/red_report_single_ac_shows_no_remaining_cycles.md)
- [red_report_uses_given_when_then_as_ac_fallback](../../../functions/tests/tdd/red_report_uses_given_when_then_as_ac_fallback.md)
- [red_report_captures_criteria_after_subsection_header](../../../functions/tests/tdd/red_report_captures_criteria_after_subsection_header.md)
- [red_report_graceful_when_spec_missing](../../../functions/tests/tdd/red_report_graceful_when_spec_missing.md)
- [pipeline_new_tdd_driven_scaffolds_both_reports](../../../functions/tests/tdd/pipeline_new_tdd_driven_scaffolds_both_reports.md)
- [first_feature_dir_after_specify](../../../functions/tests/tdd/first_feature_dir_after_specify.md)
- [idsd_pipeline_scaffold_creates_all_artifacts](../../../functions/tests/traceability/idsd_pipeline_scaffold_creates_all_artifacts.md)
- [sdd_pipeline_produces_no_idsd_artifacts](../../../functions/tests/traceability/sdd_pipeline_produces_no_idsd_artifacts.md)