---
type: Rust Module
title: security_review
resource: src/cli/security_review.rs#L1-L115
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-feature-security-review
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

- [run](../../../functions/src/cli/security_review/run.md)
- [init](../../../functions/src/cli/security_review/init.md)
- [fails_outside_project](../../../functions/src/cli/security_review/fails_outside_project.md)
- [fails_when_feature_dir_missing](../../../functions/src/cli/security_review/fails_when_feature_dir_missing.md)
- [writes_report_when_plan_exists](../../../functions/src/cli/security_review/writes_report_when_plan_exists.md)
- [dry_run_does_not_write_file](../../../functions/src/cli/security_review/dry_run_does_not_write_file.md)

# Imports

- `anyhow::{Context, Result}`
- `crate::config`
- `crate::core::{feature, security_review}`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)