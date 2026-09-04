---
type: Rust Module
title: checklist
resource: src/cli/checklist.rs#L1-L143
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-lazylock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-feature
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
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [run](../../../functions/src/cli/checklist/run.md)
- [find_last_chk_id](../../../functions/src/cli/checklist/find_last_chk_id.md)
- [generate_append_items](../../../functions/src/cli/checklist/generate_append_items.md)
- [find_last_chk_id_from_content](../../../functions/src/cli/checklist/find_last_chk_id_from_content.md)
- [find_last_chk_id_empty](../../../functions/src/cli/checklist/find_last_chk_id_empty.md)
- [append_items_start_from_given_id](../../../functions/src/cli/checklist/append_items_start_from_given_id.md)
- [append_continues_from_last_id](../../../functions/src/cli/checklist/append_continues_from_last_id.md)
- [checklist_items_match_format](../../../functions/src/cli/checklist/checklist_items_match_format.md)

# Imports

- `std::collections::HashMap`
- `std::sync::LazyLock`
- `anyhow::{Context, Result}`
- `regex::Regex`
- `crate::config`
- `crate::core::feature`
- `crate::presets::manager as preset_manager`
- `crate::templates`
- `crate::templates::resolver`
- `super::*`

# Member of

- [solidspec](../../../packages/solidspec.md)