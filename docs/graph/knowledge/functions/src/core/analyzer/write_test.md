---
type: Rust Function
title: write_test
resource: src/core/analyzer.rs#L765-L769
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/analyzer/drift_zero_at_baseline_all_not_implemented
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

`fn write_test(dir: &Path, name: &str, content: &str)`

# Called by

- [drift_zero_at_baseline_all_not_implemented](../../../../functions/src/core/analyzer/drift_zero_at_baseline_all_not_implemented.md)
- [drift_detects_unsatisfied_criteria](../../../../functions/src/core/analyzer/drift_detects_unsatisfied_criteria.md)
- [drift_score_100_when_all_criteria_uncovered](../../../../functions/src/core/analyzer/drift_score_100_when_all_criteria_uncovered.md)