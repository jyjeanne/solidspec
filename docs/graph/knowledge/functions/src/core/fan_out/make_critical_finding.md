---
type: Rust Function
title: make_critical_finding
resource: src/core/fan_out.rs#L1209-L1216
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/fan_out/critical_finding_in_security_lane_always_holds
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_critical_finding(lane: &'static str) -> FanOutFinding`

# Called by

- [critical_finding_in_security_lane_always_holds](../../../../functions/src/core/fan_out/critical_finding_in_security_lane_always_holds.md)