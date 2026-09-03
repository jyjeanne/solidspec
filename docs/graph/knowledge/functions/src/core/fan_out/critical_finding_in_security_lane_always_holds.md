---
type: Rust Function
title: critical_finding_in_security_lane_always_holds
resource: src/core/fan_out.rs#L1242-L1255
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/make_done_result
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/make_critical_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/aggregate_results
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn critical_finding_in_security_lane_always_holds()`

# Calls

- [make_done_result](../../../../functions/src/core/fan_out/make_done_result.md)
- [make_critical_finding](../../../../functions/src/core/fan_out/make_critical_finding.md)
- [aggregate_results](../../../../functions/src/core/fan_out/aggregate_results.md)