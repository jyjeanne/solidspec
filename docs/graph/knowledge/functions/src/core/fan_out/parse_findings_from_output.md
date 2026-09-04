---
type: Rust Function
title: parse_findings_from_output
resource: src/core/fan_out.rs#L405-L458
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/parse_severity
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/fan_out/run_lane_with_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/parse_findings_extracts_two_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/parse_findings_problem_without_fix_still_captured
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn parse_findings_from_output( output: &str, lane_id: &'static str, ) -> Vec<FanOutFinding>`

# Calls

- [parse_severity](../../../../functions/src/core/fan_out/parse_severity.md)

# Called by

- [run_lane_with_agent](../../../../functions/src/core/fan_out/run_lane_with_agent.md)
- [parse_findings_extracts_two_findings](../../../../functions/src/core/fan_out/parse_findings_extracts_two_findings.md)
- [parse_findings_problem_without_fix_still_captured](../../../../functions/src/core/fan_out/parse_findings_problem_without_fix_still_captured.md)