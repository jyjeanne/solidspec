---
type: Rust Function
title: execute_phase
resource: src/cli/pipeline.rs#L355-L519
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/cli/specify/run_for_existing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/invoke_or_handoff
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/refresh_knowledge_graph
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn execute_phase( phase: &str, feature_dir_name: &str, feature_dir: &std::path::Path, project_root: &std::path::Path, agent: &str, new_desc: Option<&str>, auto: bool, agent_mode: &AgentMode, schema: &str, ) -> Result<String>`

# Calls

- [run_for_existing](../../../../functions/src/cli/specify/run_for_existing.md)
- [invoke_or_handoff](../../../../functions/src/cli/pipeline/invoke_or_handoff.md)
- [refresh_knowledge_graph](../../../../functions/src/cli/pipeline/refresh_knowledge_graph.md)

# Called by

- [run](../../../../functions/src/cli/pipeline/run.md)