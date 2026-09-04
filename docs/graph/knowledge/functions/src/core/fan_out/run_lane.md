---
type: Rust Function
title: run_lane
resource: src/core/fan_out.rs#L519-L531
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/run_lane_no_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/run_lane_with_agent
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/fan_out/run_fan_out
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run_lane( lane: ReviewLane, feature_dir: &Path, project_root: &Path, no_agent: bool, timeout_secs: u64, ) -> LaneResult`

# Calls

- [run_lane_no_agent](../../../../functions/src/core/fan_out/run_lane_no_agent.md)
- [run_lane_with_agent](../../../../functions/src/core/fan_out/run_lane_with_agent.md)

# Called by

- [run_fan_out](../../../../functions/src/core/fan_out/run_fan_out.md)