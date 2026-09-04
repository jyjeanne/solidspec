---
type: Rust Function
title: truncate_at_boundary
resource: src/core/text.rs#L3-L12
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/apex/build_solidspec_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/slugify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/test_file_name
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn truncate_at_boundary(s: &str, max_bytes: usize) -> &str`

# Called by

- [build_solidspec_context](../../../../functions/src/core/apex/build_solidspec_context.md)
- [slugify](../../../../functions/src/core/test_generator/slugify.md)
- [test_file_name](../../../../functions/src/core/test_generator/test_file_name.md)