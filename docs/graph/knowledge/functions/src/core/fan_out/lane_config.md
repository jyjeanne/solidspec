---
type: Rust Function
title: lane_config
resource: src/core/fan_out.rs#L328-L336
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

`fn lane_config<'a>(config: &'a FanOutConfig, lane_id: &str) -> (Option<&'a str>, u8)`

# Called by

- [build_lanes](../../../../functions/src/core/fan_out/build_lanes.md)