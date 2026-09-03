---
type: Rust Function
title: register_spcx_commands
resource: src/agents/registry.rs#L171-L190
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/write_commands_for_agent
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/register_all
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
---

# Signature

`pub fn register_spcx_commands( project_root: &Path, agent: &AgentConfig, schema: &crate::core::schema::WorkflowSchema, ) -> Result<()>`

# Calls

- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)
- [write_commands_for_agent](../../../../functions/src/agents/registry/write_commands_for_agent.md)

# Called by

- [register_all](../../../../functions/src/agents/registry/register_all.md)
- [claude_gets_namespaced_spcx_commands_not_flat_files](../../../../functions/src/agents/registry/claude_gets_namespaced_spcx_commands_not_flat_files.md)
- [spcx_new_body_has_guardrails_and_arguments_placeholder](../../../../functions/src/agents/registry/spcx_new_body_has_guardrails_and_arguments_placeholder.md)
- [other_agents_get_flat_spcx_commands](../../../../functions/src/agents/registry/other_agents_get_flat_spcx_commands.md)