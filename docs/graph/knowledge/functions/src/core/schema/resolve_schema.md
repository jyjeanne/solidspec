---
type: Rust Function
title: resolve_schema
resource: src/core/schema.rs#L112-L134
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/schema/by_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/all_schema_spcx_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/graph_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/load_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/resolve_builtin_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/resolve_unknown_falls_back_to_default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/resolve_project_local_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/resolve_apex_driven_builtin
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/resolve_intent_apex_builtin
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_schema(name: &str, project_root: &Path) -> Result<(WorkflowSchema, SchemaSource)>`

# Calls

- [by_name](../../../../functions/src/core/schema/by_name.md)

# Called by

- [all_schema_spcx_commands](../../../../functions/src/agents/registry/all_schema_spcx_commands.md)
- [graph_for](../../../../functions/src/core/pipeline/graph_for.md)
- [load_graph](../../../../functions/src/core/schema/load_graph.md)
- [resolve_builtin_schema](../../../../functions/src/core/schema/resolve_builtin_schema.md)
- [resolve_unknown_falls_back_to_default](../../../../functions/src/core/schema/resolve_unknown_falls_back_to_default.md)
- [resolve_project_local_override](../../../../functions/src/core/schema/resolve_project_local_override.md)
- [resolve_apex_driven_builtin](../../../../functions/src/core/schema/resolve_apex_driven_builtin.md)
- [resolve_intent_apex_builtin](../../../../functions/src/core/schema/resolve_intent_apex_builtin.md)