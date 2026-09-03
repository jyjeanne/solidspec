---
type: Rust Function
title: write_command_file
resource: src/agents/registry.rs#L205-L250
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/formats/kimi_command_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/standard_command_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/register_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_command_file( project_root: &Path, agent: &AgentConfig, cmd_name: &str, content: &str, ) -> Result<()>`

# Calls

- [kimi_command_name](../../../../functions/src/agents/formats/kimi_command_name.md)
- [standard_command_name](../../../../functions/src/agents/formats/standard_command_name.md)

# Called by

- [register_commands](../../../../functions/src/agents/registry/register_commands.md)