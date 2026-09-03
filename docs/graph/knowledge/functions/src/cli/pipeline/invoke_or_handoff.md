---
type: Rust Function
title: invoke_or_handoff
resource: src/cli/pipeline.rs#L482-L542
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/invoker/invoke_agent
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/pipeline/execute_phase
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn invoke_or_handoff( agent_id: &str, phase: &str, feature_dir_name: &str, project_root: &std::path::Path, description: Option<&str>, auto: bool, ) -> Result<()>`

# Calls

- [invoke_agent](../../../../functions/src/agents/invoker/invoke_agent.md)

# Called by

- [execute_phase](../../../../functions/src/cli/pipeline/execute_phase.md)