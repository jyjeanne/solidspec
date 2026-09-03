---
type: Rust Function
title: extract_entities
resource: src/core/spec_parser.rs#L137-L142
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/spec_parser/parse_spec_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_entities(content: &str) -> Vec<String>`

# Calls

- [parse_spec_content](../../../../functions/src/core/spec_parser/parse_spec_content.md)