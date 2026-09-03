---
type: Rust Method
title: sorted_priorities
resource: src/presets/registry.rs#L75-L83
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/presets/manager/get_preset_priorities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/registry/sorted_priorities_for_resolver
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn sorted_priorities(&self) -> Vec<(String, u32)>`

# Called by

- [get_preset_priorities](../../../../../functions/src/presets/manager/get_preset_priorities.md)
- [sorted_priorities_for_resolver](../../../../../functions/src/presets/registry/sorted_priorities_for_resolver.md)