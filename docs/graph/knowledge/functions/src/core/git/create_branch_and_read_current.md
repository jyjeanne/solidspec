---
type: Rust Function
title: create_branch_and_read_current
resource: src/core/git.rs#L108-L117
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/git/init_repo
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/create_branch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/current_branch
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_branch_and_read_current()`

# Calls

- [init_repo](../../../../functions/src/core/git/init_repo.md)
- [create_branch](../../../../functions/src/core/git/create_branch.md)
- [current_branch](../../../../functions/src/core/git/current_branch.md)