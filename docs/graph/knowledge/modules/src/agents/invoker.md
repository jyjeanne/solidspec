---
type: Rust Module
title: invoker
resource: src/agents/invoker.rs#L1-L558
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-thread
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration-instant
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-config-agentconfig-find-agent
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-guardrails
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-personas
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-registry-find-binary
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-read
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-stdio
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

- [build_phase_prompt](../../../functions/src/agents/invoker/build_phase_prompt.md)
- [InvokeResult](../../../classes/src/agents/invoker/InvokeResult.md)
- [resolve_agent_cli](../../../functions/src/agents/invoker/resolve_agent_cli.md)
- [invoke_agent](../../../functions/src/agents/invoker/invoke_agent.md)
- [run_agent_cli](../../../functions/src/agents/invoker/run_agent_cli.md)
- [build_agent_args](../../../functions/src/agents/invoker/build_agent_args.md)
- [wait_with_timeout](../../../functions/src/agents/invoker/wait_with_timeout.md)
- [run_agent_cli_capture](../../../functions/src/agents/invoker/run_agent_cli_capture.md)
- [invoke_agent_with_prompt](../../../functions/src/agents/invoker/invoke_agent_with_prompt.md)
- [supports_cli](../../../functions/src/agents/invoker/supports_cli.md)
- [build_specify_prompt_includes_feature](../../../functions/src/agents/invoker/build_specify_prompt_includes_feature.md)
- [build_plan_prompt_includes_all_docs](../../../functions/src/agents/invoker/build_plan_prompt_includes_all_docs.md)
- [build_tasks_prompt_mentions_phases](../../../functions/src/agents/invoker/build_tasks_prompt_mentions_phases.md)
- [build_clarify_prompt_mentions_markers](../../../functions/src/agents/invoker/build_clarify_prompt_mentions_markers.md)
- [build_tests_prompt_mentions_scaffolds](../../../functions/src/agents/invoker/build_tests_prompt_mentions_scaffolds.md)
- [build_analyze_prompt_mentions_consistency](../../../functions/src/agents/invoker/build_analyze_prompt_mentions_consistency.md)
- [build_security_review_prompt_mentions_owasp_and_gate](../../../functions/src/agents/invoker/build_security_review_prompt_mentions_owasp_and_gate.md)
- [build_unknown_phase_returns_generic](../../../functions/src/agents/invoker/build_unknown_phase_returns_generic.md)
- [invoke_unknown_agent_returns_not_available](../../../functions/src/agents/invoker/invoke_unknown_agent_returns_not_available.md)
- [invoke_no_cli_agent_returns_not_available](../../../functions/src/agents/invoker/invoke_no_cli_agent_returns_not_available.md)
- [invoke_agent_with_prompt_unknown_agent_returns_not_available](../../../functions/src/agents/invoker/invoke_agent_with_prompt_unknown_agent_returns_not_available.md)
- [invoke_agent_with_prompt_no_cli_agent_returns_not_available](../../../functions/src/agents/invoker/invoke_agent_with_prompt_no_cli_agent_returns_not_available.md)
- [supports_cli_false_for_no_binary](../../../functions/src/agents/invoker/supports_cli_false_for_no_binary.md)
- [supports_cli_false_for_unknown](../../../functions/src/agents/invoker/supports_cli_false_for_unknown.md)
- [prompts_include_persona_role_section](../../../functions/src/agents/invoker/prompts_include_persona_role_section.md)
- [prompts_include_compliance_guardrails](../../../functions/src/agents/invoker/prompts_include_compliance_guardrails.md)
- [project_context_injected_when_provided](../../../functions/src/agents/invoker/project_context_injected_when_provided.md)
- [empty_project_context_is_skipped](../../../functions/src/agents/invoker/empty_project_context_is_skipped.md)

# Imports

- `std::path::Path`
- `std::process::Command`
- `std::thread`
- `std::time::{Duration, Instant}`
- `anyhow::{Context, Result}`
- `super::config::{AgentConfig, find_agent}`
- `super::guardrails`
- `super::personas`
- `super::registry::find_binary`
- `std::io::Read`
- `std::process::Stdio`
- `super::*`

# Member of

- [solidspec](../../../packages/solidspec.md)