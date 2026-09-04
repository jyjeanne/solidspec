---
type: Rust Function
title: unregister_removes_opencode_skills
resource: src/agents/registry.rs#L862-L870
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

`fn unregister_removes_opencode_skills()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_commands](../../../../functions/src/agents/registry/register_commands.md)
- [unregister_commands](../../../../functions/src/agents/registry/unregister_commands.md)