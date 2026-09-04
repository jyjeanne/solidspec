---
type: Rust Function
title: extract_scenarios
resource: src/core/test_generator.rs#L28-L44
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/test_generator/parse_single_scenario
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/tests_cmd/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/extract_scenarios_from_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/scenario_splits_given_when_then
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/scenario_with_missing_when_skipped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/jest_template_valid_syntax
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn extract_scenarios(spec: &ParsedSpec) -> Vec<AcceptanceScenario>`

# Calls

- [parse_single_scenario](../../../../functions/src/core/test_generator/parse_single_scenario.md)

# Called by

- [run](../../../../functions/src/cli/tests_cmd/run.md)
- [extract_scenarios_from_spec](../../../../functions/src/core/test_generator/extract_scenarios_from_spec.md)
- [scenario_splits_given_when_then](../../../../functions/src/core/test_generator/scenario_splits_given_when_then.md)
- [scenario_with_missing_when_skipped](../../../../functions/src/core/test_generator/scenario_with_missing_when_skipped.md)
- [jest_template_valid_syntax](../../../../functions/src/core/test_generator/jest_template_valid_syntax.md)