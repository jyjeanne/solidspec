---
type: Rust Function
title: run_lane_no_agent
resource: src/core/fan_out.rs#L115-L133
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/score_from_heuristics
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/fan_out/run_lane
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/code_lane_placeholder_spec_is_penalized
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/security_lane_auth_spec_without_plan_security_section_is_penalized
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/clean_spec_code_lane_scores_at_or_above_threshold
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/missing_spec_returns_failed_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/tests_lane_covers_testability_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/duration_ms_is_recorded
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run_lane_no_agent(lane: &ReviewLane, feature_dir: &Path, project_root: &Path) -> LaneResult`

# Calls

- [score_from_heuristics](../../../../functions/src/core/fan_out/score_from_heuristics.md)

# Called by

- [run_lane](../../../../functions/src/core/fan_out/run_lane.md)
- [code_lane_placeholder_spec_is_penalized](../../../../functions/src/core/fan_out/code_lane_placeholder_spec_is_penalized.md)
- [security_lane_auth_spec_without_plan_security_section_is_penalized](../../../../functions/src/core/fan_out/security_lane_auth_spec_without_plan_security_section_is_penalized.md)
- [clean_spec_code_lane_scores_at_or_above_threshold](../../../../functions/src/core/fan_out/clean_spec_code_lane_scores_at_or_above_threshold.md)
- [missing_spec_returns_failed_status](../../../../functions/src/core/fan_out/missing_spec_returns_failed_status.md)
- [tests_lane_covers_testability_only](../../../../functions/src/core/fan_out/tests_lane_covers_testability_only.md)
- [duration_ms_is_recorded](../../../../functions/src/core/fan_out/duration_ms_is_recorded.md)