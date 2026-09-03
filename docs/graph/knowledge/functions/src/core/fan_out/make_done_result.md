---
type: Rust Function
title: make_done_result
resource: src/core/fan_out.rs#L1191-L1207
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/fan_out/critical_finding_in_security_lane_always_holds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/format_ship_report_contains_machine_readable_hold_header
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_done_result( lane_id: &'static str, label: &'static str, score: u8, threshold: u8, ) -> LaneResult`

# Called by

- [critical_finding_in_security_lane_always_holds](../../../../functions/src/core/fan_out/critical_finding_in_security_lane_always_holds.md)
- [format_ship_report_contains_machine_readable_hold_header](../../../../functions/src/core/fan_out/format_ship_report_contains_machine_readable_hold_header.md)