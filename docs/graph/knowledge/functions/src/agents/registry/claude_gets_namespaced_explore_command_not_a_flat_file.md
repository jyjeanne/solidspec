---
type: Rust Function
title: claude_gets_namespaced_explore_command_not_a_flat_file
resource: src/agents/registry.rs#L1060-L1072
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
---

# Signature

`fn claude_gets_namespaced_explore_command_not_a_flat_file()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_commands](../../../../functions/src/agents/registry/register_commands.md)