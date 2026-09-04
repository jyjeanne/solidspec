---
type: Rust Function
title: write_test_file
resource: src/core/evidence.rs#L228-L233
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
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
---

# Signature

`fn write_test_file(dir: &Path, name: &str, status: &str, body: &str)`

# Called by

- [baseline_all_not_implemented](../../../../functions/src/core/evidence/baseline_all_not_implemented.md)
- [satisfied_criterion_detected](../../../../functions/src/core/evidence/satisfied_criterion_detected.md)
- [satisfaction_rate_100_gives_satisfied_status](../../../../functions/src/core/evidence/satisfaction_rate_100_gives_satisfied_status.md)
- [low_satisfaction_gives_drifted_status](../../../../functions/src/core/evidence/low_satisfaction_gives_drifted_status.md)