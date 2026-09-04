---
type: Rust Function
title: run
resource: src/cli/clarify.rs#L6-L63
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/parse_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/Task/format
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_id: Option<&str>) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [parse_spec](../../../../functions/src/core/spec_parser/parse_spec.md)
- [format](../../../../functions/src/core/task_generator/Task/format.md)