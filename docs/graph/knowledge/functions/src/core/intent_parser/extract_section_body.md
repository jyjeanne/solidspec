---
type: Rust Function
title: extract_section_body
resource: src/core/intent_parser.rs#L142-L162
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/intent_parser/parse_intent_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_section_body(content: &str, section: &str) -> String`

# Calls

- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [parse_intent_content](../../../../functions/src/core/intent_parser/parse_intent_content.md)