---
type: Rust Function
title: framework_from_name
resource: src/core/test_generator.rs#L222-L268
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/tests_cmd/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/test_file_name_long_multibyte_title_does_not_panic
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
  - target: functions/src/core/test_generator/test_file_name_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/test_file_name_truncated
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn framework_from_name(name: &str) -> Option<TestFramework>`

# Calls

- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [run](../../../../functions/src/cli/tests_cmd/run.md)
- [test_file_name_long_multibyte_title_does_not_panic](../../../../functions/src/core/test_generator/test_file_name_long_multibyte_title_does_not_panic.md)
- [jest_template_valid_syntax](../../../../functions/src/core/test_generator/jest_template_valid_syntax.md)
- [pytest_template_valid_syntax](../../../../functions/src/core/test_generator/pytest_template_valid_syntax.md)
- [cargo_template_valid_syntax](../../../../functions/src/core/test_generator/cargo_template_valid_syntax.md)
- [go_template_valid_syntax](../../../../functions/src/core/test_generator/go_template_valid_syntax.md)
- [generic_template_valid](../../../../functions/src/core/test_generator/generic_template_valid.md)
- [test_file_name_format](../../../../functions/src/core/test_generator/test_file_name_format.md)
- [test_file_name_truncated](../../../../functions/src/core/test_generator/test_file_name_truncated.md)