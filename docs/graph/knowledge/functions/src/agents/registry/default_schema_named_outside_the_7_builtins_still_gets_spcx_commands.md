---
type: Rust Function
title: default_schema_named_outside_the_7_builtins_still_gets_spcx_commands
resource: src/agents/registry.rs#L1195-L1215
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/config/find_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/spec_driven_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn default_schema_named_outside_the_7_builtins_still_gets_spcx_commands()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [spec_driven_schema](../../../../functions/src/agents/registry/spec_driven_schema.md)
- [register_all_schema_spcx_commands](../../../../functions/src/agents/registry/register_all_schema_spcx_commands.md)