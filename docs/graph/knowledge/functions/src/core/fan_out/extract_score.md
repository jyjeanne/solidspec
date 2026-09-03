---
type: Rust Function
title: extract_score
resource: src/core/fan_out.rs#L373-L382
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/derive_score_from_keywords
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/fan_out/run_lane_with_agent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn extract_score(stdout: &str) -> u8`

# Calls

- [derive_score_from_keywords](../../../../functions/src/core/fan_out/derive_score_from_keywords.md)

# Called by

- [run_lane_with_agent](../../../../functions/src/core/fan_out/run_lane_with_agent.md)