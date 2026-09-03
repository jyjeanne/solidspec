---
type: Rust Function
title: detect_framework
resource: src/core/test_generator.rs#L127-L219
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/tests_cmd/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/detect_jest_from_package_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/detect_typescript_with_tsconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/detect_cargo_test
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/detect_pytest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/detect_go_test
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/detect_generic_when_no_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn detect_framework(project_root: &Path) -> TestFramework`

# Called by

- [run](../../../../functions/src/cli/tests_cmd/run.md)
- [detect_jest_from_package_json](../../../../functions/src/core/test_generator/detect_jest_from_package_json.md)
- [detect_typescript_with_tsconfig](../../../../functions/src/core/test_generator/detect_typescript_with_tsconfig.md)
- [detect_cargo_test](../../../../functions/src/core/test_generator/detect_cargo_test.md)
- [detect_pytest](../../../../functions/src/core/test_generator/detect_pytest.md)
- [detect_go_test](../../../../functions/src/core/test_generator/detect_go_test.md)
- [detect_generic_when_no_files](../../../../functions/src/core/test_generator/detect_generic_when_no_files.md)