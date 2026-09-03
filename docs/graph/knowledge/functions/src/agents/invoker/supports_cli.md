---
type: Rust Function
title: supports_cli
resource: src/agents/invoker.rs#L395-L397
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/invoker/resolve_agent_cli
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/check_agent_availability
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn supports_cli(agent_id: &str) -> bool`

# Calls

- [resolve_agent_cli](../../../../functions/src/agents/invoker/resolve_agent_cli.md)

# Called by

- [run](../../../../functions/src/cli/pipeline/run.md)
- [check_agent_availability](../../../../functions/src/cli/pipeline/check_agent_availability.md)