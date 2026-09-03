---
type: Rust Function
title: run
resource: src/cli/apex.rs#L6-L81
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
  - target: functions/src/core/apex/find_latest_execute_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/sync_tasks_from_apex_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/build_solidspec_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_id: Option<&str>, sync: bool, context_only: bool, dry_run: bool) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [find_latest_execute_log](../../../../functions/src/core/apex/find_latest_execute_log.md)
- [sync_tasks_from_apex_log](../../../../functions/src/core/apex/sync_tasks_from_apex_log.md)
- [build_solidspec_context](../../../../functions/src/core/apex/build_solidspec_context.md)