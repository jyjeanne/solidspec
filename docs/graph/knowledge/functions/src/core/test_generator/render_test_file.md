---
type: Rust Function
title: render_test_file
resource: src/core/test_generator.rs#L299-L344
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/test_generator/render_jest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/render_pytest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/render_cargo
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/render_go
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/render_generic
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/tests_cmd/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/jest_template_valid_syntax
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/pytest_template_valid_syntax
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/cargo_template_valid_syntax
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/go_template_valid_syntax
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/generic_template_valid
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_test_file( feature_name: &str, story_index: usize, story_title: &str, story_priority: &str, scenarios: &[AcceptanceScenario], framework: &TestFramework, ) -> String`

# Calls

- [render_jest](../../../../functions/src/core/test_generator/render_jest.md)
- [render_pytest](../../../../functions/src/core/test_generator/render_pytest.md)
- [render_cargo](../../../../functions/src/core/test_generator/render_cargo.md)
- [render_go](../../../../functions/src/core/test_generator/render_go.md)
- [render_generic](../../../../functions/src/core/test_generator/render_generic.md)

# Called by

- [run](../../../../functions/src/cli/tests_cmd/run.md)
- [jest_template_valid_syntax](../../../../functions/src/core/test_generator/jest_template_valid_syntax.md)
- [pytest_template_valid_syntax](../../../../functions/src/core/test_generator/pytest_template_valid_syntax.md)
- [cargo_template_valid_syntax](../../../../functions/src/core/test_generator/cargo_template_valid_syntax.md)
- [go_template_valid_syntax](../../../../functions/src/core/test_generator/go_template_valid_syntax.md)
- [generic_template_valid](../../../../functions/src/core/test_generator/generic_template_valid.md)