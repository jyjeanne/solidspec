---
type: Rust Module
title: tasks
resource: src/cli/tasks.rs#L1-L75
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
  - target: external/crate-core-artifact-graph-artifactstate
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-feature-schema-spec-parser-task-generator
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-extensions
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [run](../../../functions/src/cli/tasks/run.md)

# Imports

- `anyhow::{Context, Result}`
- `crate::config`
- `crate::core::artifact_graph::ArtifactState`
- `crate::core::{feature, schema, spec_parser, task_generator}`
- `crate::extensions`

# Member of

- [solidspec](../../../packages/solidspec.md)