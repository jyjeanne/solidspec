---
type: Rust Function
title: lane_findings_from_report
resource: src/core/fan_out.rs#L137-L155
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/lane_covers_dimension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/apply_penalty_formula
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/fan_out/score_from_heuristics
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn lane_findings_from_report( lane_id: &'static str, report: &review::ReviewReport, ) -> (u8, Vec<FanOutFinding>)`

# Calls

- [lane_covers_dimension](../../../../functions/src/core/fan_out/lane_covers_dimension.md)
- [apply_penalty_formula](../../../../functions/src/core/fan_out/apply_penalty_formula.md)

# Called by

- [score_from_heuristics](../../../../functions/src/core/fan_out/score_from_heuristics.md)