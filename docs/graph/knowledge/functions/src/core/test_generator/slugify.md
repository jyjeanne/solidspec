---
type: Rust Function
title: slugify
resource: src/core/test_generator.rs#L272-L294
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/text/truncate_at_boundary
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn slugify(text: &str, style: &SlugStyle) -> String`

# Calls

- [truncate_at_boundary](../../../../functions/src/core/text/truncate_at_boundary.md)