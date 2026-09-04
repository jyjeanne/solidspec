---
type: Rust Function
title: register_commands
resource: src/agents/registry.rs#L135-L147
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/registry/command_body
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/write_commands_for_agent
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
  - target: functions/src/agents/registry/claude_gets_namespaced_explore_command_not_a_flat_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/other_agents_get_flat_explore_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_claude_spcx_explore_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_per_schema_spcx_commands
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
- [write_commands_for_agent](../../../../functions/src/agents/registry/write_commands_for_agent.md)

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
- [claude_gets_namespaced_explore_command_not_a_flat_file](../../../../functions/src/agents/registry/claude_gets_namespaced_explore_command_not_a_flat_file.md)
- [other_agents_get_flat_explore_command](../../../../functions/src/agents/registry/other_agents_get_flat_explore_command.md)
- [unregister_removes_claude_spcx_explore_file](../../../../functions/src/agents/registry/unregister_removes_claude_spcx_explore_file.md)
- [unregister_removes_per_schema_spcx_commands](../../../../functions/src/agents/registry/unregister_removes_per_schema_spcx_commands.md)
- [apex_command_file_contains_apex_workflow_text](../../../../functions/src/agents/registry/apex_command_file_contains_apex_workflow_text.md)