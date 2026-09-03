---
type: Rust Function
title: run_fan_out
resource: src/core/fan_out.rs#L538-L564
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/run_lane
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/print_lane_result
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/ship/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run_fan_out( lanes: Vec<ReviewLane>, feature_dir: PathBuf, project_root: PathBuf, no_agent: bool, timeout_secs: u64, ) -> Vec<LaneResult>`

# Calls

- [run_lane](../../../../functions/src/core/fan_out/run_lane.md)
- [print_lane_result](../../../../functions/src/core/fan_out/print_lane_result.md)

# Called by

- [run](../../../../functions/src/cli/ship/run.md)