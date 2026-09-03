---
type: Rust Function
title: setup_constitution
resource: src/core/analyzer.rs#L456-L463
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/analyzer/fully_traced_artifacts_high_score
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/missing_plan_is_high_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/orphan_tasks_medium_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/constitution_violation_is_critical
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/analyze_does_not_modify_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/remediation_suggestions_present
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn setup_constitution(project_root: &Path)`

# Called by

- [fully_traced_artifacts_high_score](../../../../functions/src/core/analyzer/fully_traced_artifacts_high_score.md)
- [missing_plan_is_high_finding](../../../../functions/src/core/analyzer/missing_plan_is_high_finding.md)
- [orphan_tasks_medium_finding](../../../../functions/src/core/analyzer/orphan_tasks_medium_finding.md)
- [constitution_violation_is_critical](../../../../functions/src/core/analyzer/constitution_violation_is_critical.md)
- [analyze_does_not_modify_files](../../../../functions/src/core/analyzer/analyze_does_not_modify_files.md)
- [remediation_suggestions_present](../../../../functions/src/core/analyzer/remediation_suggestions_present.md)