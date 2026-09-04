---
type: Rust Function
title: intent_alignment_perfect_score_when_all_requirements_covered
resource: src/core/review/checks.rs#L722-L737
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/review/checks/minimal_spec_with_reqs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/review_intent_alignment
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn intent_alignment_perfect_score_when_all_requirements_covered()`

# Calls

- [minimal_spec_with_reqs](../../../../../functions/src/core/review/checks/minimal_spec_with_reqs.md)
- [review_intent_alignment](../../../../../functions/src/core/review/checks/review_intent_alignment.md)