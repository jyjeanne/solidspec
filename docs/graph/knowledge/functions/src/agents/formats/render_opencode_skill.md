---
type: Rust Function
title: render_opencode_skill
resource: src/agents/formats.rs#L93-L102
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/formats/standard_command_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/formats/opencode_skill_has_name_and_description
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_opencode_skill(cmd_name: &str, description: &str, body: &str) -> String`

# Calls

- [standard_command_name](../../../../functions/src/agents/formats/standard_command_name.md)

# Called by

- [opencode_skill_has_name_and_description](../../../../functions/src/agents/formats/opencode_skill_has_name_and_description.md)
- [register_commands](../../../../functions/src/agents/registry/register_commands.md)