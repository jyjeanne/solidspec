---
type: Rust Function
title: extract_requirements
resource: src/core/spec_parser.rs#L109-L117
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/spec_parser/parse_spec_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_requirements(content: &str) -> Vec<Requirement>`

# Called by

- [parse_spec_content](../../../../functions/src/core/spec_parser/parse_spec_content.md)