---
type: Rust Module
title: apex
resource: tests/apex.rs#L1-L894
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

- [create_feature](../../functions/tests/apex/create_feature.md)
- [apex_command_is_hidden_from_top_level_help_but_still_registered](../../functions/tests/apex/apex_command_is_hidden_from_top_level_help_but_still_registered.md)
- [apex_help_shows_flags](../../functions/tests/apex/apex_help_shows_flags.md)
- [apex_fails_without_tasks_md](../../functions/tests/apex/apex_fails_without_tasks_md.md)
- [apex_fails_outside_project_root](../../functions/tests/apex/apex_fails_outside_project_root.md)
- [apex_writes_context_file](../../functions/tests/apex/apex_writes_context_file.md)
- [apex_context_file_contains_feature_id](../../functions/tests/apex/apex_context_file_contains_feature_id.md)
- [apex_context_file_contains_pending_tasks](../../functions/tests/apex/apex_context_file_contains_pending_tasks.md)
- [apex_context_only_writes_file_without_instructions](../../functions/tests/apex/apex_context_only_writes_file_without_instructions.md)
- [apex_dry_run_prints_would_write_and_creates_no_file](../../functions/tests/apex/apex_dry_run_prints_would_write_and_creates_no_file.md)
- [apex_sync_with_no_log_reports_nothing_to_sync](../../functions/tests/apex/apex_sync_with_no_log_reports_nothing_to_sync.md)
- [apex_sync_marks_completed_tasks_from_log](../../functions/tests/apex/apex_sync_marks_completed_tasks_from_log.md)
- [apex_output_shows_task_summary_and_invocation](../../functions/tests/apex/apex_output_shows_task_summary_and_invocation.md)
- [pipeline_apex_driven_dry_run_shows_apex_phase](../../functions/tests/apex/pipeline_apex_driven_dry_run_shows_apex_phase.md)
- [pipeline_apex_driven_dry_run_excludes_implement_phase](../../functions/tests/apex/pipeline_apex_driven_dry_run_excludes_implement_phase.md)
- [pipeline_apex_driven_dry_run_shows_handoff_for_apex](../../functions/tests/apex/pipeline_apex_driven_dry_run_shows_handoff_for_apex.md)
- [pipeline_intent_apex_dry_run_includes_intent_apex_evidence](../../functions/tests/apex/pipeline_intent_apex_dry_run_includes_intent_apex_evidence.md)
- [pipeline_intent_apex_dry_run_excludes_implement_phase](../../functions/tests/apex/pipeline_intent_apex_dry_run_excludes_implement_phase.md)
- [pipeline_spec_driven_dry_run_has_no_apex_phase](../../functions/tests/apex/pipeline_spec_driven_dry_run_has_no_apex_phase.md)
- [pipeline_intent_driven_dry_run_has_no_apex_phase](../../functions/tests/apex/pipeline_intent_driven_dry_run_has_no_apex_phase.md)
- [status_shows_apex_artifact_in_apex_driven_schema](../../functions/tests/apex/status_shows_apex_artifact_in_apex_driven_schema.md)
- [status_apex_driven_does_not_show_implement_artifact](../../functions/tests/apex/status_apex_driven_does_not_show_implement_artifact.md)
- [apex_context_generation_is_idempotent](../../functions/tests/apex/apex_context_generation_is_idempotent.md)
- [init_registers_apex_skill_for_claude](../../functions/tests/apex/init_registers_apex_skill_for_claude.md)
- [init_registers_apex_skill_subdirectories](../../functions/tests/apex/init_registers_apex_skill_subdirectories.md)
- [init_registers_apex_skill_for_kimi](../../functions/tests/apex/init_registers_apex_skill_for_kimi.md)
- [init_registers_apex_slash_command_for_claude](../../functions/tests/apex/init_registers_apex_slash_command_for_claude.md)
- [upgrade_refreshes_apex_skill_files](../../functions/tests/apex/upgrade_refreshes_apex_skill_files.md)
- [init_unsupported_agent_gets_slash_command_but_no_skill_dir](../../functions/tests/apex/init_unsupported_agent_gets_slash_command_but_no_skill_dir.md)
- [apex_auto_detects_feature_id](../../functions/tests/apex/apex_auto_detects_feature_id.md)
- [apex_context_includes_fr_requirements_from_spec](../../functions/tests/apex/apex_context_includes_fr_requirements_from_spec.md)
- [pipeline_apex_driven_only_apex_dry_run](../../functions/tests/apex/pipeline_apex_driven_only_apex_dry_run.md)
- [pipeline_apex_driven_from_tasks_to_analyze_dry_run](../../functions/tests/apex/pipeline_apex_driven_from_tasks_to_analyze_dry_run.md)
- [pipeline_intent_apex_dry_run_shows_ten_phases](../../functions/tests/apex/pipeline_intent_apex_dry_run_shows_ten_phases.md)
- [pipeline_apex_driven_runs_apex_when_no_finish_file](../../functions/tests/apex/pipeline_apex_driven_runs_apex_when_no_finish_file.md)
- [pipeline_apex_driven_skips_apex_when_finish_exists](../../functions/tests/apex/pipeline_apex_driven_skips_apex_when_finish_exists.md)

# Imports

- `predicates::prelude::*`
- `tempfile::TempDir`
- `common::{first_feature_dir, init_project, solidspec}`

# Member of

- [solidspec](../../packages/solidspec.md)