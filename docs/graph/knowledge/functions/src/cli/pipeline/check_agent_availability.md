---
type: Rust Function
title: check_agent_availability
resource: src/cli/pipeline.rs#L286-L316
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/PipelineConfig/agent_for_phase
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/supports_cli
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_agent_availability( phases: &[&str], pipeline_config: &config::PipelineConfig, default_agent: &str, ) -> AgentMode`

# Calls

- [agent_for_phase](../../../../functions/src/config/PipelineConfig/agent_for_phase.md)
- [supports_cli](../../../../functions/src/agents/invoker/supports_cli.md)

# Called by

- [run](../../../../functions/src/cli/pipeline/run.md)