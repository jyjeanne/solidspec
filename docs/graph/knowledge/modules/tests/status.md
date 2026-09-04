---
type: Rust Module
title: status
resource: tests/status.rs#L1-L137
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

- [status_shows_artifacts_after_pipeline_scaffold](../../functions/tests/status/status_shows_artifacts_after_pipeline_scaffold.md)
- [status_with_minimal_schema](../../functions/tests/status/status_with_minimal_schema.md)
- [status_fails_in_non_solidspec_dir](../../functions/tests/status/status_fails_in_non_solidspec_dir.md)
- [status_warns_instead_of_panicking_on_cyclic_schema](../../functions/tests/status/status_warns_instead_of_panicking_on_cyclic_schema.md)

# Imports

- `assert_cmd::Command`
- `predicates::prelude::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../packages/solidspec.md)