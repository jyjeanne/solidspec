---
type: Rust Function
title: format_task_list
resource: src/core/task_generator.rs#L155-L186
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/tasks/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/formatted_output_has_phases_and_checkpoints
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_task_list(tasks: &TaskList, feature_name: &str, branch_name: &str) -> String`

# Called by

- [run](../../../../functions/src/cli/tasks/run.md)
- [formatted_output_has_phases_and_checkpoints](../../../../functions/src/core/task_generator/formatted_output_has_phases_and_checkpoints.md)