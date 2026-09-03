---
type: Rust Function
title: scaffold_refactor_report
resource: src/core/tdd.rs#L151-L236
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/tdd/count_pending_tasks
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/tdd_refactor/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/scaffold_refactor_report_warns_with_pending_tasks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/scaffold_refactor_report_no_warning_when_all_done
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/scaffold_refactor_report_has_candidates_checklist
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn scaffold_refactor_report(feature_dir: &Path, feature_id: &str) -> Result<String>`

# Calls

- [count_pending_tasks](../../../../functions/src/core/tdd/count_pending_tasks.md)

# Called by

- [run](../../../../functions/src/cli/tdd_refactor/run.md)
- [scaffold_refactor_report_warns_with_pending_tasks](../../../../functions/src/core/tdd/scaffold_refactor_report_warns_with_pending_tasks.md)
- [scaffold_refactor_report_no_warning_when_all_done](../../../../functions/src/core/tdd/scaffold_refactor_report_no_warning_when_all_done.md)
- [scaffold_refactor_report_has_candidates_checklist](../../../../functions/src/core/tdd/scaffold_refactor_report_has_candidates_checklist.md)