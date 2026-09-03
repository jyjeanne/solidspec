---
type: Rust Function
title: parse_severity
resource: src/core/fan_out.rs#L461-L469
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/fan_out/derive_score_from_keywords
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/parse_findings_from_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_severity(s: &str) -> Option<Severity>`

# Called by

- [derive_score_from_keywords](../../../../functions/src/core/fan_out/derive_score_from_keywords.md)
- [parse_findings_from_output](../../../../functions/src/core/fan_out/parse_findings_from_output.md)