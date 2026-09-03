---
type: Rust Module
title: extension
resource: src/cli/extension.rs#L1-L117
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/clap-subcommand
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-extensions-manager
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ExtensionCommands](../../../classes/src/cli/extension/ExtensionCommands.md)
- [run](../../../functions/src/cli/extension/run.md)

# Imports

- `anyhow::{Context, Result}`
- `clap::Subcommand`
- `crate::config`
- `crate::extensions::manager`

# Member of

- [solidspec](../../../packages/solidspec.md)