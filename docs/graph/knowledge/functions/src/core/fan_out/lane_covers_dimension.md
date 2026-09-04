---
type: Rust Function
title: lane_covers_dimension
resource: src/core/fan_out.rs#L166-L177
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/fan_out/lane_findings_from_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn lane_covers_dimension(lane_id: &str, dim: &Dimension) -> bool`

# Called by

- [lane_findings_from_report](../../../../functions/src/core/fan_out/lane_findings_from_report.md)