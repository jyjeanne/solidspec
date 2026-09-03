---
type: Rust Function
title: render_copilot_prompt
resource: src/agents/formats.rs#L60-L69
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/registry/write_commands_for_agent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_copilot_prompt(description: &str, body: &str) -> String`

# Called by

- [write_commands_for_agent](../../../../functions/src/agents/registry/write_commands_for_agent.md)