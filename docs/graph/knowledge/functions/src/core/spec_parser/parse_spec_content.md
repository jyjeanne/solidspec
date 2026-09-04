---
type: Rust Function
title: parse_spec_content
resource: src/core/spec_parser.rs#L65-L78
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/spec_parser/extract_user_stories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/extract_requirements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/extract_clarification_markers
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/review/checks/minimal_spec_with_reqs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/empty_spec_means_no_requirements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/cross_reference_gaps_found
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/task_story_link_gaps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/terse_requirements_are_not_false_medium
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/parse_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/parse_three_stories_with_correct_priorities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/extract_acceptance_scenarios
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/extract_requirements_numbered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/identify_clarification_markers_with_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/multiple_markers_counted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/extract_entities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/empty_spec_handled
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_spec_content(content: &str) -> Result<ParsedSpec>`

# Calls

- [extract_user_stories](../../../../functions/src/core/spec_parser/extract_user_stories.md)
- [extract_requirements](../../../../functions/src/core/spec_parser/extract_requirements.md)
- [extract_clarification_markers](../../../../functions/src/core/spec_parser/extract_clarification_markers.md)

# Called by

- [minimal_spec_with_reqs](../../../../functions/src/core/review/checks/minimal_spec_with_reqs.md)
- [empty_spec_means_no_requirements](../../../../functions/src/core/review/checks/empty_spec_means_no_requirements.md)
- [cross_reference_gaps_found](../../../../functions/src/core/review/checks/cross_reference_gaps_found.md)
- [task_story_link_gaps](../../../../functions/src/core/review/checks/task_story_link_gaps.md)
- [terse_requirements_are_not_false_medium](../../../../functions/src/core/review/checks/terse_requirements_are_not_false_medium.md)
- [parse_spec](../../../../functions/src/core/spec_parser/parse_spec.md)
- [parse_three_stories_with_correct_priorities](../../../../functions/src/core/spec_parser/parse_three_stories_with_correct_priorities.md)
- [extract_acceptance_scenarios](../../../../functions/src/core/spec_parser/extract_acceptance_scenarios.md)
- [extract_requirements_numbered](../../../../functions/src/core/spec_parser/extract_requirements_numbered.md)
- [identify_clarification_markers_with_count](../../../../functions/src/core/spec_parser/identify_clarification_markers_with_count.md)
- [multiple_markers_counted](../../../../functions/src/core/spec_parser/multiple_markers_counted.md)
- [extract_entities](../../../../functions/src/core/spec_parser/extract_entities.md)
- [empty_spec_handled](../../../../functions/src/core/spec_parser/empty_spec_handled.md)