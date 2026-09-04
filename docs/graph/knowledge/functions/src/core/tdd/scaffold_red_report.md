---
type: Rust Function
title: scaffold_red_report
resource: src/core/tdd.rs#L10-L115
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/tdd/extract_acceptance_criteria
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/extract_task_summary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/build_cycle_sections
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/tdd_tests/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/scaffold_red_report_contains_ac_items
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/scaffold_red_report_graceful_without_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/scaffold_red_report_has_interface_design_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/scaffold_red_report_tracer_bullet_uses_first_ac
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/scaffold_red_report_single_ac_has_no_remaining_cycles
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn scaffold_red_report(feature_dir: &Path, feature_id: &str) -> Result<String>`

# Calls

- [extract_acceptance_criteria](../../../../functions/src/core/tdd/extract_acceptance_criteria.md)
- [extract_task_summary](../../../../functions/src/core/tdd/extract_task_summary.md)
- [build_cycle_sections](../../../../functions/src/core/tdd/build_cycle_sections.md)

# Called by

- [run](../../../../functions/src/cli/tdd_tests/run.md)
- [scaffold_red_report_contains_ac_items](../../../../functions/src/core/tdd/scaffold_red_report_contains_ac_items.md)
- [scaffold_red_report_graceful_without_spec](../../../../functions/src/core/tdd/scaffold_red_report_graceful_without_spec.md)
- [scaffold_red_report_has_interface_design_section](../../../../functions/src/core/tdd/scaffold_red_report_has_interface_design_section.md)
- [scaffold_red_report_tracer_bullet_uses_first_ac](../../../../functions/src/core/tdd/scaffold_red_report_tracer_bullet_uses_first_ac.md)
- [scaffold_red_report_single_ac_has_no_remaining_cycles](../../../../functions/src/core/tdd/scaffold_red_report_single_ac_has_no_remaining_cycles.md)