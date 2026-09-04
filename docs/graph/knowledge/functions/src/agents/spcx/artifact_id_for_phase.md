---
type: Rust Function
title: artifact_id_for_phase
resource: src/agents/spcx.rs#L63-L65
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn artifact_id_for_phase(phase: &str) -> &str`

# Called by

- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)