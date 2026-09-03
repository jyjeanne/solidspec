---
type: Rust Module
title: cli
resource: src/cli/mod.rs#L1-L548
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/clap-parser-subcommand
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Cli](../../classes/src/cli/Cli.md)
- [Commands](../../classes/src/cli/Commands.md)
- [resolved_schema](../../functions/src/cli/resolved_schema.md)
- [run](../../functions/src/cli/run.md)

# Imports

- `anyhow::Result`
- `clap::{Parser, Subcommand}`

# Member of

- [solidspec](../../packages/solidspec.md)