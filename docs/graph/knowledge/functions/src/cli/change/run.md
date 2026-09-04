---
type: Rust Function
title: run
resource: src/cli/change.rs#L37-L113
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
  - target: functions/src/core/change/create_change
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/list_changes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/archive_change
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(command: ChangeCommands) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [create_change](../../../../functions/src/core/change/create_change.md)
- [list_changes](../../../../functions/src/core/change/list_changes.md)
- [archive_change](../../../../functions/src/core/change/archive_change.md)