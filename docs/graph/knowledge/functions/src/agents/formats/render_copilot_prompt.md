---
type: Rust Function
title: render_copilot_prompt
resource: src/agents/formats.rs#L60-L69
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/registry/register_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_copilot_prompt(description: &str, body: &str) -> String`

# Called by

- [register_commands](../../../../functions/src/agents/registry/register_commands.md)