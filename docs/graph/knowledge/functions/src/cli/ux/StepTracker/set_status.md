---
type: Rust Method
title: set_status
resource: src/cli/ux.rs#L71-L75
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/ux/step_tracker_add_and_update
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/ux/step_tracker_out_of_bounds_noop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_status(&mut self, idx: usize, status: StepStatus)`

# Called by

- [step_tracker_add_and_update](../../../../../functions/src/cli/ux/step_tracker_add_and_update.md)
- [step_tracker_out_of_bounds_noop](../../../../../functions/src/cli/ux/step_tracker_out_of_bounds_noop.md)