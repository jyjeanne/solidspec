---
type: Rust Module
title: apex
resource: src/core/apex.rs#L1-L880
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-os-unix-fs-permissionsext
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

- [extract_skill](../../../functions/src/core/apex/extract_skill.md)
- [write_script](../../../functions/src/core/apex/write_script.md)
- [build_solidspec_context](../../../functions/src/core/apex/build_solidspec_context.md)
- [extract_section](../../../functions/src/core/apex/extract_section.md)
- [is_pending_task](../../../functions/src/core/apex/is_pending_task.md)
- [is_completed_task](../../../functions/src/core/apex/is_completed_task.md)
- [SyncReport](../../../classes/src/core/apex/SyncReport.md)
- [sync_tasks_from_apex_log](../../../functions/src/core/apex/sync_tasks_from_apex_log.md)
- [extract_completed_task_ids](../../../functions/src/core/apex/extract_completed_task_ids.md)
- [find_task_id_after_completion_marker](../../../functions/src/core/apex/find_task_id_after_completion_marker.md)
- [task_id_from_pending](../../../functions/src/core/apex/task_id_from_pending.md)
- [find_latest_execute_log](../../../functions/src/core/apex/find_latest_execute_log.md)
- [extract_skill_creates_all_directories](../../../functions/src/core/apex/extract_skill_creates_all_directories.md)
- [extract_skill_writes_all_step_files](../../../functions/src/core/apex/extract_skill_writes_all_step_files.md)
- [extract_skill_writes_all_template_files](../../../functions/src/core/apex/extract_skill_writes_all_template_files.md)
- [extract_skill_writes_scripts](../../../functions/src/core/apex/extract_skill_writes_scripts.md)
- [extract_skill_is_idempotent](../../../functions/src/core/apex/extract_skill_is_idempotent.md)
- [skill_md_content_is_nonempty](../../../functions/src/core/apex/skill_md_content_is_nonempty.md)
- [write_spec](../../../functions/src/core/apex/write_spec.md)
- [write_plan](../../../functions/src/core/apex/write_plan.md)
- [write_tasks](../../../functions/src/core/apex/write_tasks.md)
- [context_includes_fr_lines](../../../functions/src/core/apex/context_includes_fr_lines.md)
- [context_includes_user_scenarios](../../../functions/src/core/apex/context_includes_user_scenarios.md)
- [context_includes_pending_tasks_only](../../../functions/src/core/apex/context_includes_pending_tasks_only.md)
- [context_task_counts_are_correct](../../../functions/src/core/apex/context_task_counts_are_correct.md)
- [context_plan_truncated_at_60_lines](../../../functions/src/core/apex/context_plan_truncated_at_60_lines.md)
- [context_plan_not_truncated_when_under_limit](../../../functions/src/core/apex/context_plan_not_truncated_when_under_limit.md)
- [context_missing_spec_produces_placeholder](../../../functions/src/core/apex/context_missing_spec_produces_placeholder.md)
- [context_missing_all_files_produces_placeholders](../../../functions/src/core/apex/context_missing_all_files_produces_placeholders.md)
- [context_truncation_does_not_panic_on_multibyte_content](../../../functions/src/core/apex/context_truncation_does_not_panic_on_multibyte_content.md)
- [context_counts_uppercase_checked_tasks_as_done](../../../functions/src/core/apex/context_counts_uppercase_checked_tasks_as_done.md)
- [sync_marks_uppercase_checkbox_pattern](../../../functions/src/core/apex/sync_marks_uppercase_checkbox_pattern.md)
- [context_under_16kb_for_typical_feature](../../../functions/src/core/apex/context_under_16kb_for_typical_feature.md)
- [make_tasks_md](../../../functions/src/core/apex/make_tasks_md.md)
- [make_execute_log](../../../functions/src/core/apex/make_execute_log.md)
- [read_tasks](../../../functions/src/core/apex/read_tasks.md)
- [sync_marks_checkmark_pattern](../../../functions/src/core/apex/sync_marks_checkmark_pattern.md)
- [sync_marks_checkbox_pattern](../../../functions/src/core/apex/sync_marks_checkbox_pattern.md)
- [sync_leaves_unlisted_tasks_unchanged](../../../functions/src/core/apex/sync_leaves_unlisted_tasks_unchanged.md)
- [sync_is_idempotent](../../../functions/src/core/apex/sync_is_idempotent.md)
- [sync_preserves_trailing_newline](../../../functions/src/core/apex/sync_preserves_trailing_newline.md)
- [sync_empty_log_returns_zero_counts](../../../functions/src/core/apex/sync_empty_log_returns_zero_counts.md)
- [find_latest_execute_log_returns_none_when_empty](../../../functions/src/core/apex/find_latest_execute_log_returns_none_when_empty.md)
- [find_latest_execute_log_finds_log_in_subdir](../../../functions/src/core/apex/find_latest_execute_log_finds_log_in_subdir.md)

# Imports

- `std::path::{Path, PathBuf}`
- `anyhow::{Context, Result}`
- `std::os::unix::fs::PermissionsExt`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)