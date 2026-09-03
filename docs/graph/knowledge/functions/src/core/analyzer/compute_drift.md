---
type: Rust Function
title: compute_drift
resource: src/core/analyzer.rs#L409-L515
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
  - target: functions/src/cli/status/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/analyze_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/drift_zero_at_baseline_all_not_implemented
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/drift_zero_when_no_test_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/drift_detects_unsatisfied_criteria
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/drift_score_100_when_all_criteria_uncovered
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn compute_drift(feature_dir: &Path) -> Option<IntentDrift>`

# Calls

- [parse_intent](../../../../functions/src/core/intent_parser/parse_intent.md)
- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [run](../../../../functions/src/cli/status/run.md)
- [analyze_feature](../../../../functions/src/core/analyzer/analyze_feature.md)
- [drift_zero_at_baseline_all_not_implemented](../../../../functions/src/core/analyzer/drift_zero_at_baseline_all_not_implemented.md)
- [drift_zero_when_no_test_files](../../../../functions/src/core/analyzer/drift_zero_when_no_test_files.md)
- [drift_detects_unsatisfied_criteria](../../../../functions/src/core/analyzer/drift_detects_unsatisfied_criteria.md)
- [drift_score_100_when_all_criteria_uncovered](../../../../functions/src/core/analyzer/drift_score_100_when_all_criteria_uncovered.md)