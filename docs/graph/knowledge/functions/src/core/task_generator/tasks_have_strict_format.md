---
type: Rust Function
title: tasks_have_strict_format
resource: src/core/task_generator.rs#L229-L240
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/task_generator/generate_tasks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/Task/format
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn tasks_have_strict_format()`

# Calls

- [generate_tasks](../../../../functions/src/core/task_generator/generate_tasks.md)
- [format](../../../../functions/src/core/task_generator/Task/format.md)