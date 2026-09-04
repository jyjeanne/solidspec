---
type: Rust Function
title: run
resource: src/cli/go.rs#L14-L29
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/project_default_schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(description: &str, no_agent: bool) -> Result<()>`

# Calls

- [project_default_schema](../../../../functions/src/config/project_default_schema.md)