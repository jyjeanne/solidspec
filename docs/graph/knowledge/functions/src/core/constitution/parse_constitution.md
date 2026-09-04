---
type: Rust Function
title: parse_constitution
resource: src/core/constitution.rs#L50-L91
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/constitution/load_constitution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/load_valid_constitution_extracts_all_gates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/gate_evaluation_passes_when_no_violations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/gate_evaluation_fails_with_violation_details
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/custom_constitution_with_only_simplicity
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_constitution(content: &str) -> Result<Constitution>`

# Called by

- [load_constitution](../../../../functions/src/core/constitution/load_constitution.md)
- [load_valid_constitution_extracts_all_gates](../../../../functions/src/core/constitution/load_valid_constitution_extracts_all_gates.md)
- [gate_evaluation_passes_when_no_violations](../../../../functions/src/core/constitution/gate_evaluation_passes_when_no_violations.md)
- [gate_evaluation_fails_with_violation_details](../../../../functions/src/core/constitution/gate_evaluation_fails_with_violation_details.md)
- [custom_constitution_with_only_simplicity](../../../../functions/src/core/constitution/custom_constitution_with_only_simplicity.md)