---
type: Rust Module
title: ux
resource: src/cli/ux.rs#L1-L145
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/console-style
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

- [StepStatus](../../../classes/src/cli/ux/StepStatus.md)
- [Step](../../../classes/src/cli/ux/Step.md)
- [new](../../../functions/src/cli/ux/Step/new.md)
- [display](../../../functions/src/cli/ux/Step/display.md)
- [StepTracker](../../../classes/src/cli/ux/StepTracker.md)
- [new](../../../functions/src/cli/ux/StepTracker/new.md)
- [add](../../../functions/src/cli/ux/StepTracker/add.md)
- [set_status](../../../functions/src/cli/ux/StepTracker/set_status.md)
- [set_detail](../../../functions/src/cli/ux/StepTracker/set_detail.md)
- [print_all](../../../functions/src/cli/ux/StepTracker/print_all.md)
- [step_transitions](../../../functions/src/cli/ux/step_transitions.md)
- [step_detail_text](../../../functions/src/cli/ux/step_detail_text.md)
- [step_tracker_add_and_update](../../../functions/src/cli/ux/step_tracker_add_and_update.md)
- [step_tracker_out_of_bounds_noop](../../../functions/src/cli/ux/step_tracker_out_of_bounds_noop.md)

# Imports

- `console::Style`
- `super::*`

# Member of

- [solidspec](../../../packages/solidspec.md)