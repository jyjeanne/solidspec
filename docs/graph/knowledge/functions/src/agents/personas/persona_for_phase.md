---
type: Rust Function
title: persona_for_phase
resource: src/agents/personas.rs#L14-L27
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/personas/persona_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/personas/all_phases_have_personas
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/personas/unknown_phase_returns_default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/personas/review_persona_is_adversarial
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/personas/implement_persona_emphasizes_incremental
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/personas/security_review_persona_is_owasp_focused
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn persona_for_phase(phase: &str) -> &'static Persona`

# Called by

- [persona_prompt](../../../../functions/src/agents/personas/persona_prompt.md)
- [all_phases_have_personas](../../../../functions/src/agents/personas/all_phases_have_personas.md)
- [unknown_phase_returns_default](../../../../functions/src/agents/personas/unknown_phase_returns_default.md)
- [review_persona_is_adversarial](../../../../functions/src/agents/personas/review_persona_is_adversarial.md)
- [implement_persona_emphasizes_incremental](../../../../functions/src/agents/personas/implement_persona_emphasizes_incremental.md)
- [security_review_persona_is_owasp_focused](../../../../functions/src/agents/personas/security_review_persona_is_owasp_focused.md)