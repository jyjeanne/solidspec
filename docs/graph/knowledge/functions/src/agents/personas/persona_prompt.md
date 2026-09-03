---
type: Rust Function
title: persona_prompt
resource: src/agents/personas.rs#L166-L179
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/personas/persona_for_phase
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/invoker/build_phase_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/personas/persona_prompts_are_nonempty
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn persona_prompt(phase: &str) -> String`

# Calls

- [persona_for_phase](../../../../functions/src/agents/personas/persona_for_phase.md)

# Called by

- [build_phase_prompt](../../../../functions/src/agents/invoker/build_phase_prompt.md)
- [persona_prompts_are_nonempty](../../../../functions/src/agents/personas/persona_prompts_are_nonempty.md)