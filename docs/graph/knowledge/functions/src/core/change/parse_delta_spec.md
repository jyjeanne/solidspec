---
type: Rust Function
title: parse_delta_spec
resource: src/core/change.rs#L104-L114
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/change/extract_added
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/extract_modified
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/extract_removed
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/change/archive_change
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/parse_added_requirements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/parse_modified_requirements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/parse_modified_with_multibyte_text_and_uppercase_was
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/parse_removed_requirements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/empty_delta_parsed
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_delta_spec(content: &str) -> DeltaSpec`

# Calls

- [extract_added](../../../../functions/src/core/change/extract_added.md)
- [extract_modified](../../../../functions/src/core/change/extract_modified.md)
- [extract_removed](../../../../functions/src/core/change/extract_removed.md)

# Called by

- [archive_change](../../../../functions/src/core/change/archive_change.md)
- [parse_added_requirements](../../../../functions/src/core/change/parse_added_requirements.md)
- [parse_modified_requirements](../../../../functions/src/core/change/parse_modified_requirements.md)
- [parse_modified_with_multibyte_text_and_uppercase_was](../../../../functions/src/core/change/parse_modified_with_multibyte_text_and_uppercase_was.md)
- [parse_removed_requirements](../../../../functions/src/core/change/parse_removed_requirements.md)
- [empty_delta_parsed](../../../../functions/src/core/change/empty_delta_parsed.md)