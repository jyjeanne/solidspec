---
type: Rust Function
title: setup_script_project
resource: src/templates/mod.rs#L289-L299
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/templates/bash_scripts
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/templates/create_new_feature_increments_past_existing_features
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/get_current_branch_resolves_env_var_prefix_to_dir_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/get_feature_paths_emits_eval_safe_single_line_vars
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/check_prerequisites_passes_for_complete_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/check_prerequisites_fails_when_constitution_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/setup_plan_creates_supporting_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/setup_plan_does_not_overwrite_existing_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/update_agent_context_lists_feature_status
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn setup_script_project(dir: &Path) -> std::path::PathBuf`

# Calls

- [bash_scripts](../../../functions/src/templates/bash_scripts.md)

# Called by

- [create_new_feature_increments_past_existing_features](../../../functions/src/templates/create_new_feature_increments_past_existing_features.md)
- [get_current_branch_resolves_env_var_prefix_to_dir_name](../../../functions/src/templates/get_current_branch_resolves_env_var_prefix_to_dir_name.md)
- [get_feature_paths_emits_eval_safe_single_line_vars](../../../functions/src/templates/get_feature_paths_emits_eval_safe_single_line_vars.md)
- [check_prerequisites_passes_for_complete_project](../../../functions/src/templates/check_prerequisites_passes_for_complete_project.md)
- [check_prerequisites_fails_when_constitution_missing](../../../functions/src/templates/check_prerequisites_fails_when_constitution_missing.md)
- [setup_plan_creates_supporting_files](../../../functions/src/templates/setup_plan_creates_supporting_files.md)
- [setup_plan_does_not_overwrite_existing_files](../../../functions/src/templates/setup_plan_does_not_overwrite_existing_files.md)
- [update_agent_context_lists_feature_status](../../../functions/src/templates/update_agent_context_lists_feature_status.md)