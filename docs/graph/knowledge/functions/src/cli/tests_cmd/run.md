---
type: Rust Function
title: run
resource: src/cli/tests_cmd.rs#L6-L129
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/parse_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/extract_scenarios
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/framework_from_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/detect_framework
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/test_file_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/render_test_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run( feature_id: Option<&str>, framework: Option<&str>, output: Option<&str>, dry_run: bool, ) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [parse_spec](../../../../functions/src/core/spec_parser/parse_spec.md)
- [extract_scenarios](../../../../functions/src/core/test_generator/extract_scenarios.md)
- [framework_from_name](../../../../functions/src/core/test_generator/framework_from_name.md)
- [detect_framework](../../../../functions/src/core/test_generator/detect_framework.md)
- [test_file_name](../../../../functions/src/core/test_generator/test_file_name.md)
- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)
- [render_test_file](../../../../functions/src/core/test_generator/render_test_file.md)