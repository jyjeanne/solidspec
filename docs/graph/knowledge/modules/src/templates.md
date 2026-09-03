---
type: Rust Module
title: templates
resource: src/templates/mod.rs#L1-L503
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tera-context-tera
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-errors-solidspecerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [all](../../functions/src/templates/all.md)
- [bash_scripts](../../functions/src/templates/bash_scripts.md)
- [powershell_scripts](../../functions/src/templates/powershell_scripts.md)
- [render](../../functions/src/templates/render.md)
- [copy_embedded_templates](../../functions/src/templates/copy_embedded_templates.md)
- [copy_embedded_scripts](../../functions/src/templates/copy_embedded_scripts.md)
- [render_with_all_variables](../../functions/src/templates/render_with_all_variables.md)
- [render_missing_variable_returns_error](../../functions/src/templates/render_missing_variable_returns_error.md)
- [render_empty_arguments_handled](../../functions/src/templates/render_empty_arguments_handled.md)
- [render_preserves_special_characters_in_markdown](../../functions/src/templates/render_preserves_special_characters_in_markdown.md)
- [all_embedded_templates_are_nonempty](../../functions/src/templates/all_embedded_templates_are_nonempty.md)
- [embedded_templates_contain_expected_markers](../../functions/src/templates/embedded_templates_contain_expected_markers.md)
- [copy_embedded_templates_creates_files](../../functions/src/templates/copy_embedded_templates_creates_files.md)
- [copy_embedded_templates_preserves_existing](../../functions/src/templates/copy_embedded_templates_preserves_existing.md)
- [all_bash_scripts_are_nonempty](../../functions/src/templates/all_bash_scripts_are_nonempty.md)
- [all_powershell_scripts_are_nonempty](../../functions/src/templates/all_powershell_scripts_are_nonempty.md)
- [copy_embedded_scripts_creates_files](../../functions/src/templates/copy_embedded_scripts_creates_files.md)
- [copy_embedded_scripts_overwrites_existing](../../functions/src/templates/copy_embedded_scripts_overwrites_existing.md)
- [setup_script_project](../../functions/src/templates/setup_script_project.md)
- [create_new_feature_increments_past_existing_features](../../functions/src/templates/create_new_feature_increments_past_existing_features.md)
- [get_current_branch_resolves_env_var_prefix_to_dir_name](../../functions/src/templates/get_current_branch_resolves_env_var_prefix_to_dir_name.md)
- [get_feature_paths_emits_eval_safe_single_line_vars](../../functions/src/templates/get_feature_paths_emits_eval_safe_single_line_vars.md)
- [check_prerequisites_passes_for_complete_project](../../functions/src/templates/check_prerequisites_passes_for_complete_project.md)
- [check_prerequisites_fails_when_constitution_missing](../../functions/src/templates/check_prerequisites_fails_when_constitution_missing.md)
- [setup_plan_creates_supporting_files](../../functions/src/templates/setup_plan_creates_supporting_files.md)
- [setup_plan_does_not_overwrite_existing_files](../../functions/src/templates/setup_plan_does_not_overwrite_existing_files.md)
- [update_agent_context_lists_feature_status](../../functions/src/templates/update_agent_context_lists_feature_status.md)

# Imports

- `std::collections::HashMap`
- `std::path::Path`
- `anyhow::Result`
- `tera::{Context, Tera}`
- `crate::core::errors::SolidSpecError`
- `super::*`
- `std::process::Command`

# Member of

- [solidspec](../../packages/solidspec.md)