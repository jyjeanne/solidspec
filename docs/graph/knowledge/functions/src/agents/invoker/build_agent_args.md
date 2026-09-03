---
type: Rust Function
title: build_agent_args
resource: src/agents/invoker.rs#L275-L293
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

`fn build_agent_args(cmd: &mut Command, agent: &AgentConfig, prompt: &str)`

# Called by

- [run_agent_cli](../../../../functions/src/agents/invoker/run_agent_cli.md)
- [run_agent_cli_capture](../../../../functions/src/agents/invoker/run_agent_cli_capture.md)