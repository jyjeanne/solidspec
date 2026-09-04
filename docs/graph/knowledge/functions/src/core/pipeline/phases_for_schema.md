---
type: Rust Function
title: phases_for_schema
resource: src/core/pipeline.rs#L261-L271
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_phases
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn phases_for_schema(schema: &str) -> &'static [&'static str]`

# Called by

- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)
- [filter_phases](../../../../functions/src/core/pipeline/filter_phases.md)