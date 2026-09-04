---
type: Rust Module
title: task_generator
resource: src/core/task_generator.rs#L1-L339
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-spec-parser-parsedspec
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-spec-parser-requirement-userstory
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [TaskList](../../../classes/src/core/task_generator/TaskList.md)
- [Phase](../../../classes/src/core/task_generator/Phase.md)
- [Task](../../../classes/src/core/task_generator/Task.md)
- [format](../../../functions/src/core/task_generator/Task/format.md)
- [generate_tasks](../../../functions/src/core/task_generator/generate_tasks.md)
- [format_task_list](../../../functions/src/core/task_generator/format_task_list.md)
- [make_task](../../../functions/src/core/task_generator/make_task.md)
- [sample_spec](../../../functions/src/core/task_generator/sample_spec.md)
- [tasks_have_strict_format](../../../functions/src/core/task_generator/tasks_have_strict_format.md)
- [task_ids_are_sequential_zero_padded](../../../functions/src/core/task_generator/task_ids_are_sequential_zero_padded.md)
- [story_labels_only_in_story_phases](../../../functions/src/core/task_generator/story_labels_only_in_story_phases.md)
- [parallel_markers_present](../../../functions/src/core/task_generator/parallel_markers_present.md)
- [no_data_model_skips_schema_task](../../../functions/src/core/task_generator/no_data_model_skips_schema_task.md)
- [file_paths_included_in_entity_tasks](../../../functions/src/core/task_generator/file_paths_included_in_entity_tasks.md)
- [formatted_output_has_phases_and_checkpoints](../../../functions/src/core/task_generator/formatted_output_has_phases_and_checkpoints.md)

# Imports

- `super::spec_parser::ParsedSpec`
- `super::*`
- `crate::core::spec_parser::{Requirement, UserStory}`

# Member of

- [solidspec](../../../packages/solidspec.md)