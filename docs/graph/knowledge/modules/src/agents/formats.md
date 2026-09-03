---
type: Rust Module
title: formats
resource: src/agents/formats.rs#L1-L211
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-config-agentformat
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [translate_placeholder](../../../functions/src/agents/formats/translate_placeholder.md)
- [render_markdown](../../../functions/src/agents/formats/render_markdown.md)
- [render_toml](../../../functions/src/agents/formats/render_toml.md)
- [render_command](../../../functions/src/agents/formats/render_command.md)
- [adjust_script_paths](../../../functions/src/agents/formats/adjust_script_paths.md)
- [render_copilot_agent](../../../functions/src/agents/formats/render_copilot_agent.md)
- [render_copilot_prompt](../../../functions/src/agents/formats/render_copilot_prompt.md)
- [render_vibe_skill](../../../functions/src/agents/formats/render_vibe_skill.md)
- [render_opencode_skill](../../../functions/src/agents/formats/render_opencode_skill.md)
- [kimi_command_name](../../../functions/src/agents/formats/kimi_command_name.md)
- [standard_command_name](../../../functions/src/agents/formats/standard_command_name.md)
- [translate_to_toml_replaces_arguments](../../../functions/src/agents/formats/translate_to_toml_replaces_arguments.md)
- [translate_to_markdown_replaces_args](../../../functions/src/agents/formats/translate_to_markdown_replaces_args.md)
- [no_double_replacement](../../../functions/src/agents/formats/no_double_replacement.md)
- [markdown_has_frontmatter_delimiters](../../../functions/src/agents/formats/markdown_has_frontmatter_delimiters.md)
- [toml_has_description_and_prompt](../../../functions/src/agents/formats/toml_has_description_and_prompt.md)
- [adjust_script_paths_replaces](../../../functions/src/agents/formats/adjust_script_paths_replaces.md)
- [already_adjusted_paths_not_double_adjusted](../../../functions/src/agents/formats/already_adjusted_paths_not_double_adjusted.md)
- [vibe_skill_has_required_frontmatter](../../../functions/src/agents/formats/vibe_skill_has_required_frontmatter.md)
- [opencode_skill_has_name_and_description](../../../functions/src/agents/formats/opencode_skill_has_name_and_description.md)
- [kimi_dot_separator_naming](../../../functions/src/agents/formats/kimi_dot_separator_naming.md)
- [standard_hyphen_separator_naming](../../../functions/src/agents/formats/standard_hyphen_separator_naming.md)

# Imports

- `super::config::AgentFormat`
- `super::*`

# Member of

- [solidspec](../../../packages/solidspec.md)