---
type: Rust Function
title: refresh_if_present
resource: src/core/okf.rs#L105-L111
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/pipeline/refresh_knowledge_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/okf/refresh_if_present_regenerates_an_existing_bundle_in_place
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn refresh_if_present(project_root: &Path) -> Option<Result<GenerateReport>>`

# Called by

- [refresh_knowledge_graph](../../../../functions/src/cli/pipeline/refresh_knowledge_graph.md)
- [refresh_if_present_regenerates_an_existing_bundle_in_place](../../../../functions/src/core/okf/refresh_if_present_regenerates_an_existing_bundle_in_place.md)