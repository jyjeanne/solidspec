---
type: Rust Method
title: as_str
resource: src/core/intent_parser.rs#L50-L57
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/spcx/render_new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/completions/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/ship/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tests_cmd/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/PipelineConfig/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/compute_drift
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/topological_order
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/generates_present
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/TraceGraph/tasks_for_req
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/TraceGraph/tests_for_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/TraceGraph/format_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/build_trace_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/all_artifacts_in_default_graph_are_reachable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/check_plan_compliance
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/check_intent_constraints
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/collect_evidence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/IntentStatus/from_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/extract_section_body
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/extract_list_items
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/status_as_str_roundtrip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/review_intent_alignment
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/extract_clarification_markers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/framework_from_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/render_pytest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/test_generator/render_go
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manifest/ExtensionManifest/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manifest/PresetManifest/validate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn as_str(&self) -> &'static str`

# Called by

- [render_new](../../../../../functions/src/agents/spcx/render_new.md)
- [run](../../../../../functions/src/cli/completions/run.md)
- [run](../../../../../functions/src/cli/init/run.md)
- [run](../../../../../functions/src/cli/ship/run.md)
- [run](../../../../../functions/src/cli/tests_cmd/run.md)
- [validate](../../../../../functions/src/config/PipelineConfig/validate.md)
- [compute_drift](../../../../../functions/src/core/analyzer/compute_drift.md)
- [topological_order](../../../../../functions/src/core/artifact_graph/ArtifactGraph/topological_order.md)
- [generates_present](../../../../../functions/src/core/artifact_graph/ArtifactGraph/generates_present.md)
- [tasks_for_req](../../../../../functions/src/core/artifact_graph/TraceGraph/tasks_for_req.md)
- [tests_for_task](../../../../../functions/src/core/artifact_graph/TraceGraph/tests_for_task.md)
- [format_tree](../../../../../functions/src/core/artifact_graph/TraceGraph/format_tree.md)
- [build_trace_graph](../../../../../functions/src/core/artifact_graph/build_trace_graph.md)
- [all_artifacts_in_default_graph_are_reachable](../../../../../functions/src/core/artifact_graph/all_artifacts_in_default_graph_are_reachable.md)
- [check_plan_compliance](../../../../../functions/src/core/constitution/check_plan_compliance.md)
- [check_intent_constraints](../../../../../functions/src/core/constitution/check_intent_constraints.md)
- [collect_evidence](../../../../../functions/src/core/evidence/collect_evidence.md)
- [from_str](../../../../../functions/src/core/intent_parser/IntentStatus/from_str.md)
- [extract_section_body](../../../../../functions/src/core/intent_parser/extract_section_body.md)
- [extract_list_items](../../../../../functions/src/core/intent_parser/extract_list_items.md)
- [status_as_str_roundtrip](../../../../../functions/src/core/intent_parser/status_as_str_roundtrip.md)
- [review_intent_alignment](../../../../../functions/src/core/review/checks/review_intent_alignment.md)
- [extract_clarification_markers](../../../../../functions/src/core/spec_parser/extract_clarification_markers.md)
- [framework_from_name](../../../../../functions/src/core/test_generator/framework_from_name.md)
- [render_pytest](../../../../../functions/src/core/test_generator/render_pytest.md)
- [render_go](../../../../../functions/src/core/test_generator/render_go.md)
- [validate](../../../../../functions/src/extensions/manifest/ExtensionManifest/validate.md)
- [validate](../../../../../functions/src/presets/manifest/PresetManifest/validate.md)