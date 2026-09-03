---
type: Rust Function
title: apply_penalty_formula
resource: src/core/fan_out.rs#L196-L199
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/penalty_weight
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/fan_out/lane_findings_from_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn apply_penalty_formula(findings: &[FanOutFinding]) -> u8`

# Calls

- [penalty_weight](../../../../functions/src/core/fan_out/penalty_weight.md)

# Called by

- [lane_findings_from_report](../../../../functions/src/core/fan_out/lane_findings_from_report.md)