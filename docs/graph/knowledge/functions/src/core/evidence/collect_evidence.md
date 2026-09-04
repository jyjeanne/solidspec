---
type: Rust Function
title: collect_evidence
resource: src/core/evidence.rs#L35-L142
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
  - target: functions/src/cli/evidence/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/analyze_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/baseline_all_not_implemented
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/satisfied_criterion_detected
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/satisfaction_rate_100_gives_satisfied_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/low_satisfaction_gives_drifted_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/no_tests_dir_returns_baseline
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn collect_evidence(feature_dir: &Path) -> Result<EvidenceReport>`

# Calls

- [parse_intent](../../../../functions/src/core/intent_parser/parse_intent.md)
- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [run](../../../../functions/src/cli/evidence/run.md)
- [analyze_feature](../../../../functions/src/core/analyzer/analyze_feature.md)
- [baseline_all_not_implemented](../../../../functions/src/core/evidence/baseline_all_not_implemented.md)
- [satisfied_criterion_detected](../../../../functions/src/core/evidence/satisfied_criterion_detected.md)
- [satisfaction_rate_100_gives_satisfied_status](../../../../functions/src/core/evidence/satisfaction_rate_100_gives_satisfied_status.md)
- [low_satisfaction_gives_drifted_status](../../../../functions/src/core/evidence/low_satisfaction_gives_drifted_status.md)
- [no_tests_dir_returns_baseline](../../../../functions/src/core/evidence/no_tests_dir_returns_baseline.md)