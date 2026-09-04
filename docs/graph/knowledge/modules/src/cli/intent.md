---
type: Rust Module
title: intent
resource: src/cli/intent.rs#L1-L138
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
  - target: external/crate-core-feature-git
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

- [run](../../../functions/src/cli/intent/run.md)
- [build_template_vars](../../../functions/src/cli/intent/build_template_vars.md)

# Imports

- `std::collections::HashMap`
- `anyhow::{Context, Result}`
- `crate::config`
- `crate::core::{feature, git}`
- `crate::presets::manager as preset_manager`
- `crate::templates`
- `crate::templates::resolver`

# Member of

- [solidspec](../../../packages/solidspec.md)