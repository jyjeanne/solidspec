---
type: Rust Function
title: derive_score_from_keywords
resource: src/core/fan_out.rs#L385-L392
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/fan_out/parse_severity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/penalty_weight
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/fan_out/extract_score
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn derive_score_from_keywords(output: &str) -> u8`

# Calls

- [parse_severity](../../../../functions/src/core/fan_out/parse_severity.md)
- [penalty_weight](../../../../functions/src/core/fan_out/penalty_weight.md)

# Called by

- [extract_score](../../../../functions/src/core/fan_out/extract_score.md)