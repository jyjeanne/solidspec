---
type: Rust Function
title: current_branch_works_from_subdirectory
resource: src/core/git.rs#L126-L142
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
---

# Signature

`fn current_branch_works_from_subdirectory()`

# Calls

- [init_repo](../../../../functions/src/core/git/init_repo.md)
- [create_branch](../../../../functions/src/core/git/create_branch.md)