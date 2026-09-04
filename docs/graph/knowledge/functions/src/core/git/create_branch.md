---
type: Rust Function
title: create_branch
resource: src/core/git.rs#L39-L63
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/intent/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/run
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

`pub fn create_branch(repo_path: &Path, branch_name: &str) -> Result<()>`

# Called by

- [run](../../../../functions/src/cli/intent/run.md)
- [run](../../../../functions/src/cli/specify/run.md)
- [create_branch_and_read_current](../../../../functions/src/core/git/create_branch_and_read_current.md)
- [current_branch_works_from_subdirectory](../../../../functions/src/core/git/current_branch_works_from_subdirectory.md)