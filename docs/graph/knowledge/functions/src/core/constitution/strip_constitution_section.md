---
type: Rust Function
title: strip_constitution_section
resource: src/core/constitution.rs#L95-L114
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/constitution/check_plan_compliance
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn strip_constitution_section(content: &str) -> String`

# Called by

- [check_plan_compliance](../../../../functions/src/core/constitution/check_plan_compliance.md)