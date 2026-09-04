---
type: Rust Function
title: run
resource: src/cli/continue_cmd.rs#L14-L20
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/project_default_schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_id: Option<&str>, no_agent: bool) -> Result<()>`

# Calls

- [project_default_schema](../../../../functions/src/config/project_default_schema.md)