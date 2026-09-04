---
type: Rust Function
title: extract_acceptance_criteria
resource: src/core/tdd.rs#L293-L337
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/tdd/scaffold_red_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/extract_criteria_handles_subsection_headers
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_acceptance_criteria(feature_dir: &Path) -> Vec<String>`

# Called by

- [scaffold_red_report](../../../../functions/src/core/tdd/scaffold_red_report.md)
- [extract_criteria_handles_subsection_headers](../../../../functions/src/core/tdd/extract_criteria_handles_subsection_headers.md)