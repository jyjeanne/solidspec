---
type: Rust Function
title: project_default_schema
resource: src/config/mod.rs#L387-L392
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/continue_cmd/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/go/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/resolved_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/schemas/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn project_default_schema(start: &Path) -> String`

# Calls

- [find_project_root](../../../functions/src/config/find_project_root.md)

# Called by

- [run](../../../functions/src/cli/continue_cmd/run.md)
- [run](../../../functions/src/cli/go/run.md)
- [resolved_schema](../../../functions/src/cli/resolved_schema.md)
- [run](../../../functions/src/cli/schemas/run.md)