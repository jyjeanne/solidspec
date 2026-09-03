---
type: Rust Function
title: build_trace_graph
resource: src/core/artifact_graph.rs#L389-L541
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/core/analyzer/analyze_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/extracts_fr_ids_from_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/all_frs_orphaned_when_no_tasks_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/fr_with_task_not_orphaned
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/tasks_without_fr_refs_produce_orphaned_frs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/req_to_task_links_created
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/intent_to_req_links_created_when_intent_md_present
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/task_to_test_link_when_test_mentions_task_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/format_tree_contains_fr_ids
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_trace_graph(feature_dir: &Path) -> Option<TraceGraph>`

# Calls

- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [analyze_feature](../../../../functions/src/core/analyzer/analyze_feature.md)
- [extracts_fr_ids_from_spec](../../../../functions/src/core/artifact_graph/extracts_fr_ids_from_spec.md)
- [all_frs_orphaned_when_no_tasks_md](../../../../functions/src/core/artifact_graph/all_frs_orphaned_when_no_tasks_md.md)
- [fr_with_task_not_orphaned](../../../../functions/src/core/artifact_graph/fr_with_task_not_orphaned.md)
- [tasks_without_fr_refs_produce_orphaned_frs](../../../../functions/src/core/artifact_graph/tasks_without_fr_refs_produce_orphaned_frs.md)
- [req_to_task_links_created](../../../../functions/src/core/artifact_graph/req_to_task_links_created.md)
- [intent_to_req_links_created_when_intent_md_present](../../../../functions/src/core/artifact_graph/intent_to_req_links_created_when_intent_md_present.md)
- [task_to_test_link_when_test_mentions_task_id](../../../../functions/src/core/artifact_graph/task_to_test_link_when_test_mentions_task_id.md)
- [format_tree_contains_fr_ids](../../../../functions/src/core/artifact_graph/format_tree_contains_fr_ids.md)