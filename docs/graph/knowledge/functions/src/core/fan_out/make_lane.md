---
type: Rust Function
title: make_lane
resource: src/core/fan_out.rs#L705-L713
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
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
  - target: functions/src/core/fan_out/score_from_heuristics_returns_err_when_no_spec
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

`fn make_lane(id: &'static str, label: &'static str) -> ReviewLane`

# Called by

- [code_lane_placeholder_spec_is_penalized](../../../../functions/src/core/fan_out/code_lane_placeholder_spec_is_penalized.md)
- [security_lane_auth_spec_without_plan_security_section_is_penalized](../../../../functions/src/core/fan_out/security_lane_auth_spec_without_plan_security_section_is_penalized.md)
- [clean_spec_code_lane_scores_at_or_above_threshold](../../../../functions/src/core/fan_out/clean_spec_code_lane_scores_at_or_above_threshold.md)
- [missing_spec_returns_failed_status](../../../../functions/src/core/fan_out/missing_spec_returns_failed_status.md)
- [score_from_heuristics_returns_err_when_no_spec](../../../../functions/src/core/fan_out/score_from_heuristics_returns_err_when_no_spec.md)
- [tests_lane_covers_testability_only](../../../../functions/src/core/fan_out/tests_lane_covers_testability_only.md)
- [duration_ms_is_recorded](../../../../functions/src/core/fan_out/duration_ms_is_recorded.md)