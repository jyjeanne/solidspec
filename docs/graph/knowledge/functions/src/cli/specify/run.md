---
type: Rust Function
title: run
resource: src/cli/specify.rs#L11-L82
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
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
  - target: functions/src/core/schema/is_intent_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/get_preset_priorities
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_name: &str) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [next_feature_number](../../../../functions/src/core/feature/next_feature_number.md)
- [format_feature_id](../../../../functions/src/core/feature/format_feature_id.md)
- [generate_branch_name](../../../../functions/src/core/feature/generate_branch_name.md)
- [is_git_repo](../../../../functions/src/core/git/is_git_repo.md)
- [create_branch](../../../../functions/src/core/git/create_branch.md)
- [is_intent_schema](../../../../functions/src/core/schema/is_intent_schema.md)
- [get_preset_priorities](../../../../functions/src/presets/manager/get_preset_priorities.md)