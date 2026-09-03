---
type: Rust Function
title: format_ship_report_contains_machine_readable_hold_header
resource: src/core/fan_out.rs#L1382-L1404
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/make_done_result
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/aggregate_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/report/format_ship_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn format_ship_report_contains_machine_readable_hold_header()`

# Calls

- [make_done_result](../../../../functions/src/core/fan_out/make_done_result.md)
- [aggregate_results](../../../../functions/src/core/fan_out/aggregate_results.md)
- [format_ship_report](../../../../functions/src/core/fan_out/report/format_ship_report.md)