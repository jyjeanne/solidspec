---
type: Rust Function
title: format_ship_report
resource: src/core/fan_out/report.rs#L7-L68
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/ship/run
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

`pub fn format_ship_report(report: &ShipReport) -> String`

# Called by

- [run](../../../../../functions/src/cli/ship/run.md)
- [format_ship_report_contains_machine_readable_hold_header](../../../../../functions/src/core/fan_out/format_ship_report_contains_machine_readable_hold_header.md)
- [format_ship_report_contains_machine_readable_ship_header](../../../../../functions/src/core/fan_out/format_ship_report_contains_machine_readable_ship_header.md)