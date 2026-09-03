---
type: Rust Function
title: standard_command_name
resource: src/agents/formats.rs#L110-L112
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/formats/render_vibe_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/render_opencode_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/write_command_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn standard_command_name(cmd: &str) -> String`

# Called by

- [render_vibe_skill](../../../../functions/src/agents/formats/render_vibe_skill.md)
- [render_opencode_skill](../../../../functions/src/agents/formats/render_opencode_skill.md)
- [write_command_file](../../../../functions/src/agents/registry/write_command_file.md)
- [unregister_commands](../../../../functions/src/agents/registry/unregister_commands.md)