---
type: Rust Function
title: cross_reference_gaps_found
resource: src/core/review/checks.rs#L648-L653
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/spec_parser/parse_spec_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_cross_references
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn cross_reference_gaps_found()`

# Calls

- [parse_spec_content](../../../../../functions/src/core/spec_parser/parse_spec_content.md)
- [check_cross_references](../../../../../functions/src/core/review/checks/check_cross_references.md)