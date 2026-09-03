---
type: Rust Function
title: parse_count_line
resource: src/core/tdd.rs#L276-L289
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/tdd/parse_red_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_count_line(content: &str, label: &str) -> usize`

# Called by

- [parse_red_report](../../../../functions/src/core/tdd/parse_red_report.md)