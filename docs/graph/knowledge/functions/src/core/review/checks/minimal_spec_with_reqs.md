---
type: Rust Function
title: minimal_spec_with_reqs
resource: src/core/review/checks.rs#L577-L584
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/spec_parser/parse_spec_content
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/review/checks/intent_alignment_zero_score_when_no_intent_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/intent_alignment_high_finding_when_draft_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/intent_alignment_medium_finding_for_untraced_requirement
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/intent_alignment_perfect_score_when_all_requirements_covered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/parse_error_shows_high_finding_not_not_found_message
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn minimal_spec_with_reqs() -> spec_parser::ParsedSpec`

# Calls

- [parse_spec_content](../../../../../functions/src/core/spec_parser/parse_spec_content.md)

# Called by

- [intent_alignment_zero_score_when_no_intent_md](../../../../../functions/src/core/review/checks/intent_alignment_zero_score_when_no_intent_md.md)
- [intent_alignment_high_finding_when_draft_status](../../../../../functions/src/core/review/checks/intent_alignment_high_finding_when_draft_status.md)
- [intent_alignment_medium_finding_for_untraced_requirement](../../../../../functions/src/core/review/checks/intent_alignment_medium_finding_for_untraced_requirement.md)
- [intent_alignment_perfect_score_when_all_requirements_covered](../../../../../functions/src/core/review/checks/intent_alignment_perfect_score_when_all_requirements_covered.md)
- [parse_error_shows_high_finding_not_not_found_message](../../../../../functions/src/core/review/checks/parse_error_shows_high_finding_not_not_found_message.md)