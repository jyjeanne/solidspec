---
type: Rust Function
title: generate_constitution
resource: src/cli/init.rs#L130-L148
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/task_generator/Task/format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/render
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn generate_constitution(project_dir: &Path, project_name: &str) -> Result<()>`

# Calls

- [format](../../../../functions/src/core/task_generator/Task/format.md)
- [render](../../../../functions/src/templates/render.md)

# Called by

- [run](../../../../functions/src/cli/init/run.md)