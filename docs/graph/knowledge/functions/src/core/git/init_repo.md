---
type: Rust Function
title: init_repo
resource: src/core/git.rs#L12-L37
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/cli/security_review/init
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/init_repo_creates_git_dir_and_initial_commit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/create_branch_and_read_current
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/current_branch_works_from_subdirectory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn init_repo(path: &Path) -> Result<()>`

# Calls

- [init](../../../../functions/src/cli/security_review/init.md)

# Called by

- [run](../../../../functions/src/cli/init/run.md)
- [init_repo_creates_git_dir_and_initial_commit](../../../../functions/src/core/git/init_repo_creates_git_dir_and_initial_commit.md)
- [create_branch_and_read_current](../../../../functions/src/core/git/create_branch_and_read_current.md)
- [current_branch_works_from_subdirectory](../../../../functions/src/core/git/current_branch_works_from_subdirectory.md)