---
type: Rust Function
title: validate_spec_quality
resource: src/core/spec_parser.rs#L162-L196
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/specify/write_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/validate_spec_detects_impl_details
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/validate_spec_clean_passes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/validate_spec_too_many_markers
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_spec_quality(content: &str) -> Vec<String>`

# Called by

- [write_spec](../../../../functions/src/cli/specify/write_spec.md)
- [validate_spec_detects_impl_details](../../../../functions/src/core/spec_parser/validate_spec_detects_impl_details.md)
- [validate_spec_clean_passes](../../../../functions/src/core/spec_parser/validate_spec_clean_passes.md)
- [validate_spec_too_many_markers](../../../../functions/src/core/spec_parser/validate_spec_too_many_markers.md)