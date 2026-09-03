---
type: Rust Method
title: into_graph
resource: src/core/schema.rs#L90-L104
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/graph_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/load_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/schema_converts_to_valid_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/apex_driven_converts_to_valid_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/intent_apex_converts_to_valid_graph
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn into_graph(self) -> Result<ArtifactGraph, String>`

# Called by

- [generate_bodies](../../../../../functions/src/agents/spcx/generate_bodies.md)
- [graph_for](../../../../../functions/src/core/pipeline/graph_for.md)
- [load_graph](../../../../../functions/src/core/schema/load_graph.md)
- [schema_converts_to_valid_graph](../../../../../functions/src/core/schema/schema_converts_to_valid_graph.md)
- [apex_driven_converts_to_valid_graph](../../../../../functions/src/core/schema/apex_driven_converts_to_valid_graph.md)
- [intent_apex_converts_to_valid_graph](../../../../../functions/src/core/schema/intent_apex_converts_to_valid_graph.md)