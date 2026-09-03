---
type: Rust Function
title: unregister_all_schema_spcx_commands
resource: src/agents/registry.rs#L461-L514
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/schema/names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/kimi_command_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/standard_command_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/unregister_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn unregister_all_schema_spcx_commands(project_root: &Path, agent: &AgentConfig) -> Result<()>`

# Calls

- [names](../../../../functions/src/core/schema/names.md)
- [kimi_command_name](../../../../functions/src/agents/formats/kimi_command_name.md)
- [standard_command_name](../../../../functions/src/agents/formats/standard_command_name.md)

# Called by

- [unregister_commands](../../../../functions/src/agents/registry/unregister_commands.md)