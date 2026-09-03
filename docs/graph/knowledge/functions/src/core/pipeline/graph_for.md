---
type: Rust Function
title: graph_for
resource: src/core/pipeline.rs#L361-L364
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/schema/resolve_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/WorkflowSchema/into_graph
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/pipeline/should_skip_specify_when_spec_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_clarify_when_no_markers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_not_skip_clarify_when_markers_present
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_implement_when_all_tasks_done
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_not_skip_implement_when_tasks_pending
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/analyze_never_skipped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_ship_when_report_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_not_skip_ship_when_report_absent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_evidence_when_report_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_not_skip_evidence_when_absent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_intent_when_intent_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_not_skip_intent_when_absent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_apex_false_when_no_apex_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_apex_false_when_apex_dir_empty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_apex_false_when_run_dir_has_no_finish
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_apex_true_when_finish_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_apex_false_when_force
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_apex_ignores_file_entries_in_apex_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_tests_when_dir_nonempty_via_schema_generates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_unknown_phase_defaults_false_when_absent_from_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip_security_review_when_report_exists
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn graph_for(schema_name: &str) -> ArtifactGraph`

# Calls

- [resolve_schema](../../../../functions/src/core/schema/resolve_schema.md)
- [into_graph](../../../../functions/src/core/schema/WorkflowSchema/into_graph.md)

# Called by

- [should_skip_specify_when_spec_exists](../../../../functions/src/core/pipeline/should_skip_specify_when_spec_exists.md)
- [should_skip_clarify_when_no_markers](../../../../functions/src/core/pipeline/should_skip_clarify_when_no_markers.md)
- [should_not_skip_clarify_when_markers_present](../../../../functions/src/core/pipeline/should_not_skip_clarify_when_markers_present.md)
- [should_skip_implement_when_all_tasks_done](../../../../functions/src/core/pipeline/should_skip_implement_when_all_tasks_done.md)
- [should_not_skip_implement_when_tasks_pending](../../../../functions/src/core/pipeline/should_not_skip_implement_when_tasks_pending.md)
- [analyze_never_skipped](../../../../functions/src/core/pipeline/analyze_never_skipped.md)
- [should_skip_ship_when_report_exists](../../../../functions/src/core/pipeline/should_skip_ship_when_report_exists.md)
- [should_not_skip_ship_when_report_absent](../../../../functions/src/core/pipeline/should_not_skip_ship_when_report_absent.md)
- [should_skip_evidence_when_report_exists](../../../../functions/src/core/pipeline/should_skip_evidence_when_report_exists.md)
- [should_not_skip_evidence_when_absent](../../../../functions/src/core/pipeline/should_not_skip_evidence_when_absent.md)
- [should_skip_intent_when_intent_exists](../../../../functions/src/core/pipeline/should_skip_intent_when_intent_exists.md)
- [should_not_skip_intent_when_absent](../../../../functions/src/core/pipeline/should_not_skip_intent_when_absent.md)
- [should_skip_apex_false_when_no_apex_dir](../../../../functions/src/core/pipeline/should_skip_apex_false_when_no_apex_dir.md)
- [should_skip_apex_false_when_apex_dir_empty](../../../../functions/src/core/pipeline/should_skip_apex_false_when_apex_dir_empty.md)
- [should_skip_apex_false_when_run_dir_has_no_finish](../../../../functions/src/core/pipeline/should_skip_apex_false_when_run_dir_has_no_finish.md)
- [should_skip_apex_true_when_finish_exists](../../../../functions/src/core/pipeline/should_skip_apex_true_when_finish_exists.md)
- [should_skip_apex_false_when_force](../../../../functions/src/core/pipeline/should_skip_apex_false_when_force.md)
- [should_skip_apex_ignores_file_entries_in_apex_dir](../../../../functions/src/core/pipeline/should_skip_apex_ignores_file_entries_in_apex_dir.md)
- [should_skip_tests_when_dir_nonempty_via_schema_generates](../../../../functions/src/core/pipeline/should_skip_tests_when_dir_nonempty_via_schema_generates.md)
- [should_skip_unknown_phase_defaults_false_when_absent_from_schema](../../../../functions/src/core/pipeline/should_skip_unknown_phase_defaults_false_when_absent_from_schema.md)
- [should_skip_security_review_when_report_exists](../../../../functions/src/core/pipeline/should_skip_security_review_when_report_exists.md)