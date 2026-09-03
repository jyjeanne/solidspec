---
type: Rust Module
title: artifact_graph
resource: src/core/artifact_graph.rs#L1-L962
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-collections-hashmap-hashset-vecdeque
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-lazylock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ArtifactNode](../../../classes/src/core/artifact_graph/ArtifactNode.md)
- [ArtifactGraph](../../../classes/src/core/artifact_graph/ArtifactGraph.md)
- [ArtifactState](../../../classes/src/core/artifact_graph/ArtifactState.md)
- [new](../../../functions/src/core/artifact_graph/ArtifactGraph/new.md)
- [get](../../../functions/src/core/artifact_graph/ArtifactGraph/get.md)
- [topological_order](../../../functions/src/core/artifact_graph/ArtifactGraph/topological_order.md)
- [first_ready](../../../functions/src/core/artifact_graph/ArtifactGraph/first_ready.md)
- [compute_states](../../../functions/src/core/artifact_graph/ArtifactGraph/compute_states.md)
- [detect_completion](../../../functions/src/core/artifact_graph/ArtifactGraph/detect_completion.md)
- [generates_present](../../../functions/src/core/artifact_graph/ArtifactGraph/generates_present.md)
- [glob_matches](../../../functions/src/core/artifact_graph/glob_matches.md)
- [TraceLinkType](../../../classes/src/core/artifact_graph/TraceLinkType.md)
- [TraceLink](../../../classes/src/core/artifact_graph/TraceLink.md)
- [TraceGraph](../../../classes/src/core/artifact_graph/TraceGraph.md)
- [tasks_for_req](../../../functions/src/core/artifact_graph/TraceGraph/tasks_for_req.md)
- [tests_for_task](../../../functions/src/core/artifact_graph/TraceGraph/tests_for_task.md)
- [format_tree](../../../functions/src/core/artifact_graph/TraceGraph/format_tree.md)
- [build_trace_graph](../../../functions/src/core/artifact_graph/build_trace_graph.md)
- [spec_driven_graph](../../../functions/src/core/artifact_graph/spec_driven_graph.md)
- [default_graph_has_eight_artifacts](../../../functions/src/core/artifact_graph/default_graph_has_eight_artifacts.md)
- [topological_order_starts_with_no_dependency_artifacts](../../../functions/src/core/artifact_graph/topological_order_starts_with_no_dependency_artifacts.md)
- [first_ready_on_fresh_project_is_the_root_artifact](../../../functions/src/core/artifact_graph/first_ready_on_fresh_project_is_the_root_artifact.md)
- [first_ready_advances_as_artifacts_complete](../../../functions/src/core/artifact_graph/first_ready_advances_as_artifacts_complete.md)
- [first_ready_is_none_when_everything_is_done](../../../functions/src/core/artifact_graph/first_ready_is_none_when_everything_is_done.md)
- [tasks_requires_spec_and_plan](../../../functions/src/core/artifact_graph/tasks_requires_spec_and_plan.md)
- [compute_states_shows_blocked_when_deps_missing](../../../functions/src/core/artifact_graph/compute_states_shows_blocked_when_deps_missing.md)
- [detect_completion_finds_existing_files](../../../functions/src/core/artifact_graph/detect_completion_finds_existing_files.md)
- [detect_completion_empty_trailing_slash_dir_is_incomplete](../../../functions/src/core/artifact_graph/detect_completion_empty_trailing_slash_dir_is_incomplete.md)
- [detect_completion_nonempty_trailing_slash_dir_is_complete](../../../functions/src/core/artifact_graph/detect_completion_nonempty_trailing_slash_dir_is_complete.md)
- [all_artifacts_in_default_graph_are_reachable](../../../functions/src/core/artifact_graph/all_artifacts_in_default_graph_are_reachable.md)
- [glob_matches_star_patterns](../../../functions/src/core/artifact_graph/glob_matches_star_patterns.md)
- [detect_completion_glob_requires_matching_file](../../../functions/src/core/artifact_graph/detect_completion_glob_requires_matching_file.md)
- [detect_completion_glob_in_subdirectory](../../../functions/src/core/artifact_graph/detect_completion_glob_in_subdirectory.md)
- [duplicate_artifact_id_errors](../../../functions/src/core/artifact_graph/duplicate_artifact_id_errors.md)
- [unknown_dependency_errors](../../../functions/src/core/artifact_graph/unknown_dependency_errors.md)
- [write](../../../functions/src/core/artifact_graph/write.md)
- [no_spec_returns_none](../../../functions/src/core/artifact_graph/no_spec_returns_none.md)
- [extracts_fr_ids_from_spec](../../../functions/src/core/artifact_graph/extracts_fr_ids_from_spec.md)
- [all_frs_orphaned_when_no_tasks_md](../../../functions/src/core/artifact_graph/all_frs_orphaned_when_no_tasks_md.md)
- [fr_with_task_not_orphaned](../../../functions/src/core/artifact_graph/fr_with_task_not_orphaned.md)
- [tasks_without_fr_refs_produce_orphaned_frs](../../../functions/src/core/artifact_graph/tasks_without_fr_refs_produce_orphaned_frs.md)
- [req_to_task_links_created](../../../functions/src/core/artifact_graph/req_to_task_links_created.md)
- [intent_to_req_links_created_when_intent_md_present](../../../functions/src/core/artifact_graph/intent_to_req_links_created_when_intent_md_present.md)
- [task_to_test_link_when_test_mentions_task_id](../../../functions/src/core/artifact_graph/task_to_test_link_when_test_mentions_task_id.md)
- [format_tree_contains_fr_ids](../../../functions/src/core/artifact_graph/format_tree_contains_fr_ids.md)

# Imports

- `std::collections::{HashMap, HashSet, VecDeque}`
- `std::path::Path`
- `std::sync::LazyLock`
- `regex::Regex`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)