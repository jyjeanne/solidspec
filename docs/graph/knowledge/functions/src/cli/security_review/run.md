---
type: Rust Function
title: run
resource: src/cli/security_review.rs#L6-L52
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
  - target: functions/src/core/security_review/run_security_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/security_review/format_security_review
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_id: Option<&str>, dry_run: bool) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [run_security_review](../../../../functions/src/core/security_review/run_security_review.md)
- [format_security_review](../../../../functions/src/core/security_review/format_security_review.md)