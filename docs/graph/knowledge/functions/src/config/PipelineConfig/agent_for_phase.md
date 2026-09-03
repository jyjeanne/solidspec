---
type: Rust Method
title: agent_for_phase
resource: src/config/mod.rs#L152-L165
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/check_agent_availability
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn agent_for_phase(&self, phase: &str, default_agent: &str) -> String`

# Called by

- [run](../../../../functions/src/cli/pipeline/run.md)
- [check_agent_availability](../../../../functions/src/cli/pipeline/check_agent_availability.md)