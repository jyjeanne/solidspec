---
type: Rust Function
title: schema
resource: src/agents/spcx.rs#L213-L215
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/schema/by_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/spcx/minimal_new_covers_specify_plan_tasks_and_stops_before_implement
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/minimal_finalise_has_nothing_to_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/minimal_apply_says_schema_ends_here
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/spec_driven_new_stops_before_implement_includes_tests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/spec_driven_finalise_includes_analyze_review_and_ship
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/security_first_new_includes_security_review_step
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/security_first_finalise_has_nothing_to_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/tdd_driven_apply_covers_all_three_handoff_phases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/intent_driven_new_uses_intent_cli_command_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/every_builtin_schema_generates_without_error
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn schema(name: &str) -> WorkflowSchema`

# Calls

- [by_name](../../../../functions/src/core/schema/by_name.md)

# Called by

- [minimal_new_covers_specify_plan_tasks_and_stops_before_implement](../../../../functions/src/agents/spcx/minimal_new_covers_specify_plan_tasks_and_stops_before_implement.md)
- [minimal_finalise_has_nothing_to_run](../../../../functions/src/agents/spcx/minimal_finalise_has_nothing_to_run.md)
- [minimal_apply_says_schema_ends_here](../../../../functions/src/agents/spcx/minimal_apply_says_schema_ends_here.md)
- [spec_driven_new_stops_before_implement_includes_tests](../../../../functions/src/agents/spcx/spec_driven_new_stops_before_implement_includes_tests.md)
- [spec_driven_finalise_includes_analyze_review_and_ship](../../../../functions/src/agents/spcx/spec_driven_finalise_includes_analyze_review_and_ship.md)
- [security_first_new_includes_security_review_step](../../../../functions/src/agents/spcx/security_first_new_includes_security_review_step.md)
- [security_first_finalise_has_nothing_to_run](../../../../functions/src/agents/spcx/security_first_finalise_has_nothing_to_run.md)
- [tdd_driven_apply_covers_all_three_handoff_phases](../../../../functions/src/agents/spcx/tdd_driven_apply_covers_all_three_handoff_phases.md)
- [intent_driven_new_uses_intent_cli_command_first](../../../../functions/src/agents/spcx/intent_driven_new_uses_intent_cli_command_first.md)
- [every_builtin_schema_generates_without_error](../../../../functions/src/agents/spcx/every_builtin_schema_generates_without_error.md)