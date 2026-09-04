---
type: Rust Function
title: parse_intent_content
resource: src/core/intent_parser.rs#L100-L114
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/extract_title
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/extract_inline_field
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/extract_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/extract_section_body
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/extract_list_items
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/intent_parser/parse_intent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_title
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_intent_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_status_active
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_goal
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_constraints_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_evidence_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_risks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_open_questions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_empty_content_returns_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/raw_field_preserved
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_intent_content(content: &str) -> Result<IntentSpec>`

# Calls

- [extract_title](../../../../functions/src/core/intent_parser/extract_title.md)
- [extract_inline_field](../../../../functions/src/core/intent_parser/extract_inline_field.md)
- [extract_status](../../../../functions/src/core/intent_parser/extract_status.md)
- [extract_section_body](../../../../functions/src/core/intent_parser/extract_section_body.md)
- [extract_list_items](../../../../functions/src/core/intent_parser/extract_list_items.md)

# Called by

- [parse_intent](../../../../functions/src/core/intent_parser/parse_intent.md)
- [parse_title](../../../../functions/src/core/intent_parser/parse_title.md)
- [parse_intent_id](../../../../functions/src/core/intent_parser/parse_intent_id.md)
- [parse_feature](../../../../functions/src/core/intent_parser/parse_feature.md)
- [parse_status_active](../../../../functions/src/core/intent_parser/parse_status_active.md)
- [parse_goal](../../../../functions/src/core/intent_parser/parse_goal.md)
- [parse_constraints_count](../../../../functions/src/core/intent_parser/parse_constraints_count.md)
- [parse_evidence_count](../../../../functions/src/core/intent_parser/parse_evidence_count.md)
- [parse_risks](../../../../functions/src/core/intent_parser/parse_risks.md)
- [parse_open_questions](../../../../functions/src/core/intent_parser/parse_open_questions.md)
- [parse_empty_content_returns_defaults](../../../../functions/src/core/intent_parser/parse_empty_content_returns_defaults.md)
- [raw_field_preserved](../../../../functions/src/core/intent_parser/raw_field_preserved.md)