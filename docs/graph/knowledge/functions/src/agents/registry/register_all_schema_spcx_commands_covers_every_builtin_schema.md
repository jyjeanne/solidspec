---
type: Rust Function
title: register_all_schema_spcx_commands_covers_every_builtin_schema
resource: src/agents/registry.rs#L1040-L1053
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/config/find_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/names
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn register_all_schema_spcx_commands_covers_every_builtin_schema()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_all_schema_spcx_commands](../../../../functions/src/agents/registry/register_all_schema_spcx_commands.md)
- [names](../../../../functions/src/core/schema/names.md)