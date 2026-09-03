---
type: Rust Function
title: constitution_violation_is_critical
resource: src/core/analyzer.rs#L676-L690
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

`fn constitution_violation_is_critical()`

# Calls

- [setup_constitution](../../../../functions/src/core/analyzer/setup_constitution.md)
- [analyze_feature](../../../../functions/src/core/analyzer/analyze_feature.md)