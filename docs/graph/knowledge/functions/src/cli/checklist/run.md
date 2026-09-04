---
type: Rust Function
title: run
resource: src/cli/checklist.rs#L16-L75
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/checklist/find_last_chk_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/checklist/generate_append_items
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/Task/format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/get_preset_priorities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/resolver/load_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_id: Option<&str>, append: bool) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [find_last_chk_id](../../../../functions/src/cli/checklist/find_last_chk_id.md)
- [generate_append_items](../../../../functions/src/cli/checklist/generate_append_items.md)
- [format](../../../../functions/src/core/task_generator/Task/format.md)
- [get_preset_priorities](../../../../functions/src/presets/manager/get_preset_priorities.md)
- [load_template](../../../../functions/src/templates/resolver/load_template.md)
- [render](../../../../functions/src/templates/render.md)