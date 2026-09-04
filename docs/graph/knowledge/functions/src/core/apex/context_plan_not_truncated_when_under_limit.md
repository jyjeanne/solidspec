---
type: Rust Function
title: context_plan_not_truncated_when_under_limit
resource: src/core/apex.rs#L645-L653
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/apex/write_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/write_tasks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/build_solidspec_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn context_plan_not_truncated_when_under_limit()`

# Calls

- [write_plan](../../../../functions/src/core/apex/write_plan.md)
- [write_tasks](../../../../functions/src/core/apex/write_tasks.md)
- [build_solidspec_context](../../../../functions/src/core/apex/build_solidspec_context.md)