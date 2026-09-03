---
type: Rust Module
title: status
resource: src/cli/status.rs#L1-L107
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-artifact-graph-artifactstate
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-schema
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-analyzer-feature-pipeline
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [run](../../../functions/src/cli/status/run.md)

# Imports

- `anyhow::Result`
- `crate::config`
- `crate::core::artifact_graph::ArtifactState`
- `crate::core::schema`
- `crate::core::{analyzer, feature, pipeline}`

# Member of

- [solidspec](../../../packages/solidspec.md)