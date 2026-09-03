---
type: Rust Function
title: node
resource: src/core/pipeline.rs#L224-L232
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/pipeline/points_at_ship_directly_instead_of_continue
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/points_at_continue_for_a_regular_phase
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn node(id: &str) -> ArtifactNode`

# Called by

- [points_at_ship_directly_instead_of_continue](../../../../functions/src/core/pipeline/points_at_ship_directly_instead_of_continue.md)
- [points_at_continue_for_a_regular_phase](../../../../functions/src/core/pipeline/points_at_continue_for_a_regular_phase.md)