---
type: Rust Function
title: tests_lane_covers_testability_only
resource: src/core/fan_out.rs#L942-L966
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/make_lane
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/run_lane_no_agent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn tests_lane_covers_testability_only()`

# Calls

- [make_lane](../../../../functions/src/core/fan_out/make_lane.md)
- [run_lane_no_agent](../../../../functions/src/core/fan_out/run_lane_no_agent.md)