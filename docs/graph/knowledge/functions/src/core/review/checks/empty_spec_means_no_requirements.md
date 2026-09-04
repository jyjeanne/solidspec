---
type: Rust Function
title: empty_spec_means_no_requirements
resource: src/core/review/checks.rs#L641-L645
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/spec_parser/parse_spec_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_requirement_quality
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn empty_spec_means_no_requirements()`

# Calls

- [parse_spec_content](../../../../../functions/src/core/spec_parser/parse_spec_content.md)
- [check_requirement_quality](../../../../../functions/src/core/review/checks/check_requirement_quality.md)