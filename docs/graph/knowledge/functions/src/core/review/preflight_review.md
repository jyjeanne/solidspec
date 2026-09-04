---
type: Rust Function
title: preflight_review
resource: src/core/review.rs#L95-L212
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/spec_parser/parse_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_placeholders
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_section_completeness
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_ambiguous_language
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_requirement_quality
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_scenario_coverage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_cross_references
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_task_story_links
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_test_coverage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_security_hints
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/review_intent_alignment
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/score_dimensions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/review/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/score_from_heuristics
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/good_spec_scores_high
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/stories_without_scenarios_flagged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/review_is_read_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/preflight_review_includes_intent_alignment_dimension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/format_report_contains_intent_alignment_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/intent_alignment_section_shows_traced_when_all_good
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/no_issues_message_suppressed_when_ia_has_findings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn preflight_review(feature_dir: &Path, _project_root: &Path) -> Result<ReviewReport>`

# Calls

- [parse_spec](../../../../functions/src/core/spec_parser/parse_spec.md)
- [check_placeholders](../../../../functions/src/core/review/checks/check_placeholders.md)
- [check_section_completeness](../../../../functions/src/core/review/checks/check_section_completeness.md)
- [check_ambiguous_language](../../../../functions/src/core/review/checks/check_ambiguous_language.md)
- [check_requirement_quality](../../../../functions/src/core/review/checks/check_requirement_quality.md)
- [check_scenario_coverage](../../../../functions/src/core/review/checks/check_scenario_coverage.md)
- [check_cross_references](../../../../functions/src/core/review/checks/check_cross_references.md)
- [check_task_story_links](../../../../functions/src/core/review/checks/check_task_story_links.md)
- [check_test_coverage](../../../../functions/src/core/review/checks/check_test_coverage.md)
- [check_security_hints](../../../../functions/src/core/review/checks/check_security_hints.md)
- [review_intent_alignment](../../../../functions/src/core/review/checks/review_intent_alignment.md)
- [score_dimensions](../../../../functions/src/core/review/score_dimensions.md)

# Called by

- [run](../../../../functions/src/cli/review/run.md)
- [score_from_heuristics](../../../../functions/src/core/fan_out/score_from_heuristics.md)
- [good_spec_scores_high](../../../../functions/src/core/review/good_spec_scores_high.md)
- [stories_without_scenarios_flagged](../../../../functions/src/core/review/stories_without_scenarios_flagged.md)
- [review_is_read_only](../../../../functions/src/core/review/review_is_read_only.md)
- [preflight_review_includes_intent_alignment_dimension](../../../../functions/src/core/review/preflight_review_includes_intent_alignment_dimension.md)
- [format_report_contains_intent_alignment_section](../../../../functions/src/core/review/format_report_contains_intent_alignment_section.md)
- [intent_alignment_section_shows_traced_when_all_good](../../../../functions/src/core/review/intent_alignment_section_shows_traced_when_all_good.md)
- [no_issues_message_suppressed_when_ia_has_findings](../../../../functions/src/core/review/no_issues_message_suppressed_when_ia_has_findings.md)