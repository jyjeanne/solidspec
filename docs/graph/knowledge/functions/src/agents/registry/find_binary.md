---
type: Rust Function
title: find_binary
resource: src/agents/registry.rs#L391-L421
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/invoker/resolve_agent_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/detect_agents
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn find_binary(name: &str) -> Option<PathBuf>`

# Called by

- [resolve_agent_cli](../../../../functions/src/agents/invoker/resolve_agent_cli.md)
- [detect_agents](../../../../functions/src/agents/registry/detect_agents.md)