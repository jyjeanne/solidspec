---
type: Rust Module
title: report
resource: src/core/review/report.rs#L1-L160
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-dimension-reviewreport-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-review-dimensionscore-reviewfinding
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [format_review_report](../../../../functions/src/core/review/report/format_review_report.md)
- [format_report_renders_markdown](../../../../functions/src/core/review/report/format_report_renders_markdown.md)

# Imports

- `super::{Dimension, ReviewReport, Severity}`
- `super::*`
- `crate::core::review::{DimensionScore, ReviewFinding}`

# Member of

- [solidspec](../../../../packages/solidspec.md)