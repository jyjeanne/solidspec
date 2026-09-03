---
type: Rust Function
title: extract_clarification_markers
resource: src/core/spec_parser.rs#L119-L135
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/spec_parser/parse_spec_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_clarification_markers(content: &str) -> Vec<ClarificationMarker>`

# Calls

- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [parse_spec_content](../../../../functions/src/core/spec_parser/parse_spec_content.md)