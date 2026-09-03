---
type: Rust Function
title: spcx_new_body_has_guardrails_and_arguments_placeholder
resource: src/agents/registry.rs#L875-L886
visibility: private
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
  - target: functions/src/agents/registry/spec_driven_schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn spcx_new_body_has_guardrails_and_arguments_placeholder()`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [register_commands](../../../../functions/src/agents/registry/register_commands.md)
- [register_spcx_commands](../../../../functions/src/agents/registry/register_spcx_commands.md)
- [spec_driven_schema](../../../../functions/src/agents/registry/spec_driven_schema.md)