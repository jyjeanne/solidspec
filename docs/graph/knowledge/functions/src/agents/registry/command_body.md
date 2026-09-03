---
type: Rust Function
title: command_body
resource: src/agents/registry.rs#L45-L74
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/registry/register_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/command_body_generic_fallback_for_unknown_phase
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn command_body(cmd_name: &str, project_root: &Path) -> String`

# Called by

- [register_commands](../../../../functions/src/agents/registry/register_commands.md)
- [command_body_generic_fallback_for_unknown_phase](../../../../functions/src/agents/registry/command_body_generic_fallback_for_unknown_phase.md)