---
type: Rust Function
title: phase_type
resource: src/core/pipeline.rs#L197-L202
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn phase_type(phase: &str) -> PhaseType`

# Called by

- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)
- [run](../../../../functions/src/cli/pipeline/run.md)