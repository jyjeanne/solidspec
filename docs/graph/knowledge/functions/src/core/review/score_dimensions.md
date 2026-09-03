---
type: Rust Function
title: score_dimensions
resource: src/core/review.rs#L215-L251
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/scoring_penalizes_critical_findings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn score_dimensions(findings: &[ReviewFinding]) -> Vec<DimensionScore>`

# Called by

- [preflight_review](../../../../functions/src/core/review/preflight_review.md)
- [scoring_penalizes_critical_findings](../../../../functions/src/core/review/scoring_penalizes_critical_findings.md)