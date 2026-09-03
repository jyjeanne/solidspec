---
type: Rust Function
title: format_ship_report_contains_machine_readable_ship_header
resource: src/core/fan_out.rs#L1407-L1413
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/aggregate_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/report/format_ship_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn format_ship_report_contains_machine_readable_ship_header()`

# Calls

- [aggregate_results](../../../../functions/src/core/fan_out/aggregate_results.md)
- [format_ship_report](../../../../functions/src/core/fan_out/report/format_ship_report.md)