---
type: Rust Function
title: claude_gets_namespaced_spcx_commands_not_flat_files
resource: src/agents/registry.rs#L993-L1009
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
  - target: functions/src/agents/registry/register_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/spec_driven_schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn claude_gets_namespaced_spcx_commands_not_flat_files()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_commands](../../../../functions/src/agents/registry/register_commands.md)
- [register_spcx_commands](../../../../functions/src/agents/registry/register_spcx_commands.md)
- [spec_driven_schema](../../../../functions/src/agents/registry/spec_driven_schema.md)