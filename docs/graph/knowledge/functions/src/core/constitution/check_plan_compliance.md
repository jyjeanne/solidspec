---
type: Rust Function
title: check_plan_compliance
resource: src/core/constitution.rs#L117-L159
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/constitution/strip_constitution_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/analyze_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/gate_evaluation_passes_when_no_violations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/gate_evaluation_fails_with_violation_details
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn check_plan_compliance(constitution: &Constitution, plan_content: &str) -> Vec<GateResult>`

# Calls

- [strip_constitution_section](../../../../functions/src/core/constitution/strip_constitution_section.md)
- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [run](../../../../functions/src/cli/plan/run.md)
- [analyze_feature](../../../../functions/src/core/analyzer/analyze_feature.md)
- [gate_evaluation_passes_when_no_violations](../../../../functions/src/core/constitution/gate_evaluation_passes_when_no_violations.md)
- [gate_evaluation_fails_with_violation_details](../../../../functions/src/core/constitution/gate_evaluation_fails_with_violation_details.md)