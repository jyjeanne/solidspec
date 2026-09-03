---
type: Rust Function
title: render_command
resource: src/agents/formats.rs#L34-L39
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/formats/render_markdown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/render_toml
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/register_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_command(format: AgentFormat, description: &str, body: &str) -> String`

# Calls

- [render_markdown](../../../../functions/src/agents/formats/render_markdown.md)
- [render_toml](../../../../functions/src/agents/formats/render_toml.md)

# Called by

- [register_commands](../../../../functions/src/agents/registry/register_commands.md)