---
type: Rust Module
title: tdd
resource: src/core/tdd.rs#L1-L596
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [scaffold_red_report](../../../functions/src/core/tdd/scaffold_red_report.md)
- [build_cycle_sections](../../../functions/src/core/tdd/build_cycle_sections.md)
- [scaffold_refactor_report](../../../functions/src/core/tdd/scaffold_refactor_report.md)
- [RedReport](../../../classes/src/core/tdd/RedReport.md)
- [parse_red_report](../../../functions/src/core/tdd/parse_red_report.md)
- [parse_count_line](../../../functions/src/core/tdd/parse_count_line.md)
- [extract_acceptance_criteria](../../../functions/src/core/tdd/extract_acceptance_criteria.md)
- [extract_task_summary](../../../functions/src/core/tdd/extract_task_summary.md)
- [count_pending_tasks](../../../functions/src/core/tdd/count_pending_tasks.md)
- [write_spec_with_ac](../../../functions/src/core/tdd/write_spec_with_ac.md)
- [scaffold_red_report_contains_ac_items](../../../functions/src/core/tdd/scaffold_red_report_contains_ac_items.md)
- [scaffold_red_report_graceful_without_spec](../../../functions/src/core/tdd/scaffold_red_report_graceful_without_spec.md)
- [scaffold_refactor_report_warns_with_pending_tasks](../../../functions/src/core/tdd/scaffold_refactor_report_warns_with_pending_tasks.md)
- [scaffold_refactor_report_no_warning_when_all_done](../../../functions/src/core/tdd/scaffold_refactor_report_no_warning_when_all_done.md)
- [parse_red_report_extracts_counts](../../../functions/src/core/tdd/parse_red_report_extracts_counts.md)
- [parse_red_report_preserves_colon_in_framework_name](../../../functions/src/core/tdd/parse_red_report_preserves_colon_in_framework_name.md)
- [scaffold_red_report_has_interface_design_section](../../../functions/src/core/tdd/scaffold_red_report_has_interface_design_section.md)
- [scaffold_red_report_tracer_bullet_uses_first_ac](../../../functions/src/core/tdd/scaffold_red_report_tracer_bullet_uses_first_ac.md)
- [scaffold_red_report_single_ac_has_no_remaining_cycles](../../../functions/src/core/tdd/scaffold_red_report_single_ac_has_no_remaining_cycles.md)
- [scaffold_refactor_report_has_candidates_checklist](../../../functions/src/core/tdd/scaffold_refactor_report_has_candidates_checklist.md)
- [extract_criteria_handles_subsection_headers](../../../functions/src/core/tdd/extract_criteria_handles_subsection_headers.md)

# Imports

- `std::path::Path`
- `anyhow::Result`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)