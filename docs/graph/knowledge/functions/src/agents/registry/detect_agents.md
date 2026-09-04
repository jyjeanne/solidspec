---
type: Rust Function
title: detect_agents
resource: src/agents/registry.rs#L133-L151
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/registry/find_binary
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/register_all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/detect_claude_when_dir_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/detect_multiple_agents
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/empty_repo_detects_nothing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn detect_agents(project_root: &Path) -> Vec<DetectedAgent>`

# Calls

- [find_binary](../../../../functions/src/agents/registry/find_binary.md)

# Called by

- [register_all](../../../../functions/src/agents/registry/register_all.md)
- [detect_claude_when_dir_exists](../../../../functions/src/agents/registry/detect_claude_when_dir_exists.md)
- [detect_multiple_agents](../../../../functions/src/agents/registry/detect_multiple_agents.md)
- [empty_repo_detects_nothing](../../../../functions/src/agents/registry/empty_repo_detects_nothing.md)