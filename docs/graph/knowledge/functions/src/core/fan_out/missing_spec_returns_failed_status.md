---
type: Rust Function
title: missing_spec_returns_failed_status
resource: src/core/fan_out.rs#L911-L926
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

`fn missing_spec_returns_failed_status()`

# Calls

- [make_lane](../../../../functions/src/core/fan_out/make_lane.md)
- [run_lane_no_agent](../../../../functions/src/core/fan_out/run_lane_no_agent.md)