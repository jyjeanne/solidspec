---
type: Rust Function
title: adjust_script_paths
resource: src/agents/formats.rs#L42-L44
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/formats/adjust_script_paths_replaces
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/already_adjusted_paths_not_double_adjusted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/write_commands_for_agent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn adjust_script_paths(content: &str) -> String`

# Called by

- [adjust_script_paths_replaces](../../../../functions/src/agents/formats/adjust_script_paths_replaces.md)
- [already_adjusted_paths_not_double_adjusted](../../../../functions/src/agents/formats/already_adjusted_paths_not_double_adjusted.md)
- [write_commands_for_agent](../../../../functions/src/agents/registry/write_commands_for_agent.md)