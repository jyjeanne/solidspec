---
type: Rust Function
title: spec_driven_schema
resource: src/agents/registry.rs#L597-L599
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/registry/register_all_with_specific_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_all_with_invalid_agent_returns_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_all_auto_detect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_all_also_registers_apex_skill_for_claude
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
  - target: functions/src/agents/registry/register_all_registers_per_schema_spcx_commands_too
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn spec_driven_schema() -> WorkflowSchema`

# Called by

- [register_all_with_specific_agent](../../../../functions/src/agents/registry/register_all_with_specific_agent.md)
- [register_all_with_invalid_agent_returns_error](../../../../functions/src/agents/registry/register_all_with_invalid_agent_returns_error.md)
- [register_all_auto_detect](../../../../functions/src/agents/registry/register_all_auto_detect.md)
- [register_all_also_registers_apex_skill_for_claude](../../../../functions/src/agents/registry/register_all_also_registers_apex_skill_for_claude.md)
- [claude_gets_namespaced_spcx_commands_not_flat_files](../../../../functions/src/agents/registry/claude_gets_namespaced_spcx_commands_not_flat_files.md)
- [spcx_new_body_has_guardrails_and_arguments_placeholder](../../../../functions/src/agents/registry/spcx_new_body_has_guardrails_and_arguments_placeholder.md)
- [other_agents_get_flat_spcx_commands](../../../../functions/src/agents/registry/other_agents_get_flat_spcx_commands.md)
- [register_all_registers_per_schema_spcx_commands_too](../../../../functions/src/agents/registry/register_all_registers_per_schema_spcx_commands_too.md)