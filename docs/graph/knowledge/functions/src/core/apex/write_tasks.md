---
type: Rust Function
title: write_tasks
resource: src/core/apex.rs#L571-L579
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
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
---

# Signature

`fn write_tasks(dir: &Path)`

# Called by

- [context_includes_fr_lines](../../../../functions/src/core/apex/context_includes_fr_lines.md)
- [context_includes_user_scenarios](../../../../functions/src/core/apex/context_includes_user_scenarios.md)
- [context_includes_pending_tasks_only](../../../../functions/src/core/apex/context_includes_pending_tasks_only.md)
- [context_task_counts_are_correct](../../../../functions/src/core/apex/context_task_counts_are_correct.md)
- [context_plan_truncated_at_60_lines](../../../../functions/src/core/apex/context_plan_truncated_at_60_lines.md)
- [context_plan_not_truncated_when_under_limit](../../../../functions/src/core/apex/context_plan_not_truncated_when_under_limit.md)
- [context_missing_spec_produces_placeholder](../../../../functions/src/core/apex/context_missing_spec_produces_placeholder.md)