---
type: Rust Function
title: run_agent_cli_capture
resource: src/agents/invoker.rs#L323-L368
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/invoker/build_agent_args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/wait_with_timeout
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/invoker/invoke_agent_with_prompt
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn run_agent_cli_capture( agent: &AgentConfig, binary_path: &Path, prompt: &str, working_dir: &Path, timeout_secs: u64, ) -> Result<String>`

# Calls

- [build_agent_args](../../../../functions/src/agents/invoker/build_agent_args.md)
- [wait_with_timeout](../../../../functions/src/agents/invoker/wait_with_timeout.md)

# Called by

- [invoke_agent_with_prompt](../../../../functions/src/agents/invoker/invoke_agent_with_prompt.md)