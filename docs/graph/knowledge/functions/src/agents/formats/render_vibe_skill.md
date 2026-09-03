---
type: Rust Function
title: render_vibe_skill
resource: src/agents/formats.rs#L72-L89
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/formats/standard_command_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/formats/vibe_skill_has_required_frontmatter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_vibe_skill(cmd_name: &str, description: &str, body: &str) -> String`

# Calls

- [standard_command_name](../../../../functions/src/agents/formats/standard_command_name.md)

# Called by

- [vibe_skill_has_required_frontmatter](../../../../functions/src/agents/formats/vibe_skill_has_required_frontmatter.md)
- [register_commands](../../../../functions/src/agents/registry/register_commands.md)