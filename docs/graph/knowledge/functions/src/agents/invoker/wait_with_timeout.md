---
type: Rust Function
title: wait_with_timeout
resource: src/agents/invoker.rs#L297-L316
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/invoker/run_agent_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/run_agent_cli_capture
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn wait_with_timeout( mut child: std::process::Child, agent_name: &str, timeout_secs: u64, ) -> Result<std::process::ExitStatus>`

# Called by

- [run_agent_cli](../../../../functions/src/agents/invoker/run_agent_cli.md)
- [run_agent_cli_capture](../../../../functions/src/agents/invoker/run_agent_cli_capture.md)