---
type: Rust Function
title: gate_evaluation_passes_when_no_violations
resource: src/core/constitution.rs#L256-L261
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

`fn gate_evaluation_passes_when_no_violations()`

# Calls

- [parse_constitution](../../../../functions/src/core/constitution/parse_constitution.md)
- [check_plan_compliance](../../../../functions/src/core/constitution/check_plan_compliance.md)