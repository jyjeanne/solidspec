---
type: Rust Function
title: render_go
resource: src/core/test_generator.rs#L455-L496
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/test_generator/render_test_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_go( feature: &str, idx: usize, title: &str, priority: &str, scenarios: &[AcceptanceScenario], ) -> String`

# Calls

- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [render_test_file](../../../../functions/src/core/test_generator/render_test_file.md)