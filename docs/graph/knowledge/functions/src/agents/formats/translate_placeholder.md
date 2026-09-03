---
type: Rust Function
title: translate_placeholder
resource: src/agents/formats.rs#L4-L21
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/formats/translate_to_toml_replaces_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/translate_to_markdown_replaces_args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/no_double_replacement
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/write_commands_for_agent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn translate_placeholder(content: &str, target_placeholder: &str) -> String`

# Called by

- [translate_to_toml_replaces_arguments](../../../../functions/src/agents/formats/translate_to_toml_replaces_arguments.md)
- [translate_to_markdown_replaces_args](../../../../functions/src/agents/formats/translate_to_markdown_replaces_args.md)
- [no_double_replacement](../../../../functions/src/agents/formats/no_double_replacement.md)
- [write_commands_for_agent](../../../../functions/src/agents/registry/write_commands_for_agent.md)