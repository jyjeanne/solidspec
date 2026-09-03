---
type: Rust Function
title: review_intent_alignment
resource: src/core/review/checks.rs#L431-L531
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/parse_intent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/core/review/checks/terse_requirements_are_not_false_medium
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn review_intent_alignment( feature_dir: &Path, spec: &spec_parser::ParsedSpec, ) -> (Vec<ReviewFinding>, f64)`

# Calls

- [parse_intent](../../../../../functions/src/core/intent_parser/parse_intent.md)
- [as_str](../../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [preflight_review](../../../../../functions/src/core/review/preflight_review.md)
- [intent_alignment_zero_score_when_no_intent_md](../../../../../functions/src/core/review/checks/intent_alignment_zero_score_when_no_intent_md.md)
- [intent_alignment_high_finding_when_draft_status](../../../../../functions/src/core/review/checks/intent_alignment_high_finding_when_draft_status.md)
- [intent_alignment_medium_finding_for_untraced_requirement](../../../../../functions/src/core/review/checks/intent_alignment_medium_finding_for_untraced_requirement.md)
- [intent_alignment_perfect_score_when_all_requirements_covered](../../../../../functions/src/core/review/checks/intent_alignment_perfect_score_when_all_requirements_covered.md)
- [parse_error_shows_high_finding_not_not_found_message](../../../../../functions/src/core/review/checks/parse_error_shows_high_finding_not_not_found_message.md)
- [terse_requirements_are_not_false_medium](../../../../../functions/src/core/review/checks/terse_requirements_are_not_false_medium.md)