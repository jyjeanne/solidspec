---
type: Rust Module
title: check
resource: tests/check.rs#L1-L38
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/assert-cmd-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/predicates-prelude
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

- [check_detects_non_project_directory](../../functions/tests/check/check_detects_non_project_directory.md)
- [check_detects_initialized_project](../../functions/tests/check/check_detects_initialized_project.md)

# Imports

- `assert_cmd::Command`
- `predicates::prelude::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../packages/solidspec.md)