---
type: Rust Function
title: render_apply
resource: src/agents/spcx.rs#L189-L230
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

`fn render_apply(short: &str, phases: &[&ArtifactNode], nothing_after: bool) -> String`

# Called by

- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)