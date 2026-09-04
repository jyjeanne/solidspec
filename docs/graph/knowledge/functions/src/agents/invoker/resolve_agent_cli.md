---
type: Rust Function
title: resolve_agent_cli
resource: src/agents/invoker.rs#L193-L209
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/config/find_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/find_binary
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/invoker/invoke_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/invoke_agent_with_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/supports_cli
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_agent_cli(agent_id: &str) -> Result<(&'static AgentConfig, std::path::PathBuf), String>`

# Calls

- [find_agent](../../../../functions/src/agents/config/find_agent.md)
- [find_binary](../../../../functions/src/agents/registry/find_binary.md)

# Called by

- [invoke_agent](../../../../functions/src/agents/invoker/invoke_agent.md)
- [invoke_agent_with_prompt](../../../../functions/src/agents/invoker/invoke_agent_with_prompt.md)
- [supports_cli](../../../../functions/src/agents/invoker/supports_cli.md)