---
type: Rust Function
title: unregister_removes_claude_spcx_directory_contents
resource: src/agents/registry.rs#L1022-L1035
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
  - target: functions/src/agents/registry/unregister_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn unregister_removes_claude_spcx_directory_contents()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_commands](../../../../functions/src/agents/registry/register_commands.md)
- [unregister_commands](../../../../functions/src/agents/registry/unregister_commands.md)