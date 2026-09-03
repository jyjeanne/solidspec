---
type: Rust Function
title: parse_red_report
resource: src/core/tdd.rs#L254-L274
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/tdd/parse_count_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/tdd/parse_red_report_extracts_counts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/tdd/parse_red_report_preserves_colon_in_framework_name
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_red_report(report_path: &Path) -> Result<RedReport>`

# Calls

- [parse_count_line](../../../../functions/src/core/tdd/parse_count_line.md)

# Called by

- [parse_red_report_extracts_counts](../../../../functions/src/core/tdd/parse_red_report_extracts_counts.md)
- [parse_red_report_preserves_colon_in_framework_name](../../../../functions/src/core/tdd/parse_red_report_preserves_colon_in_framework_name.md)