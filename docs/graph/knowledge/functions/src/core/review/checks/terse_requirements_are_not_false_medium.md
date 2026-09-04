---
type: Rust Function
title: terse_requirements_are_not_false_medium
resource: src/core/review/checks.rs#L774-L798
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/spec_parser/parse_spec_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/review_intent_alignment
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn terse_requirements_are_not_false_medium()`

# Calls

- [parse_spec_content](../../../../../functions/src/core/spec_parser/parse_spec_content.md)
- [review_intent_alignment](../../../../../functions/src/core/review/checks/review_intent_alignment.md)