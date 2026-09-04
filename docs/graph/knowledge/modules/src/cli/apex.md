---
type: Rust Module
title: apex
resource: src/cli/apex.rs#L1-L122
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
  - target: external/crate-core-apex-feature
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [run](../../../functions/src/cli/apex/run.md)
- [feature_slug](../../../functions/src/cli/apex/feature_slug.md)
- [feature_slug_strips_numeric_prefix](../../../functions/src/cli/apex/feature_slug_strips_numeric_prefix.md)
- [feature_slug_leaves_non_numeric_prefix_intact](../../../functions/src/cli/apex/feature_slug_leaves_non_numeric_prefix_intact.md)
- [feature_slug_leaves_plain_name_intact](../../../functions/src/cli/apex/feature_slug_leaves_plain_name_intact.md)
- [run_fails_without_project_root](../../../functions/src/cli/apex/run_fails_without_project_root.md)

# Imports

- `anyhow::{Context, Result}`
- `crate::config`
- `crate::core::{apex, feature}`
- `super::*`

# Member of

- [solidspec](../../../packages/solidspec.md)