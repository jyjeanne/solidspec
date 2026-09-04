---
type: Rust Function
title: test_file_name
resource: src/core/test_generator.rs#L527-L534
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/text/truncate_at_boundary
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/tests_cmd/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/test_file_name_long_multibyte_title_does_not_panic
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

`pub fn test_file_name(story_index: usize, story_title: &str, framework: &TestFramework) -> String`

# Calls

- [truncate_at_boundary](../../../../functions/src/core/text/truncate_at_boundary.md)

# Called by

- [run](../../../../functions/src/cli/tests_cmd/run.md)
- [test_file_name_long_multibyte_title_does_not_panic](../../../../functions/src/core/test_generator/test_file_name_long_multibyte_title_does_not_panic.md)
- [test_file_name_format](../../../../functions/src/core/test_generator/test_file_name_format.md)
- [test_file_name_truncated](../../../../functions/src/core/test_generator/test_file_name_truncated.md)