---
type: Rust Function
title: apex_command_file_contains_apex_workflow_text
resource: src/agents/registry.rs#L1169-L1183
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

`fn apex_command_file_contains_apex_workflow_text()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_commands](../../../../functions/src/agents/registry/register_commands.md)