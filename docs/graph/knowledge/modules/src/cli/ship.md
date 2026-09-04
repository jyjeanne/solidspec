---
type: Rust Module
title: ship
resource: src/cli/ship.rs#L1-L144
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
  - target: external/crate-core-fan-out-self-shipdecision
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-feature
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [run](../../../functions/src/cli/ship/run.md)

# Imports

- `anyhow::{Context, Result}`
- `crate::config`
- `crate::core::fan_out::{self, ShipDecision}`
- `crate::core::feature`

# Member of

- [solidspec](../../../packages/solidspec.md)