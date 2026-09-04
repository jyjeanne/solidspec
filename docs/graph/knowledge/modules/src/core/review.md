---
type: Rust Module
title: review
resource: src/core/review.rs#L1-L533
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-errors-solidspecerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-spec-parser
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-report-format-review-report
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Dimension](../../../classes/src/core/review/Dimension.md)
- [fmt](../../../functions/src/core/review/Dimension/std-fmt-display/fmt.md)
- [Severity](../../../classes/src/core/review/Severity.md)
- [fmt](../../../functions/src/core/review/Severity/std-fmt-display/fmt.md)
- [ReviewFinding](../../../classes/src/core/review/ReviewFinding.md)
- [DimensionScore](../../../classes/src/core/review/DimensionScore.md)
- [ReviewReport](../../../classes/src/core/review/ReviewReport.md)
- [preflight_review](../../../functions/src/core/review/preflight_review.md)
- [score_dimensions](../../../functions/src/core/review/score_dimensions.md)
- [setup_feature](../../../functions/src/core/review/setup_feature.md)
- [setup_project](../../../functions/src/core/review/setup_project.md)
- [good_spec_scores_high](../../../functions/src/core/review/good_spec_scores_high.md)
- [stories_without_scenarios_flagged](../../../functions/src/core/review/stories_without_scenarios_flagged.md)
- [missing_spec_returns_error](../../../functions/src/core/review/missing_spec_returns_error.md)
- [review_is_read_only](../../../functions/src/core/review/review_is_read_only.md)
- [setup_intent](../../../functions/src/core/review/setup_intent.md)
- [preflight_review_includes_intent_alignment_dimension](../../../functions/src/core/review/preflight_review_includes_intent_alignment_dimension.md)
- [format_report_contains_intent_alignment_section](../../../functions/src/core/review/format_report_contains_intent_alignment_section.md)
- [intent_alignment_section_shows_traced_when_all_good](../../../functions/src/core/review/intent_alignment_section_shows_traced_when_all_good.md)
- [no_issues_message_suppressed_when_ia_has_findings](../../../functions/src/core/review/no_issues_message_suppressed_when_ia_has_findings.md)
- [scoring_penalizes_critical_findings](../../../functions/src/core/review/scoring_penalizes_critical_findings.md)

# Imports

- `std::path::Path`
- `anyhow::Result`
- `super::errors::SolidSpecError`
- `super::spec_parser`
- `pub use report::format_review_report`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)