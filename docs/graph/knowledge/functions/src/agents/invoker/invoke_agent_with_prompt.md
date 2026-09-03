---
type: Rust Function
title: invoke_agent_with_prompt
resource: src/agents/invoker.rs#L375-L392
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/invoker/resolve_agent_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/run_agent_cli_capture
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/invoker/invoke_agent_with_prompt_unknown_agent_returns_not_available
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/invoke_agent_with_prompt_no_cli_agent_returns_not_available
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/run_lane_with_agent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn invoke_agent_with_prompt( agent_id: &str, prompt: &str, project_root: &Path, timeout_secs: u64, ) -> InvokeResult`

# Calls

- [resolve_agent_cli](../../../../functions/src/agents/invoker/resolve_agent_cli.md)
- [run_agent_cli_capture](../../../../functions/src/agents/invoker/run_agent_cli_capture.md)

# Called by

- [invoke_agent_with_prompt_unknown_agent_returns_not_available](../../../../functions/src/agents/invoker/invoke_agent_with_prompt_unknown_agent_returns_not_available.md)
- [invoke_agent_with_prompt_no_cli_agent_returns_not_available](../../../../functions/src/agents/invoker/invoke_agent_with_prompt_no_cli_agent_returns_not_available.md)
- [run_lane_with_agent](../../../../functions/src/core/fan_out/run_lane_with_agent.md)