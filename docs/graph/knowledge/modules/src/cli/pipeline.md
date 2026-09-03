---
type: Rust Module
title: pipeline
resource: src/cli/pipeline.rs#L1-L553
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-instant
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-agents-invoker-self-invokeresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-feature-pipeline
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [run](../../../functions/src/cli/pipeline/run.md)
- [AgentMode](../../../classes/src/cli/pipeline/AgentMode.md)
- [check_agent_availability](../../../functions/src/cli/pipeline/check_agent_availability.md)
- [execute_phase](../../../functions/src/cli/pipeline/execute_phase.md)
- [invoke_or_handoff](../../../functions/src/cli/pipeline/invoke_or_handoff.md)
- [skip_reason](../../../functions/src/cli/pipeline/skip_reason.md)

# Imports

- `std::io::Write`
- `std::time::Instant`
- `anyhow::{Context, Result}`
- `crate::agents::invoker::{self, InvokeResult}`
- `crate::config`
- `crate::core::{feature, pipeline}`

# Member of

- [solidspec](../../../packages/solidspec.md)