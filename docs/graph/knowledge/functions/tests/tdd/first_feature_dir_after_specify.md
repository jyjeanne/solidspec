---
type: Rust Function
title: first_feature_dir_after_specify
resource: tests/tdd.rs#L1401-L1408
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/common/first_feature_dir
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/tdd/pipeline_tdd_skips_tdd_tests_when_red_report_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/pipeline_tdd_skips_tdd_refactor_when_refactor_report_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/pipeline_dry_run_from_tdd_tests_skips_earlier_phases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/pipeline_dry_run_only_tdd_tests_shows_one_phase
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/pipeline_force_reruns_tdd_tests_when_red_report_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/pipeline_dry_run_shows_handoff_label_for_tdd_phases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/pipeline_tdd_phase_numbers_are_correct
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/status_tdd_tests_shows_done_when_tests_dir_nonempty_and_report_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/status_tdd_tests_not_done_when_tests_dir_is_empty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tdd/status_tdd_refactor_shows_done_when_report_exists
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn first_feature_dir_after_specify(dir: &std::path::Path, name: &str) -> std::path::PathBuf`

# Calls

- [first_feature_dir](../../../functions/tests/common/first_feature_dir.md)

# Called by

- [pipeline_tdd_skips_tdd_tests_when_red_report_exists](../../../functions/tests/tdd/pipeline_tdd_skips_tdd_tests_when_red_report_exists.md)
- [pipeline_tdd_skips_tdd_refactor_when_refactor_report_exists](../../../functions/tests/tdd/pipeline_tdd_skips_tdd_refactor_when_refactor_report_exists.md)
- [pipeline_dry_run_from_tdd_tests_skips_earlier_phases](../../../functions/tests/tdd/pipeline_dry_run_from_tdd_tests_skips_earlier_phases.md)
- [pipeline_dry_run_only_tdd_tests_shows_one_phase](../../../functions/tests/tdd/pipeline_dry_run_only_tdd_tests_shows_one_phase.md)
- [pipeline_force_reruns_tdd_tests_when_red_report_exists](../../../functions/tests/tdd/pipeline_force_reruns_tdd_tests_when_red_report_exists.md)
- [pipeline_dry_run_shows_handoff_label_for_tdd_phases](../../../functions/tests/tdd/pipeline_dry_run_shows_handoff_label_for_tdd_phases.md)
- [pipeline_tdd_phase_numbers_are_correct](../../../functions/tests/tdd/pipeline_tdd_phase_numbers_are_correct.md)
- [status_tdd_tests_shows_done_when_tests_dir_nonempty_and_report_exists](../../../functions/tests/tdd/status_tdd_tests_shows_done_when_tests_dir_nonempty_and_report_exists.md)
- [status_tdd_tests_not_done_when_tests_dir_is_empty](../../../functions/tests/tdd/status_tdd_tests_not_done_when_tests_dir_is_empty.md)
- [status_tdd_refactor_shows_done_when_report_exists](../../../functions/tests/tdd/status_tdd_refactor_shows_done_when_report_exists.md)