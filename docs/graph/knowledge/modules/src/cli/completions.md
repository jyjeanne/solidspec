---
type: Rust Module
title: completions
resource: src/cli/completions.rs#L1-L63
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/clap-commandfactory
    resolved_by: tree-sitter
    confidence: exact
  - target: external/clap-complete-shell-generate
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-cli
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

- [generate_completions](../../../functions/src/cli/completions/generate_completions.md)
- [run](../../../functions/src/cli/completions/run.md)
- [bash_completions_contain_subcommands](../../../functions/src/cli/completions/bash_completions_contain_subcommands.md)
- [powershell_completions_generated](../../../functions/src/cli/completions/powershell_completions_generated.md)
- [fish_completions_generated](../../../functions/src/cli/completions/fish_completions_generated.md)
- [zsh_completions_generated](../../../functions/src/cli/completions/zsh_completions_generated.md)

# Imports

- `std::io::Write`
- `anyhow::Result`
- `clap::CommandFactory`
- `clap_complete::{Shell, generate}`
- `super::Cli`
- `super::*`

# Member of

- [solidspec](../../../packages/solidspec.md)