---
type: Rust Function
title: score_from_heuristics
resource: src/core/fan_out.rs#L101-L108
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/lane_findings_from_report
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/fan_out/run_lane_no_agent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn score_from_heuristics( lane: &ReviewLane, feature_dir: &Path, project_root: &Path, ) -> Result<(u8, Vec<FanOutFinding>)>`

# Calls

- [preflight_review](../../../../functions/src/core/review/preflight_review.md)
- [lane_findings_from_report](../../../../functions/src/core/fan_out/lane_findings_from_report.md)

# Called by

- [run_lane_no_agent](../../../../functions/src/core/fan_out/run_lane_no_agent.md)