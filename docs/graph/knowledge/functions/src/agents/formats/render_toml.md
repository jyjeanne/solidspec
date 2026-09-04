---
type: Rust Function
title: render_toml
resource: src/agents/formats.rs#L29-L31
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/formats/render_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/toml_has_description_and_prompt
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_toml(description: &str, body: &str) -> String`

# Called by

- [render_command](../../../../functions/src/agents/formats/render_command.md)
- [toml_has_description_and_prompt](../../../../functions/src/agents/formats/toml_has_description_and_prompt.md)