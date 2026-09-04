---
type: Rust Function
title: setup_full_idsd_feature
resource: tests/traceability.rs#L46-L183
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/traceability/analyze_prints_traceability_chain_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/traceability/trace_tree_shows_task_to_test_links
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/traceability/orphaned_requirement_produces_high_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/traceability/analyze_shows_intent_coverage_with_intent_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/traceability/evidence_update_reflects_in_intent_md_status
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn setup_full_idsd_feature(project_root: &std::path::Path) -> std::path::PathBuf`

# Called by

- [analyze_prints_traceability_chain_tree](../../../functions/tests/traceability/analyze_prints_traceability_chain_tree.md)
- [trace_tree_shows_task_to_test_links](../../../functions/tests/traceability/trace_tree_shows_task_to_test_links.md)
- [orphaned_requirement_produces_high_finding](../../../functions/tests/traceability/orphaned_requirement_produces_high_finding.md)
- [analyze_shows_intent_coverage_with_intent_md](../../../functions/tests/traceability/analyze_shows_intent_coverage_with_intent_md.md)
- [evidence_update_reflects_in_intent_md_status](../../../functions/tests/traceability/evidence_update_reflects_in_intent_md_status.md)