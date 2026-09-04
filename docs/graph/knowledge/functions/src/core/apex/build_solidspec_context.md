---
type: Rust Function
title: build_solidspec_context
resource: src/core/apex.rs#L158-L249
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/apex/is_pending_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/is_completed_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/text/truncate_at_boundary
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/apex/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/context_includes_fr_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/context_includes_user_scenarios
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/context_includes_pending_tasks_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/context_task_counts_are_correct
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/context_plan_truncated_at_60_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/context_plan_not_truncated_when_under_limit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/context_missing_spec_produces_placeholder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/context_missing_all_files_produces_placeholders
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/context_truncation_does_not_panic_on_multibyte_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/context_counts_uppercase_checked_tasks_as_done
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/context_under_16kb_for_typical_feature
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_solidspec_context(feature_dir: &Path, feature_id: &str) -> Result<String>`

# Calls

- [is_pending_task](../../../../functions/src/core/apex/is_pending_task.md)
- [is_completed_task](../../../../functions/src/core/apex/is_completed_task.md)
- [truncate_at_boundary](../../../../functions/src/core/text/truncate_at_boundary.md)

# Called by

- [run](../../../../functions/src/cli/apex/run.md)
- [context_includes_fr_lines](../../../../functions/src/core/apex/context_includes_fr_lines.md)
- [context_includes_user_scenarios](../../../../functions/src/core/apex/context_includes_user_scenarios.md)
- [context_includes_pending_tasks_only](../../../../functions/src/core/apex/context_includes_pending_tasks_only.md)
- [context_task_counts_are_correct](../../../../functions/src/core/apex/context_task_counts_are_correct.md)
- [context_plan_truncated_at_60_lines](../../../../functions/src/core/apex/context_plan_truncated_at_60_lines.md)
- [context_plan_not_truncated_when_under_limit](../../../../functions/src/core/apex/context_plan_not_truncated_when_under_limit.md)
- [context_missing_spec_produces_placeholder](../../../../functions/src/core/apex/context_missing_spec_produces_placeholder.md)
- [context_missing_all_files_produces_placeholders](../../../../functions/src/core/apex/context_missing_all_files_produces_placeholders.md)
- [context_truncation_does_not_panic_on_multibyte_content](../../../../functions/src/core/apex/context_truncation_does_not_panic_on_multibyte_content.md)
- [context_counts_uppercase_checked_tasks_as_done](../../../../functions/src/core/apex/context_counts_uppercase_checked_tasks_as_done.md)
- [context_under_16kb_for_typical_feature](../../../../functions/src/core/apex/context_under_16kb_for_typical_feature.md)