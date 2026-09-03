---
type: Rust Function
title: latest_feature_dir
resource: src/core/feature.rs#L205-L236
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn latest_feature_dir(specs_dir: &Path) -> Result<String>`

# Called by

- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)