---
type: Rust Function
title: extract_title
resource: src/core/intent_parser.rs#L116-L121
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

`fn extract_title(content: &str) -> String`

# Called by

- [parse_intent_content](../../../../functions/src/core/intent_parser/parse_intent_content.md)