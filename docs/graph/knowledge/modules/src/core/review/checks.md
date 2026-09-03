---
type: Rust Module
title: checks
resource: src/core/review/checks.rs#L1-L799
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-lazylock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-dimension-reviewfinding-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-intent-parser-spec-parser
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
- [setup_intent](../../../../functions/src/core/review/checks/setup_intent.md)
- [minimal_spec_with_reqs](../../../../functions/src/core/review/checks/minimal_spec_with_reqs.md)
- [placeholder_detection](../../../../functions/src/core/review/checks/placeholder_detection.md)
- [missing_sections_detected](../../../../functions/src/core/review/checks/missing_sections_detected.md)
- [ambiguous_language_flagged](../../../../functions/src/core/review/checks/ambiguous_language_flagged.md)
- [empty_spec_means_no_requirements](../../../../functions/src/core/review/checks/empty_spec_means_no_requirements.md)
- [cross_reference_gaps_found](../../../../functions/src/core/review/checks/cross_reference_gaps_found.md)
- [task_story_link_gaps](../../../../functions/src/core/review/checks/task_story_link_gaps.md)
- [intent_alignment_zero_score_when_no_intent_md](../../../../functions/src/core/review/checks/intent_alignment_zero_score_when_no_intent_md.md)
- [intent_alignment_high_finding_when_draft_status](../../../../functions/src/core/review/checks/intent_alignment_high_finding_when_draft_status.md)
- [intent_alignment_medium_finding_for_untraced_requirement](../../../../functions/src/core/review/checks/intent_alignment_medium_finding_for_untraced_requirement.md)
- [intent_alignment_perfect_score_when_all_requirements_covered](../../../../functions/src/core/review/checks/intent_alignment_perfect_score_when_all_requirements_covered.md)
- [parse_error_shows_high_finding_not_not_found_message](../../../../functions/src/core/review/checks/parse_error_shows_high_finding_not_not_found_message.md)
- [terse_requirements_are_not_false_medium](../../../../functions/src/core/review/checks/terse_requirements_are_not_false_medium.md)

# Imports

- `std::path::Path`
- `std::sync::LazyLock`
- `regex::Regex`
- `super::{Dimension, ReviewFinding, Severity}`
- `crate::core::{intent_parser, spec_parser}`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../../packages/solidspec.md)