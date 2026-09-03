---
type: Rust Function
title: per_schema_spcx_bodies_actually_differ_by_schema
resource: src/agents/registry.rs#L1056-L1082
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

`fn per_schema_spcx_bodies_actually_differ_by_schema()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_all_schema_spcx_commands](../../../../functions/src/agents/registry/register_all_schema_spcx_commands.md)