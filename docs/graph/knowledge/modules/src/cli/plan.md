---
type: Rust Module
title: plan
resource: src/cli/plan.rs#L1-L247
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
  - target: external/crate-core-constitution-feature-intent-parser-spec-parser
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

- [run](../../../functions/src/cli/plan/run.md)

# Imports

- `std::collections::HashMap`
- `anyhow::{Context, Result}`
- `crate::config`
- `crate::core::{constitution, feature, intent_parser, spec_parser}`
- `crate::presets::manager as preset_manager`
- `crate::templates`
- `crate::templates::resolver`

# Member of

- [solidspec](../../../packages/solidspec.md)