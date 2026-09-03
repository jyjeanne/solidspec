---
type: Rust Function
title: run
resource: src/cli/intent.rs#L11-L119
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/find_feature_dir_by_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/next_feature_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/format_feature_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/generate_branch_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/is_git_repo
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/create_branch
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

`pub fn run(title: &str, feature_id: Option<&str>) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [find_feature_dir_by_prefix](../../../../functions/src/core/feature/find_feature_dir_by_prefix.md)
- [next_feature_number](../../../../functions/src/core/feature/next_feature_number.md)
- [format_feature_id](../../../../functions/src/core/feature/format_feature_id.md)
- [generate_branch_name](../../../../functions/src/core/feature/generate_branch_name.md)
- [is_git_repo](../../../../functions/src/core/git/is_git_repo.md)
- [create_branch](../../../../functions/src/core/git/create_branch.md)
- [get_preset_priorities](../../../../functions/src/presets/manager/get_preset_priorities.md)
- [load_template](../../../../functions/src/templates/resolver/load_template.md)
- [render](../../../../functions/src/templates/render.md)