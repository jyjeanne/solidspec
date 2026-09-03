---
type: Rust Function
title: run_lane_with_agent
resource: src/core/fan_out.rs#L477-L514
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/invoker/invoke_agent_with_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/extract_score
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/parse_findings_from_output
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/fan_out/run_lane
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run_lane_with_agent( lane: &ReviewLane, project_root: &Path, timeout_secs: u64, ) -> LaneResult`

# Calls

- [invoke_agent_with_prompt](../../../../functions/src/agents/invoker/invoke_agent_with_prompt.md)
- [extract_score](../../../../functions/src/core/fan_out/extract_score.md)
- [parse_findings_from_output](../../../../functions/src/core/fan_out/parse_findings_from_output.md)

# Called by

- [run_lane](../../../../functions/src/core/fan_out/run_lane.md)