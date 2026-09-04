---
type: Rust Function
title: extract_inline_field
resource: src/core/intent_parser.rs#L123-L128
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/intent_parser/parse_intent_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_inline_field(content: &str, re: &Regex) -> String`

# Called by

- [parse_intent_content](../../../../functions/src/core/intent_parser/parse_intent_content.md)