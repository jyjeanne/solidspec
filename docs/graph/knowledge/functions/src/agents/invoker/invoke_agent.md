---
type: Rust Function
title: invoke_agent
resource: src/agents/invoker.rs#L215-L236
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/invoker/resolve_agent_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/build_phase_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/run_agent_cli
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/invoker/invoke_unknown_agent_returns_not_available
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/invoke_no_cli_agent_returns_not_available
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/invoke_or_handoff
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn invoke_agent( agent_id: &str, phase: &str, feature_dir_name: &str, project_root: &Path, description: Option<&str>, project_context: Option<&str>, ) -> InvokeResult`

# Calls

- [resolve_agent_cli](../../../../functions/src/agents/invoker/resolve_agent_cli.md)
- [build_phase_prompt](../../../../functions/src/agents/invoker/build_phase_prompt.md)
- [run_agent_cli](../../../../functions/src/agents/invoker/run_agent_cli.md)

# Called by

- [invoke_unknown_agent_returns_not_available](../../../../functions/src/agents/invoker/invoke_unknown_agent_returns_not_available.md)
- [invoke_no_cli_agent_returns_not_available](../../../../functions/src/agents/invoker/invoke_no_cli_agent_returns_not_available.md)
- [invoke_or_handoff](../../../../functions/src/cli/pipeline/invoke_or_handoff.md)