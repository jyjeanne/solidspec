---
type: Rust Function
title: find_agent
resource: src/agents/config.rs#L303-L310
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/config/cli_agents_have_requires_cli_true
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/config/ide_agents_have_requires_cli_false
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/config/copilot_uses_agent_md_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/config/kimi_uses_skill_md_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/resolve_agent_cli
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/agents/registry/register_all_schema_spcx_commands_covers_every_builtin_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/per_schema_spcx_bodies_actually_differ_by_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/other_agents_get_flat_per_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_per_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/default_schema_named_outside_the_7_builtins_still_gets_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_custom_named_default_schema_spcx_commands_too
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/apex_command_file_contains_apex_workflow_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn find_agent(id: &str) -> Option<&'static AgentConfig>`

# Called by

- [cli_agents_have_requires_cli_true](../../../../functions/src/agents/config/cli_agents_have_requires_cli_true.md)
- [ide_agents_have_requires_cli_false](../../../../functions/src/agents/config/ide_agents_have_requires_cli_false.md)
- [copilot_uses_agent_md_extension](../../../../functions/src/agents/config/copilot_uses_agent_md_extension.md)
- [kimi_uses_skill_md_extension](../../../../functions/src/agents/config/kimi_uses_skill_md_extension.md)
- [resolve_agent_cli](../../../../functions/src/agents/invoker/resolve_agent_cli.md)
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
- [register_all_schema_spcx_commands_covers_every_builtin_schema](../../../../functions/src/agents/registry/register_all_schema_spcx_commands_covers_every_builtin_schema.md)
- [per_schema_spcx_bodies_actually_differ_by_schema](../../../../functions/src/agents/registry/per_schema_spcx_bodies_actually_differ_by_schema.md)
- [other_agents_get_flat_per_schema_spcx_commands](../../../../functions/src/agents/registry/other_agents_get_flat_per_schema_spcx_commands.md)
- [unregister_removes_per_schema_spcx_commands](../../../../functions/src/agents/registry/unregister_removes_per_schema_spcx_commands.md)
- [default_schema_named_outside_the_7_builtins_still_gets_spcx_commands](../../../../functions/src/agents/registry/default_schema_named_outside_the_7_builtins_still_gets_spcx_commands.md)
- [unregister_removes_custom_named_default_schema_spcx_commands_too](../../../../functions/src/agents/registry/unregister_removes_custom_named_default_schema_spcx_commands_too.md)
- [apex_command_file_contains_apex_workflow_text](../../../../functions/src/agents/registry/apex_command_file_contains_apex_workflow_text.md)