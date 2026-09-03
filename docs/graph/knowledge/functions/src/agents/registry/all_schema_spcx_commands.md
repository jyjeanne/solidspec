---
type: Rust Function
title: all_schema_spcx_commands
resource: src/agents/registry.rs#L230-L246
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/schema/names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/resolve_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/register_all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn all_schema_spcx_commands(project_root: &Path) -> Result<Vec<(String, &'static str, String)>>`

# Calls

- [names](../../../../functions/src/core/schema/names.md)
- [resolve_schema](../../../../functions/src/core/schema/resolve_schema.md)
- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)

# Called by

- [register_all_schema_spcx_commands](../../../../functions/src/agents/registry/register_all_schema_spcx_commands.md)