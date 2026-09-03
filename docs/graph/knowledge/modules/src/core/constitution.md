---
type: Rust Module
title: constitution
resource: src/core/constitution.rs#L1-L289
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-errors-solidspecerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Constitution](../../../classes/src/core/constitution/Constitution.md)
- [Gate](../../../classes/src/core/constitution/Gate.md)
- [GateResult](../../../classes/src/core/constitution/GateResult.md)
- [load_constitution](../../../functions/src/core/constitution/load_constitution.md)
- [parse_constitution](../../../functions/src/core/constitution/parse_constitution.md)
- [strip_constitution_section](../../../functions/src/core/constitution/strip_constitution_section.md)
- [check_plan_compliance](../../../functions/src/core/constitution/check_plan_compliance.md)
- [check_intent_constraints](../../../functions/src/core/constitution/check_intent_constraints.md)
- [load_valid_constitution_extracts_all_gates](../../../functions/src/core/constitution/load_valid_constitution_extracts_all_gates.md)
- [missing_constitution_returns_error_with_path](../../../functions/src/core/constitution/missing_constitution_returns_error_with_path.md)
- [gate_evaluation_passes_when_no_violations](../../../functions/src/core/constitution/gate_evaluation_passes_when_no_violations.md)
- [gate_evaluation_fails_with_violation_details](../../../functions/src/core/constitution/gate_evaluation_fails_with_violation_details.md)
- [custom_constitution_with_only_simplicity](../../../functions/src/core/constitution/custom_constitution_with_only_simplicity.md)
- [load_from_file](../../../functions/src/core/constitution/load_from_file.md)

# Imports

- `std::path::Path`
- `anyhow::Result`
- `super::errors::SolidSpecError`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)