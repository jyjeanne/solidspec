---
type: Rust Function
title: count_pending_tasks
resource: src/core/tdd.rs#L350-L355
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/tdd/scaffold_refactor_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn count_pending_tasks(feature_dir: &Path) -> usize`

# Called by

- [scaffold_refactor_report](../../../../functions/src/core/tdd/scaffold_refactor_report.md)