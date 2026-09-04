---
type: Rust Function
title: current_branch
resource: src/core/git.rs#L65-L71
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/create_branch_and_read_current
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn current_branch(repo_path: &Path) -> Option<String>`

# Called by

- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [create_branch_and_read_current](../../../../functions/src/core/git/create_branch_and_read_current.md)