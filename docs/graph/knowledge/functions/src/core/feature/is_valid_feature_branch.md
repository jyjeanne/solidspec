---
type: Rust Function
title: is_valid_feature_branch
resource: src/core/feature.rs#L102-L104
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_valid_feature_branch(name: &str) -> bool`

# Called by

- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)