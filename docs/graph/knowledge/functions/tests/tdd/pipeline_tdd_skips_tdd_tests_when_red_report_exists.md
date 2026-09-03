---
type: Rust Function
title: pipeline_tdd_skips_tdd_tests_when_red_report_exists
resource: tests/tdd.rs#L347-L377
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/tdd/first_feature_dir_after_specify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/seed_tdd_artifacts
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pipeline_tdd_skips_tdd_tests_when_red_report_exists()`

# Calls

- [first_feature_dir_after_specify](../../../functions/tests/tdd/first_feature_dir_after_specify.md)
- [seed_tdd_artifacts](../../../functions/tests/tdd/seed_tdd_artifacts.md)