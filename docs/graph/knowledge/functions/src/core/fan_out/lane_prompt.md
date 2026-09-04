---
type: Rust Function
title: lane_prompt
resource: src/core/fan_out.rs#L297-L325
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/fan_out/build_lanes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn lane_prompt(feat: &str, spec: &LaneSpec) -> String`

# Called by

- [build_lanes](../../../../functions/src/core/fan_out/build_lanes.md)