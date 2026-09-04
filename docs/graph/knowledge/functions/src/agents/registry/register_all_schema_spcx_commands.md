---
type: Rust Function
title: register_all_schema_spcx_commands
resource: src/agents/registry.rs#L674-L681
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/registry/all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/write_commands_for_agent
    resolved_by: tree-sitter
    confidence: exact
  called_by:
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
---

# Signature

`fn register_all_schema_spcx_commands( project_root: &Path, agent: &AgentConfig, default_schema: &WorkflowSchema, ) -> Result<()>`

# Calls

- [all_schema_spcx_commands](../../../../functions/src/agents/registry/all_schema_spcx_commands.md)
- [write_commands_for_agent](../../../../functions/src/agents/registry/write_commands_for_agent.md)

# Called by

- [register_all_schema_spcx_commands_covers_every_builtin_schema](../../../../functions/src/agents/registry/register_all_schema_spcx_commands_covers_every_builtin_schema.md)
- [per_schema_spcx_bodies_actually_differ_by_schema](../../../../functions/src/agents/registry/per_schema_spcx_bodies_actually_differ_by_schema.md)
- [other_agents_get_flat_per_schema_spcx_commands](../../../../functions/src/agents/registry/other_agents_get_flat_per_schema_spcx_commands.md)
- [unregister_removes_per_schema_spcx_commands](../../../../functions/src/agents/registry/unregister_removes_per_schema_spcx_commands.md)
- [default_schema_named_outside_the_7_builtins_still_gets_spcx_commands](../../../../functions/src/agents/registry/default_schema_named_outside_the_7_builtins_still_gets_spcx_commands.md)
- [unregister_removes_custom_named_default_schema_spcx_commands_too](../../../../functions/src/agents/registry/unregister_removes_custom_named_default_schema_spcx_commands_too.md)