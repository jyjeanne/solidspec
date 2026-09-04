---
type: Rust Function
title: build_lanes
resource: src/core/fan_out.rs#L342-L366
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/lane_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/lane_prompt
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/ship/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/build_lanes_creates_four_lanes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/build_lanes_uses_default_agent_when_no_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/build_lanes_uses_per_lane_agent_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/build_lanes_uses_config_thresholds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/build_lanes_prompts_contain_feature_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/build_lanes_prompts_contain_score_instruction
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_lanes( config: &FanOutConfig, feature_dir: &Path, default_agent: &str, ) -> Vec<ReviewLane>`

# Calls

- [lane_config](../../../../functions/src/core/fan_out/lane_config.md)
- [lane_prompt](../../../../functions/src/core/fan_out/lane_prompt.md)

# Called by

- [run](../../../../functions/src/cli/ship/run.md)
- [build_lanes_creates_four_lanes](../../../../functions/src/core/fan_out/build_lanes_creates_four_lanes.md)
- [build_lanes_uses_default_agent_when_no_override](../../../../functions/src/core/fan_out/build_lanes_uses_default_agent_when_no_override.md)
- [build_lanes_uses_per_lane_agent_override](../../../../functions/src/core/fan_out/build_lanes_uses_per_lane_agent_override.md)
- [build_lanes_uses_config_thresholds](../../../../functions/src/core/fan_out/build_lanes_uses_config_thresholds.md)
- [build_lanes_prompts_contain_feature_name](../../../../functions/src/core/fan_out/build_lanes_prompts_contain_feature_name.md)
- [build_lanes_prompts_contain_score_instruction](../../../../functions/src/core/fan_out/build_lanes_prompts_contain_score_instruction.md)