---
type: Rust Function
title: register_commands
resource: src/agents/registry.rs#L154-L199
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/registry/command_body
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/agents/registry/register_all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_markdown_agent_creates_md_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/project_local_override_wins_over_embedded_command_body
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/no_override_falls_back_to_embedded_default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_toml_agent_creates_toml_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/copilot_creates_agent_md_and_prompt_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/kimi_creates_directory_based_skills
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_copilot_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_kimi_dirs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/opencode_creates_directory_based_skills
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_opencode_skills
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/kimi_uses_dot_separator_others_use_hyphen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/command_files_contain_compliance_guardrails
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_apex_skill_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/claude_gets_namespaced_spcx_commands_not_flat_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/spcx_new_body_has_guardrails_and_arguments_placeholder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/other_agents_get_flat_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_claude_spcx_directory_contents
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/apex_command_file_contains_apex_workflow_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn register_commands(project_root: &Path, agent: &AgentConfig) -> Result<()>`

# Calls

- [command_body](../../../../functions/src/agents/registry/command_body.md)
- [translate_placeholder](../../../../functions/src/agents/formats/translate_placeholder.md)
- [compliance_footer](../../../../functions/src/agents/guardrails/compliance_footer.md)
- [adjust_script_paths](../../../../functions/src/agents/formats/adjust_script_paths.md)
- [render_copilot_agent](../../../../functions/src/agents/formats/render_copilot_agent.md)
- [render_copilot_prompt](../../../../functions/src/agents/formats/render_copilot_prompt.md)
- [render_opencode_skill](../../../../functions/src/agents/formats/render_opencode_skill.md)
- [render_command](../../../../functions/src/agents/formats/render_command.md)
- [write_command_file](../../../../functions/src/agents/registry/write_command_file.md)

# Called by

- [register_all](../../../../functions/src/agents/registry/register_all.md)
- [register_markdown_agent_creates_md_files](../../../../functions/src/agents/registry/register_markdown_agent_creates_md_files.md)
- [project_local_override_wins_over_embedded_command_body](../../../../functions/src/agents/registry/project_local_override_wins_over_embedded_command_body.md)
- [no_override_falls_back_to_embedded_default](../../../../functions/src/agents/registry/no_override_falls_back_to_embedded_default.md)
- [register_toml_agent_creates_toml_files](../../../../functions/src/agents/registry/register_toml_agent_creates_toml_files.md)
- [copilot_creates_agent_md_and_prompt_md](../../../../functions/src/agents/registry/copilot_creates_agent_md_and_prompt_md.md)
- [kimi_creates_directory_based_skills](../../../../functions/src/agents/registry/kimi_creates_directory_based_skills.md)
- [unregister_removes_copilot_files](../../../../functions/src/agents/registry/unregister_removes_copilot_files.md)
- [unregister_removes_kimi_dirs](../../../../functions/src/agents/registry/unregister_removes_kimi_dirs.md)
- [opencode_creates_directory_based_skills](../../../../functions/src/agents/registry/opencode_creates_directory_based_skills.md)
- [unregister_removes_opencode_skills](../../../../functions/src/agents/registry/unregister_removes_opencode_skills.md)
- [kimi_uses_dot_separator_others_use_hyphen](../../../../functions/src/agents/registry/kimi_uses_dot_separator_others_use_hyphen.md)
- [command_files_contain_compliance_guardrails](../../../../functions/src/agents/registry/command_files_contain_compliance_guardrails.md)
- [unregister_removes_apex_skill_directory](../../../../functions/src/agents/registry/unregister_removes_apex_skill_directory.md)
- [claude_gets_namespaced_spcx_commands_not_flat_files](../../../../functions/src/agents/registry/claude_gets_namespaced_spcx_commands_not_flat_files.md)
- [spcx_new_body_has_guardrails_and_arguments_placeholder](../../../../functions/src/agents/registry/spcx_new_body_has_guardrails_and_arguments_placeholder.md)
- [other_agents_get_flat_spcx_commands](../../../../functions/src/agents/registry/other_agents_get_flat_spcx_commands.md)
- [unregister_removes_claude_spcx_directory_contents](../../../../functions/src/agents/registry/unregister_removes_claude_spcx_directory_contents.md)
- [apex_command_file_contains_apex_workflow_text](../../../../functions/src/agents/registry/apex_command_file_contains_apex_workflow_text.md)