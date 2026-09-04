---
type: Rust Function
title: apply_reminds_the_agent_to_refresh_the_knowledge_graph
resource: src/agents/spcx.rs#L326-L335
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/spcx/generate_bodies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn apply_reminds_the_agent_to_refresh_the_knowledge_graph()`

# Calls

- [generate_bodies](../../../../functions/src/agents/spcx/generate_bodies.md)
- [schema](../../../../functions/src/agents/spcx/schema.md)