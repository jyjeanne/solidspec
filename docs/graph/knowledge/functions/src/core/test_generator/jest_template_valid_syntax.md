---
type: Rust Function
title: jest_template_valid_syntax
resource: src/core/test_generator.rs#L751-L770
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/test_generator/extract_scenarios
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/framework_from_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/render_test_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn jest_template_valid_syntax()`

# Calls

- [extract_scenarios](../../../../functions/src/core/test_generator/extract_scenarios.md)
- [framework_from_name](../../../../functions/src/core/test_generator/framework_from_name.md)
- [render_test_file](../../../../functions/src/core/test_generator/render_test_file.md)