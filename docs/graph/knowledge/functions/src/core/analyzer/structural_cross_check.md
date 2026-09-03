---
type: Rust Function
title: structural_cross_check
resource: src/core/analyzer.rs#L305-L350
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/analyzer/extract_symbol_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/okf/BundleIndex/has_symbol
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/extract_file_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/okf/BundleIndex/has_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/analyzer/analyze_feature
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn structural_cross_check(tasks_content: &str, project_root: &Path) -> Option<Vec<Finding>>`

# Calls

- [extract_symbol_name](../../../../functions/src/core/analyzer/extract_symbol_name.md)
- [has_symbol](../../../../functions/src/core/okf/BundleIndex/has_symbol.md)
- [extract_file_path](../../../../functions/src/core/analyzer/extract_file_path.md)
- [has_file](../../../../functions/src/core/okf/BundleIndex/has_file.md)

# Called by

- [analyze_feature](../../../../functions/src/core/analyzer/analyze_feature.md)