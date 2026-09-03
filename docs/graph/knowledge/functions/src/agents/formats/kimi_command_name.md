---
type: Rust Function
title: kimi_command_name
resource: src/agents/formats.rs#L105-L107
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/registry/write_command_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn kimi_command_name(cmd: &str) -> String`

# Called by

- [write_command_file](../../../../functions/src/agents/registry/write_command_file.md)
- [unregister_commands](../../../../functions/src/agents/registry/unregister_commands.md)