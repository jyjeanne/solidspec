---
type: Rust Function
title: parse_spec
resource: src/core/spec_parser.rs#L55-L63
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/cli/ux/Step/display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/parse_spec_content
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/clarify/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tasks/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tests_cmd/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/analyze_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_spec(path: &Path) -> Result<ParsedSpec>`

# Calls

- [display](../../../../functions/src/cli/ux/Step/display.md)
- [parse_spec_content](../../../../functions/src/core/spec_parser/parse_spec_content.md)

# Called by

- [run](../../../../functions/src/cli/clarify/run.md)
- [run](../../../../functions/src/cli/plan/run.md)
- [run](../../../../functions/src/cli/tasks/run.md)
- [run](../../../../functions/src/cli/tests_cmd/run.md)
- [analyze_feature](../../../../functions/src/core/analyzer/analyze_feature.md)
- [should_skip](../../../../functions/src/core/pipeline/should_skip.md)
- [preflight_review](../../../../functions/src/core/review/preflight_review.md)