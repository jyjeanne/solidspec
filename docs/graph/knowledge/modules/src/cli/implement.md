---
type: Rust Module
title: implement
resource: src/cli/implement.rs#L1-L113
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-feature
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-extensions
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

- [run](../../../functions/src/cli/implement/run.md)
- [mark_task_done](../../../functions/src/cli/implement/mark_task_done.md)
- [mark_task_done_updates_checkbox](../../../functions/src/cli/implement/mark_task_done_updates_checkbox.md)
- [mark_nonexistent_task_is_noop](../../../functions/src/cli/implement/mark_nonexistent_task_is_noop.md)

# Imports

- `std::path::Path`
- `anyhow::{Context, Result}`
- `crate::config`
- `crate::core::feature`
- `crate::extensions`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)