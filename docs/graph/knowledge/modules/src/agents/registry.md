---
type: Rust Module
title: registry
resource: src/agents/registry.rs#L1-L798
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-config-agents-agentconfig-find-agent
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-formats
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-guardrails
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-apex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [command_body](../../../functions/src/agents/registry/command_body.md)
- [DetectedAgent](../../../classes/src/agents/registry/DetectedAgent.md)
- [detect_agents](../../../functions/src/agents/registry/detect_agents.md)
- [register_commands](../../../functions/src/agents/registry/register_commands.md)
- [write_command_file](../../../functions/src/agents/registry/write_command_file.md)
- [apex_skill_dir](../../../functions/src/agents/registry/apex_skill_dir.md)
- [register_apex_skill](../../../functions/src/agents/registry/register_apex_skill.md)
- [unregister_apex_skill](../../../functions/src/agents/registry/unregister_apex_skill.md)
- [unregister_commands](../../../functions/src/agents/registry/unregister_commands.md)
- [register_all](../../../functions/src/agents/registry/register_all.md)
- [find_binary](../../../functions/src/agents/registry/find_binary.md)
- [detect_claude_when_dir_exists](../../../functions/src/agents/registry/detect_claude_when_dir_exists.md)
- [detect_multiple_agents](../../../functions/src/agents/registry/detect_multiple_agents.md)
- [empty_repo_detects_nothing](../../../functions/src/agents/registry/empty_repo_detects_nothing.md)
- [register_markdown_agent_creates_md_files](../../../functions/src/agents/registry/register_markdown_agent_creates_md_files.md)
- [command_body_generic_fallback_for_unknown_phase](../../../functions/src/agents/registry/command_body_generic_fallback_for_unknown_phase.md)
- [project_local_override_wins_over_embedded_command_body](../../../functions/src/agents/registry/project_local_override_wins_over_embedded_command_body.md)
- [no_override_falls_back_to_embedded_default](../../../functions/src/agents/registry/no_override_falls_back_to_embedded_default.md)
- [register_toml_agent_creates_toml_files](../../../functions/src/agents/registry/register_toml_agent_creates_toml_files.md)
- [copilot_creates_agent_md_and_prompt_md](../../../functions/src/agents/registry/copilot_creates_agent_md_and_prompt_md.md)
- [kimi_creates_directory_based_skills](../../../functions/src/agents/registry/kimi_creates_directory_based_skills.md)
- [unregister_removes_copilot_files](../../../functions/src/agents/registry/unregister_removes_copilot_files.md)
- [unregister_removes_kimi_dirs](../../../functions/src/agents/registry/unregister_removes_kimi_dirs.md)
- [register_all_with_specific_agent](../../../functions/src/agents/registry/register_all_with_specific_agent.md)
- [register_all_with_invalid_agent_returns_error](../../../functions/src/agents/registry/register_all_with_invalid_agent_returns_error.md)
- [register_all_auto_detect](../../../functions/src/agents/registry/register_all_auto_detect.md)
- [vibe_creates_directory_based_skills](../../../functions/src/agents/registry/vibe_creates_directory_based_skills.md)
- [unregister_removes_vibe_dirs](../../../functions/src/agents/registry/unregister_removes_vibe_dirs.md)
- [opencode_creates_directory_based_skills](../../../functions/src/agents/registry/opencode_creates_directory_based_skills.md)
- [unregister_removes_opencode_skills](../../../functions/src/agents/registry/unregister_removes_opencode_skills.md)
- [kimi_uses_dot_separator_others_use_hyphen](../../../functions/src/agents/registry/kimi_uses_dot_separator_others_use_hyphen.md)
- [command_files_contain_compliance_guardrails](../../../functions/src/agents/registry/command_files_contain_compliance_guardrails.md)
- [apex_skill_dir_returns_correct_paths](../../../functions/src/agents/registry/apex_skill_dir_returns_correct_paths.md)
- [register_apex_skill_creates_files_for_claude](../../../functions/src/agents/registry/register_apex_skill_creates_files_for_claude.md)
- [register_apex_skill_returns_false_for_unsupported_agent](../../../functions/src/agents/registry/register_apex_skill_returns_false_for_unsupported_agent.md)
- [unregister_apex_skill_removes_directory](../../../functions/src/agents/registry/unregister_apex_skill_removes_directory.md)
- [unregister_apex_skill_is_idempotent](../../../functions/src/agents/registry/unregister_apex_skill_is_idempotent.md)
- [register_all_also_registers_apex_skill_for_claude](../../../functions/src/agents/registry/register_all_also_registers_apex_skill_for_claude.md)
- [unregister_removes_apex_skill_directory](../../../functions/src/agents/registry/unregister_removes_apex_skill_directory.md)
- [apex_command_file_contains_apex_workflow_text](../../../functions/src/agents/registry/apex_command_file_contains_apex_workflow_text.md)

# Imports

- `std::path::{Path, PathBuf}`
- `anyhow::Result`
- `super::config::{AGENTS, AgentConfig, find_agent}`
- `super::formats`
- `super::guardrails`
- `crate::core::apex`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)