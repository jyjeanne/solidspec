---
type: Rust Method
title: display
resource: src/cli/ux.rs#L35-L52
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/ux/step_transitions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/ux/step_detail_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/analyze_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_intent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/parse_spec
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn display(&self) -> String`

# Called by

- [step_transitions](../../../../../functions/src/cli/ux/step_transitions.md)
- [step_detail_text](../../../../../functions/src/cli/ux/step_detail_text.md)
- [analyze_feature](../../../../../functions/src/core/analyzer/analyze_feature.md)
- [parse_intent](../../../../../functions/src/core/intent_parser/parse_intent.md)
- [parse_spec](../../../../../functions/src/core/spec_parser/parse_spec.md)