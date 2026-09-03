---
type: Rust Function
title: format_log_entry
resource: src/core/pipeline.rs#L299-L337
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/templates/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/pipeline/write_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/format_log_has_table_and_totals
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_log_entry(results: &[PhaseResult]) -> String`

# Calls

- [all](../../../../functions/src/templates/all.md)

# Called by

- [write_log](../../../../functions/src/core/pipeline/write_log.md)
- [format_log_has_table_and_totals](../../../../functions/src/core/pipeline/format_log_has_table_and_totals.md)