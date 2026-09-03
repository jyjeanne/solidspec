---
type: Rust Function
title: all
resource: src/templates/mod.rs#L24-L36
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/apex/feature_slug
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/upgrade/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/upgrade/setup_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/generates_present
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/aggregate_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/generate_branch_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/find_feature_dir_by_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/format_log_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/copy_embedded_templates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/all_embedded_templates_are_nonempty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/copy_embedded_templates_creates_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn all() -> Vec<(&'static str, &'static str)>`

# Called by

- [feature_slug](../../../functions/src/cli/apex/feature_slug.md)
- [run](../../../functions/src/cli/upgrade/run.md)
- [setup_project](../../../functions/src/cli/upgrade/setup_project.md)
- [generates_present](../../../functions/src/core/artifact_graph/ArtifactGraph/generates_present.md)
- [aggregate_results](../../../functions/src/core/fan_out/aggregate_results.md)
- [generate_branch_name](../../../functions/src/core/feature/generate_branch_name.md)
- [find_feature_dir_by_prefix](../../../functions/src/core/feature/find_feature_dir_by_prefix.md)
- [format_log_entry](../../../functions/src/core/pipeline/format_log_entry.md)
- [copy_embedded_templates](../../../functions/src/templates/copy_embedded_templates.md)
- [all_embedded_templates_are_nonempty](../../../functions/src/templates/all_embedded_templates_are_nonempty.md)
- [copy_embedded_templates_creates_files](../../../functions/src/templates/copy_embedded_templates_creates_files.md)