---
type: Rust Module
title: spec_parser
resource: src/core/spec_parser.rs#L1-L349
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
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-errors-solidspecerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ParsedSpec](../../../classes/src/core/spec_parser/ParsedSpec.md)
- [UserStory](../../../classes/src/core/spec_parser/UserStory.md)
- [Requirement](../../../classes/src/core/spec_parser/Requirement.md)
- [ClarificationMarker](../../../classes/src/core/spec_parser/ClarificationMarker.md)
- [parse_spec](../../../functions/src/core/spec_parser/parse_spec.md)
- [parse_spec_content](../../../functions/src/core/spec_parser/parse_spec_content.md)
- [extract_user_stories](../../../functions/src/core/spec_parser/extract_user_stories.md)
- [extract_requirements](../../../functions/src/core/spec_parser/extract_requirements.md)
- [extract_clarification_markers](../../../functions/src/core/spec_parser/extract_clarification_markers.md)
- [extract_entities](../../../functions/src/core/spec_parser/extract_entities.md)
- [extract_entities_with_descriptions](../../../functions/src/core/spec_parser/extract_entities_with_descriptions.md)
- [validate_spec_quality](../../../functions/src/core/spec_parser/validate_spec_quality.md)
- [parse_three_stories_with_correct_priorities](../../../functions/src/core/spec_parser/parse_three_stories_with_correct_priorities.md)
- [extract_acceptance_scenarios](../../../functions/src/core/spec_parser/extract_acceptance_scenarios.md)
- [extract_requirements_numbered](../../../functions/src/core/spec_parser/extract_requirements_numbered.md)
- [identify_clarification_markers_with_count](../../../functions/src/core/spec_parser/identify_clarification_markers_with_count.md)
- [multiple_markers_counted](../../../functions/src/core/spec_parser/multiple_markers_counted.md)
- [extract_entities](../../../functions/src/core/spec_parser/extract_entities-2.md)
- [empty_spec_handled](../../../functions/src/core/spec_parser/empty_spec_handled.md)
- [validate_spec_detects_impl_details](../../../functions/src/core/spec_parser/validate_spec_detects_impl_details.md)
- [validate_spec_clean_passes](../../../functions/src/core/spec_parser/validate_spec_clean_passes.md)
- [validate_spec_too_many_markers](../../../functions/src/core/spec_parser/validate_spec_too_many_markers.md)
- [extract_entities_with_descriptions](../../../functions/src/core/spec_parser/extract_entities_with_descriptions-2.md)
- [extract_entities_with_empty_description](../../../functions/src/core/spec_parser/extract_entities_with_empty_description.md)

# Imports

- `std::path::Path`
- `std::sync::LazyLock`
- `anyhow::Result`
- `regex::Regex`
- `super::errors::SolidSpecError`
- `super::*`

# Member of

- [solidspec](../../../packages/solidspec.md)