---
type: Rust Function
title: filter_phases
resource: src/core/pipeline.rs#L256-L296
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_all_phases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_idsd_phases_includes_intent_and_evidence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_minimal_phases_excludes_tests_and_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_security_first_phases_includes_security_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_from_plan_to_tasks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_only_one_phase
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_from_after_to_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_invalid_phase_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_apex_driven_has_apex_not_implement
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_apex_driven_apex_at_correct_position
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_intent_apex_has_all_idsd_phases_with_apex
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_intent_apex_apex_before_evidence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_apex_driven_from_tasks_to_analyze
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_apex_driven_only_apex
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_existing_schemas_unchanged_by_apex_addition
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn filter_phases( schema: &str, from: Option<&str>, to: Option<&str>, ) -> Result<Vec<&'static str>>`

# Called by

- [run](../../../../functions/src/cli/pipeline/run.md)
- [filter_all_phases](../../../../functions/src/core/pipeline/filter_all_phases.md)
- [filter_idsd_phases_includes_intent_and_evidence](../../../../functions/src/core/pipeline/filter_idsd_phases_includes_intent_and_evidence.md)
- [filter_minimal_phases_excludes_tests_and_review](../../../../functions/src/core/pipeline/filter_minimal_phases_excludes_tests_and_review.md)
- [filter_security_first_phases_includes_security_review](../../../../functions/src/core/pipeline/filter_security_first_phases_includes_security_review.md)
- [filter_from_plan_to_tasks](../../../../functions/src/core/pipeline/filter_from_plan_to_tasks.md)
- [filter_only_one_phase](../../../../functions/src/core/pipeline/filter_only_one_phase.md)
- [filter_from_after_to_errors](../../../../functions/src/core/pipeline/filter_from_after_to_errors.md)
- [filter_invalid_phase_errors](../../../../functions/src/core/pipeline/filter_invalid_phase_errors.md)
- [filter_apex_driven_has_apex_not_implement](../../../../functions/src/core/pipeline/filter_apex_driven_has_apex_not_implement.md)
- [filter_apex_driven_apex_at_correct_position](../../../../functions/src/core/pipeline/filter_apex_driven_apex_at_correct_position.md)
- [filter_intent_apex_has_all_idsd_phases_with_apex](../../../../functions/src/core/pipeline/filter_intent_apex_has_all_idsd_phases_with_apex.md)
- [filter_intent_apex_apex_before_evidence](../../../../functions/src/core/pipeline/filter_intent_apex_apex_before_evidence.md)
- [filter_apex_driven_from_tasks_to_analyze](../../../../functions/src/core/pipeline/filter_apex_driven_from_tasks_to_analyze.md)
- [filter_apex_driven_only_apex](../../../../functions/src/core/pipeline/filter_apex_driven_only_apex.md)
- [filter_existing_schemas_unchanged_by_apex_addition](../../../../functions/src/core/pipeline/filter_existing_schemas_unchanged_by_apex_addition.md)