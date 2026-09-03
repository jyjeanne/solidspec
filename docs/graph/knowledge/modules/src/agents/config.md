---
type: Rust Module
title: config
resource: src/agents/config.rs#L1-L459
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashset
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [AgentConfig](../../../classes/src/agents/config/AgentConfig.md)
- [AgentFormat](../../../classes/src/agents/config/AgentFormat.md)
- [find_agent](../../../functions/src/agents/config/find_agent.md)
- [config_table_has_20_agents](../../../functions/src/agents/config/config_table_has_20_agents.md)
- [no_duplicate_ids](../../../functions/src/agents/config/no_duplicate_ids.md)
- [no_duplicate_aliases](../../../functions/src/agents/config/no_duplicate_aliases.md)
- [all_agents_have_nonempty_command_dir_and_format](../../../functions/src/agents/config/all_agents_have_nonempty_command_dir_and_format.md)
- [cli_agents_have_requires_cli_true](../../../functions/src/agents/config/cli_agents_have_requires_cli_true.md)
- [ide_agents_have_requires_cli_false](../../../functions/src/agents/config/ide_agents_have_requires_cli_false.md)
- [alias_resolution_works](../../../functions/src/agents/config/alias_resolution_works.md)
- [toml_agents_use_args_placeholder](../../../functions/src/agents/config/toml_agents_use_args_placeholder.md)
- [copilot_uses_agent_md_extension](../../../functions/src/agents/config/copilot_uses_agent_md_extension.md)
- [kimi_uses_skill_md_extension](../../../functions/src/agents/config/kimi_uses_skill_md_extension.md)
- [alias_bijection](../../../functions/src/agents/config/alias_bijection.md)

# Imports

- `super::*`
- `std::collections::HashSet`

# Member of

- [solidspec](../../../packages/solidspec.md)