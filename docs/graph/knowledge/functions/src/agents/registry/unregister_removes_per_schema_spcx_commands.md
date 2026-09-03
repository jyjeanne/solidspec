---
type: Rust Function
title: unregister_removes_per_schema_spcx_commands
resource: src/agents/registry.rs#L1115-L1131
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/config/find_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/names
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn unregister_removes_per_schema_spcx_commands()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_commands](../../../../functions/src/agents/registry/register_commands.md)
- [register_all_schema_spcx_commands](../../../../functions/src/agents/registry/register_all_schema_spcx_commands.md)
- [unregister_commands](../../../../functions/src/agents/registry/unregister_commands.md)
- [names](../../../../functions/src/core/schema/names.md)