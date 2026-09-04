---
type: Rust Function
title: parse_single_scenario
resource: src/core/test_generator.rs#L47-L91
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/test_generator/extract_scenarios
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_single_scenario( raw: &str, story_index: usize, story_title: &str, story_priority: &str, ) -> Option<AcceptanceScenario>`

# Called by

- [extract_scenarios](../../../../functions/src/core/test_generator/extract_scenarios.md)