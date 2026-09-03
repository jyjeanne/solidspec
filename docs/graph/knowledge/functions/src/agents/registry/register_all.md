---
type: Rust Function
title: register_all
resource: src/agents/registry.rs#L401-L438
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/config/find_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_apex_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/detect_agents
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/upgrade/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn register_all( project_root: &Path, target_agent: Option<&str>, schema: &crate::core::schema::WorkflowSchema, ) -> Result<Vec<String>>`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_commands](../../../../functions/src/agents/registry/register_commands.md)
- [register_spcx_commands](../../../../functions/src/agents/registry/register_spcx_commands.md)
- [register_apex_skill](../../../../functions/src/agents/registry/register_apex_skill.md)
- [detect_agents](../../../../functions/src/agents/registry/detect_agents.md)

# Called by

- [register_all_with_specific_agent](../../../../functions/src/agents/registry/register_all_with_specific_agent.md)
- [register_all_with_invalid_agent_returns_error](../../../../functions/src/agents/registry/register_all_with_invalid_agent_returns_error.md)
- [register_all_auto_detect](../../../../functions/src/agents/registry/register_all_auto_detect.md)
- [register_all_also_registers_apex_skill_for_claude](../../../../functions/src/agents/registry/register_all_also_registers_apex_skill_for_claude.md)
- [run](../../../../functions/src/cli/init/run.md)
- [run](../../../../functions/src/cli/upgrade/run.md)