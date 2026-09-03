---
type: Rust Function
title: run_for_existing
resource: src/cli/specify.rs#L88-L148
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/get_preset_priorities
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/pipeline/execute_phase
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run_for_existing(feature_dir_name: &str, feature_title: &str, schema: &str) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [get_preset_priorities](../../../../functions/src/presets/manager/get_preset_priorities.md)

# Called by

- [execute_phase](../../../../functions/src/cli/pipeline/execute_phase.md)