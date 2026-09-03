---
type: Rust Function
title: check_intent_constraints
resource: src/core/constitution.rs#L165-L214
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn check_intent_constraints( intent: &super::intent_parser::IntentSpec, plan_content: &str, ) -> GateResult`

# Calls

- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [run](../../../../functions/src/cli/plan/run.md)