---
type: Rust Function
title: resolved_schema
resource: src/cli/mod.rs#L420-L425
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/project_default_schema
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolved_schema(schema: Option<String>) -> String`

# Calls

- [project_default_schema](../../../functions/src/config/project_default_schema.md)

# Called by

- [run](../../../functions/src/cli/run.md)