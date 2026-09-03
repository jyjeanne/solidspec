---
type: Rust Function
title: load_constitution
resource: src/core/constitution.rs#L37-L48
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/constitution/parse_constitution
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/analyze_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/missing_constitution_returns_error_with_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/load_from_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_constitution(path: &Path) -> Result<Constitution>`

# Calls

- [parse_constitution](../../../../functions/src/core/constitution/parse_constitution.md)

# Called by

- [run](../../../../functions/src/cli/plan/run.md)
- [analyze_feature](../../../../functions/src/core/analyzer/analyze_feature.md)
- [missing_constitution_returns_error_with_path](../../../../functions/src/core/constitution/missing_constitution_returns_error_with_path.md)
- [load_from_file](../../../../functions/src/core/constitution/load_from_file.md)