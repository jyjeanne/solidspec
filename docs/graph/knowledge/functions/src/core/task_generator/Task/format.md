---
type: Rust Method
title: format
resource: src/core/task_generator.rs#L24-L32
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/checklist/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/clarify/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/generate_constitution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/generate_agent_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/intent/build_template_vars
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/build_template_vars
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/tasks_have_strict_format
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format(&self) -> String`

# Called by

- [run](../../../../../functions/src/cli/checklist/run.md)
- [run](../../../../../functions/src/cli/clarify/run.md)
- [generate_constitution](../../../../../functions/src/cli/init/generate_constitution.md)
- [generate_agent_file](../../../../../functions/src/cli/init/generate_agent_file.md)
- [build_template_vars](../../../../../functions/src/cli/intent/build_template_vars.md)
- [run](../../../../../functions/src/cli/plan/run.md)
- [build_template_vars](../../../../../functions/src/cli/specify/build_template_vars.md)
- [tasks_have_strict_format](../../../../../functions/src/core/task_generator/tasks_have_strict_format.md)