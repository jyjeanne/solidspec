---
type: Rust Function
title: run
resource: src/cli/schemas.rs#L11-L53
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/list_available_schemas
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/project_default_schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run() -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [list_available_schemas](../../../../functions/src/core/schema/list_available_schemas.md)
- [project_default_schema](../../../../functions/src/config/project_default_schema.md)