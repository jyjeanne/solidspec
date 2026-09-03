---
type: Rust Function
title: analyze_does_not_modify_files
resource: src/core/analyzer.rs#L550-L567
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/analyzer/setup_constitution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/analyze_feature
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn analyze_does_not_modify_files()`

# Calls

- [setup_constitution](../../../../functions/src/core/analyzer/setup_constitution.md)
- [analyze_feature](../../../../functions/src/core/analyzer/analyze_feature.md)