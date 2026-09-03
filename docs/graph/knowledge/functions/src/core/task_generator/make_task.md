---
type: Rust Function
title: make_task
resource: src/core/task_generator.rs#L188-L197
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/task_generator/generate_tasks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_task(counter: &mut u32, parallel: bool, story: Option<&str>, desc: &str) -> Task`

# Called by

- [generate_tasks](../../../../functions/src/core/task_generator/generate_tasks.md)