---
type: Rust Function
title: write_commands_for_agent
resource: src/agents/registry.rs#L255-L303
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/formats/translate_placeholder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/guardrails/compliance_footer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/adjust_script_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/render_copilot_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/render_copilot_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/render_opencode_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/render_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/write_command_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/register_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_commands_for_agent( project_root: &Path, agent: &AgentConfig, commands: &[(String, &str, String)], ) -> Result<()>`

# Calls

- [translate_placeholder](../../../../functions/src/agents/formats/translate_placeholder.md)
- [compliance_footer](../../../../functions/src/agents/guardrails/compliance_footer.md)
- [adjust_script_paths](../../../../functions/src/agents/formats/adjust_script_paths.md)
- [render_copilot_agent](../../../../functions/src/agents/formats/render_copilot_agent.md)
- [render_copilot_prompt](../../../../functions/src/agents/formats/render_copilot_prompt.md)
- [render_opencode_skill](../../../../functions/src/agents/formats/render_opencode_skill.md)
- [render_command](../../../../functions/src/agents/formats/render_command.md)
- [write_command_file](../../../../functions/src/agents/registry/write_command_file.md)

# Called by

- [register_commands](../../../../functions/src/agents/registry/register_commands.md)
- [register_spcx_commands](../../../../functions/src/agents/registry/register_spcx_commands.md)
- [register_all_schema_spcx_commands](../../../../functions/src/agents/registry/register_all_schema_spcx_commands.md)