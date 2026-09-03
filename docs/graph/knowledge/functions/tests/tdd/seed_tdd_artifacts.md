---
type: Rust Function
title: seed_tdd_artifacts
resource: tests/tdd.rs#L1410-L1416
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/tdd/pipeline_tdd_skips_tdd_tests_when_red_report_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/pipeline_tdd_skips_tdd_refactor_when_refactor_report_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/pipeline_force_reruns_tdd_tests_when_red_report_exists
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn seed_tdd_artifacts(feature_dir: &std::path::Path)`

# Called by

- [pipeline_tdd_skips_tdd_tests_when_red_report_exists](../../../functions/tests/tdd/pipeline_tdd_skips_tdd_tests_when_red_report_exists.md)
- [pipeline_tdd_skips_tdd_refactor_when_refactor_report_exists](../../../functions/tests/tdd/pipeline_tdd_skips_tdd_refactor_when_refactor_report_exists.md)
- [pipeline_force_reruns_tdd_tests_when_red_report_exists](../../../functions/tests/tdd/pipeline_force_reruns_tdd_tests_when_red_report_exists.md)