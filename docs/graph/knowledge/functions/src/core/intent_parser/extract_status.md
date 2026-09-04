---
type: Rust Function
title: extract_status
resource: src/core/intent_parser.rs#L130-L139
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/from_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/intent_parser/parse_intent_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_status(content: &str) -> IntentStatus`

# Calls

- [from_str](../../../../functions/src/core/intent_parser/IntentStatus/from_str.md)

# Called by

- [parse_intent_content](../../../../functions/src/core/intent_parser/parse_intent_content.md)