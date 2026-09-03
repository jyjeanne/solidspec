---
type: Rust Function
title: generate_tasks
resource: src/core/task_generator.rs#L36-L152
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/task_generator/make_task
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/tasks/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/tasks_have_strict_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/task_ids_are_sequential_zero_padded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/story_labels_only_in_story_phases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/parallel_markers_present
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/no_data_model_skips_schema_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/file_paths_included_in_entity_tasks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/formatted_output_has_phases_and_checkpoints
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_tasks(spec: &ParsedSpec, plan_content: &str, has_data_model: bool) -> TaskList`

# Calls

- [make_task](../../../../functions/src/core/task_generator/make_task.md)

# Called by

- [run](../../../../functions/src/cli/tasks/run.md)
- [tasks_have_strict_format](../../../../functions/src/core/task_generator/tasks_have_strict_format.md)
- [task_ids_are_sequential_zero_padded](../../../../functions/src/core/task_generator/task_ids_are_sequential_zero_padded.md)
- [story_labels_only_in_story_phases](../../../../functions/src/core/task_generator/story_labels_only_in_story_phases.md)
- [parallel_markers_present](../../../../functions/src/core/task_generator/parallel_markers_present.md)
- [no_data_model_skips_schema_task](../../../../functions/src/core/task_generator/no_data_model_skips_schema_task.md)
- [file_paths_included_in_entity_tasks](../../../../functions/src/core/task_generator/file_paths_included_in_entity_tasks.md)
- [formatted_output_has_phases_and_checkpoints](../../../../functions/src/core/task_generator/formatted_output_has_phases_and_checkpoints.md)