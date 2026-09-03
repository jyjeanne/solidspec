---
type: Rust Module
title: git
resource: src/core/git.rs#L1-L143
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/git2-repository-signature
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-errors-solidspecerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [is_git_repo](../../../functions/src/core/git/is_git_repo.md)
- [init_repo](../../../functions/src/core/git/init_repo.md)
- [create_branch](../../../functions/src/core/git/create_branch.md)
- [current_branch](../../../functions/src/core/git/current_branch.md)
- [is_git_repo_false_for_plain_dir](../../../functions/src/core/git/is_git_repo_false_for_plain_dir.md)
- [init_repo_creates_git_dir_and_initial_commit](../../../functions/src/core/git/init_repo_creates_git_dir_and_initial_commit.md)
- [create_branch_and_read_current](../../../functions/src/core/git/create_branch_and_read_current.md)
- [current_branch_returns_none_for_non_git](../../../functions/src/core/git/current_branch_returns_none_for_non_git.md)
- [current_branch_works_from_subdirectory](../../../functions/src/core/git/current_branch_works_from_subdirectory.md)

# Imports

- `std::path::Path`
- `anyhow::Result`
- `git2::{Repository, Signature}`
- `super::errors::SolidSpecError`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)