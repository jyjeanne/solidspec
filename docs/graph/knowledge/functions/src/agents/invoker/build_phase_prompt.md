---
type: Rust Function
title: build_phase_prompt
resource: src/agents/invoker.rs#L18-L175
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/personas/persona_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/guardrails/compliance_footer
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/invoker/invoke_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/build_specify_prompt_includes_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/build_plan_prompt_includes_all_docs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/build_tasks_prompt_mentions_phases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/build_clarify_prompt_mentions_markers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/build_tests_prompt_mentions_scaffolds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/build_analyze_prompt_mentions_consistency
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/build_security_review_prompt_mentions_owasp_and_gate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/build_unknown_phase_returns_generic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/prompts_include_persona_role_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/prompts_include_compliance_guardrails
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/project_context_injected_when_provided
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/empty_project_context_is_skipped
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_phase_prompt( phase: &str, feature_dir_name: &str, description: Option<&str>, project_context: Option<&str>, ) -> String`

# Calls

- [persona_prompt](../../../../functions/src/agents/personas/persona_prompt.md)
- [compliance_footer](../../../../functions/src/agents/guardrails/compliance_footer.md)

# Called by

- [invoke_agent](../../../../functions/src/agents/invoker/invoke_agent.md)
- [build_specify_prompt_includes_feature](../../../../functions/src/agents/invoker/build_specify_prompt_includes_feature.md)
- [build_plan_prompt_includes_all_docs](../../../../functions/src/agents/invoker/build_plan_prompt_includes_all_docs.md)
- [build_tasks_prompt_mentions_phases](../../../../functions/src/agents/invoker/build_tasks_prompt_mentions_phases.md)
- [build_clarify_prompt_mentions_markers](../../../../functions/src/agents/invoker/build_clarify_prompt_mentions_markers.md)
- [build_tests_prompt_mentions_scaffolds](../../../../functions/src/agents/invoker/build_tests_prompt_mentions_scaffolds.md)
- [build_analyze_prompt_mentions_consistency](../../../../functions/src/agents/invoker/build_analyze_prompt_mentions_consistency.md)
- [build_security_review_prompt_mentions_owasp_and_gate](../../../../functions/src/agents/invoker/build_security_review_prompt_mentions_owasp_and_gate.md)
- [build_unknown_phase_returns_generic](../../../../functions/src/agents/invoker/build_unknown_phase_returns_generic.md)
- [prompts_include_persona_role_section](../../../../functions/src/agents/invoker/prompts_include_persona_role_section.md)
- [prompts_include_compliance_guardrails](../../../../functions/src/agents/invoker/prompts_include_compliance_guardrails.md)
- [project_context_injected_when_provided](../../../../functions/src/agents/invoker/project_context_injected_when_provided.md)
- [empty_project_context_is_skipped](../../../../functions/src/agents/invoker/empty_project_context_is_skipped.md)