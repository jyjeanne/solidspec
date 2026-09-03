---
type: Rust Function
title: extract_list_items
resource: src/core/intent_parser.rs#L165-L188
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

`fn extract_list_items(content: &str, section: &str) -> Vec<String>`

# Calls

- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [parse_intent_content](../../../../functions/src/core/intent_parser/parse_intent_content.md)