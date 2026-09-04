---
type: Rust Function
title: other_agents_get_flat_per_schema_spcx_commands
resource: src/agents/registry.rs#L1183-L1192
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
---

# Signature

`fn other_agents_get_flat_per_schema_spcx_commands()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_all_schema_spcx_commands](../../../../functions/src/agents/registry/register_all_schema_spcx_commands.md)