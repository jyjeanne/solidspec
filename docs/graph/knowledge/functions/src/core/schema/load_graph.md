---
type: Rust Function
title: load_graph
resource: src/core/schema.rs#L216-L222
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/schema/resolve_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/WorkflowSchema/into_graph
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/status/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tasks/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/load_graph_one_step
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_graph(name: &str, project_root: &Path) -> Result<(ArtifactGraph, SchemaSource)>`

# Calls

- [resolve_schema](../../../../functions/src/core/schema/resolve_schema.md)
- [into_graph](../../../../functions/src/core/schema/WorkflowSchema/into_graph.md)

# Called by

- [run](../../../../functions/src/cli/pipeline/run.md)
- [run](../../../../functions/src/cli/status/run.md)
- [run](../../../../functions/src/cli/tasks/run.md)
- [load_graph_one_step](../../../../functions/src/core/schema/load_graph_one_step.md)