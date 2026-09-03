---
type: Rust Function
title: sanitize
resource: src/core/feature.rs#L94-L99
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/feature/generate_branch_name
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sanitize(s: &str) -> String`

# Called by

- [generate_branch_name](../../../../functions/src/core/feature/generate_branch_name.md)