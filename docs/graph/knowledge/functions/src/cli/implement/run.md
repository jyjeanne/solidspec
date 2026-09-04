---
type: Rust Function
title: run
resource: src/cli/implement.rs#L9-L69
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
  - target: functions/src/extensions/manager/load_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/hooks/fire_hooks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_id: Option<&str>, pass: Option<u32>) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [load_registry](../../../../functions/src/extensions/manager/load_registry.md)
- [fire_hooks](../../../../functions/src/extensions/hooks/fire_hooks.md)