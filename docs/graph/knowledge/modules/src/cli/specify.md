---
type: Rust Module
title: specify
resource: src/cli/specify.rs#L1-L229
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-feature-git-spec-parser
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-presets-manager-as-preset-manager
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-templates
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-templates-resolver
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [run](../../../functions/src/cli/specify/run.md)
- [run_for_existing](../../../functions/src/cli/specify/run_for_existing.md)
- [write_spec](../../../functions/src/cli/specify/write_spec.md)
- [build_template_vars](../../../functions/src/cli/specify/build_template_vars.md)

# Imports

- `std::collections::HashMap`
- `anyhow::{Context, Result}`
- `crate::config`
- `crate::core::{feature, git, spec_parser}`
- `crate::presets::manager as preset_manager`
- `crate::templates`
- `crate::templates::resolver`

# Member of

- [solidspec](../../../packages/solidspec.md)