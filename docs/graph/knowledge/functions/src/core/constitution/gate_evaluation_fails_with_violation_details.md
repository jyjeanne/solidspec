---
type: Rust Function
title: gate_evaluation_fails_with_violation_details
resource: src/core/constitution.rs#L264-L271
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/constitution/parse_constitution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/check_plan_compliance
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn gate_evaluation_fails_with_violation_details()`

# Calls

- [parse_constitution](../../../../functions/src/core/constitution/parse_constitution.md)
- [check_plan_compliance](../../../../functions/src/core/constitution/check_plan_compliance.md)