---
type: Rust Function
title: formatted_output_has_phases_and_checkpoints
resource: src/core/task_generator.rs#L330-L338
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/task_generator/generate_tasks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/format_task_list
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn formatted_output_has_phases_and_checkpoints()`

# Calls

- [generate_tasks](../../../../functions/src/core/task_generator/generate_tasks.md)
- [format_task_list](../../../../functions/src/core/task_generator/format_task_list.md)