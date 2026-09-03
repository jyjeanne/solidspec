---
type: Rust Function
title: parse_intent
resource: src/core/intent_parser.rs#L90-L97
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/cli/ux/Step/display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_intent_content
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/compute_drift
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/collect_evidence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/review_intent_alignment
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_intent(path: &Path) -> Result<IntentSpec>`

# Calls

- [display](../../../../functions/src/cli/ux/Step/display.md)
- [parse_intent_content](../../../../functions/src/core/intent_parser/parse_intent_content.md)

# Called by

- [run](../../../../functions/src/cli/plan/run.md)
- [compute_drift](../../../../functions/src/core/analyzer/compute_drift.md)
- [collect_evidence](../../../../functions/src/core/evidence/collect_evidence.md)
- [review_intent_alignment](../../../../functions/src/core/review/checks/review_intent_alignment.md)