---
type: Rust Function
title: get_preset_priorities
resource: src/presets/manager.rs#L87-L91
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/presets/registry/PresetRegistry/sorted_priorities
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/checklist/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/intent/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/run_for_existing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/priorities_for_resolver
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_preset_priorities(project_root: &Path) -> Result<Vec<(String, u32)>>`

# Calls

- [sorted_priorities](../../../../functions/src/presets/registry/PresetRegistry/sorted_priorities.md)

# Called by

- [run](../../../../functions/src/cli/checklist/run.md)
- [run](../../../../functions/src/cli/intent/run.md)
- [run](../../../../functions/src/cli/plan/run.md)
- [run](../../../../functions/src/cli/specify/run.md)
- [run_for_existing](../../../../functions/src/cli/specify/run_for_existing.md)
- [priorities_for_resolver](../../../../functions/src/presets/manager/priorities_for_resolver.md)