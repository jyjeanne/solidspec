---
type: Rust Function
title: render_jest
resource: src/core/test_generator.rs#L346-L375
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/test_generator/render_test_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_jest( feature: &str, idx: usize, title: &str, priority: &str, scenarios: &[AcceptanceScenario], ) -> String`

# Called by

- [render_test_file](../../../../functions/src/core/test_generator/render_test_file.md)