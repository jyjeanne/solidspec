---
type: Rust Function
title: unregister_removes_apex_skill_directory
resource: src/agents/registry.rs#L778-L792
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
  - target: functions/src/agents/registry/register_apex_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn unregister_removes_apex_skill_directory()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_commands](../../../../functions/src/agents/registry/register_commands.md)
- [register_apex_skill](../../../../functions/src/agents/registry/register_apex_skill.md)
- [unregister_commands](../../../../functions/src/agents/registry/unregister_commands.md)