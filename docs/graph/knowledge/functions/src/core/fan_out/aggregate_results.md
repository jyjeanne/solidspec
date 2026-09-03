---
type: Rust Function
title: aggregate_results
resource: src/core/fan_out.rs#L600-L696
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/templates/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/ship/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/all_lanes_pass_returns_ship
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/one_lane_below_threshold_returns_hold
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/critical_finding_in_security_lane_always_holds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/timed_out_lane_returns_hold
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/ignore_timeout_uses_partial_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/all_lanes_failed_returns_hold_with_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/all_lanes_timed_out_with_ignore_timeout_returns_hold
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/format_ship_report_contains_machine_readable_hold_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/format_ship_report_contains_machine_readable_ship_header
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn aggregate_results( results: Vec<LaneResult>, feature_id: &str, block_on_critical: bool, ignore_timeout: bool, ) -> ShipReport`

# Calls

- [all](../../../../functions/src/templates/all.md)

# Called by

- [run](../../../../functions/src/cli/ship/run.md)
- [all_lanes_pass_returns_ship](../../../../functions/src/core/fan_out/all_lanes_pass_returns_ship.md)
- [one_lane_below_threshold_returns_hold](../../../../functions/src/core/fan_out/one_lane_below_threshold_returns_hold.md)
- [critical_finding_in_security_lane_always_holds](../../../../functions/src/core/fan_out/critical_finding_in_security_lane_always_holds.md)
- [timed_out_lane_returns_hold](../../../../functions/src/core/fan_out/timed_out_lane_returns_hold.md)
- [ignore_timeout_uses_partial_results](../../../../functions/src/core/fan_out/ignore_timeout_uses_partial_results.md)
- [all_lanes_failed_returns_hold_with_message](../../../../functions/src/core/fan_out/all_lanes_failed_returns_hold_with_message.md)
- [all_lanes_timed_out_with_ignore_timeout_returns_hold](../../../../functions/src/core/fan_out/all_lanes_timed_out_with_ignore_timeout_returns_hold.md)
- [format_ship_report_contains_machine_readable_hold_header](../../../../functions/src/core/fan_out/format_ship_report_contains_machine_readable_hold_header.md)
- [format_ship_report_contains_machine_readable_ship_header](../../../../functions/src/core/fan_out/format_ship_report_contains_machine_readable_ship_header.md)