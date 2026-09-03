---
type: Rust Function
title: compliance_footer
resource: src/agents/guardrails.rs#L42-L44
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/guardrails/compliance_footer_contains_both_sections
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/build_phase_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/write_commands_for_agent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn compliance_footer() -> String`

# Called by

- [compliance_footer_contains_both_sections](../../../../functions/src/agents/guardrails/compliance_footer_contains_both_sections.md)
- [build_phase_prompt](../../../../functions/src/agents/invoker/build_phase_prompt.md)
- [write_commands_for_agent](../../../../functions/src/agents/registry/write_commands_for_agent.md)