---
type: Rust Function
title: refresh_knowledge_graph
resource: src/cli/pipeline.rs#L329-L343
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/okf/refresh_if_present
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/pipeline/execute_phase
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn refresh_knowledge_graph(project_root: &std::path::Path)`

# Calls

- [refresh_if_present](../../../../functions/src/core/okf/refresh_if_present.md)

# Called by

- [execute_phase](../../../../functions/src/cli/pipeline/execute_phase.md)