---
type: Rust Function
title: run
resource: src/cli/pipeline.rs#L11-L272
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/filter_phases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/load_graph
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
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/check_agent_availability
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/PipelineConfig/agent_for_phase
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/should_skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/phase_type
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/invoker/supports_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/skip_reason
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/execute_phase
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/find_feature_dir_by_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/write_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/detect_completion
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/compute_states
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/artifact_graph/ArtifactGraph/first_ready
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run( feature_id: Option<&str>, new_desc: Option<&str>, from: Option<&str>, to: Option<&str>, only: Option<&str>, force: bool, dry_run: bool, auto: bool, no_agent: bool, schema: &str, ) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [filter_phases](../../../../functions/src/core/pipeline/filter_phases.md)
- [load_graph](../../../../functions/src/core/schema/load_graph.md)
- [next_feature_number](../../../../functions/src/core/feature/next_feature_number.md)
- [format_feature_id](../../../../functions/src/core/feature/format_feature_id.md)
- [generate_branch_name](../../../../functions/src/core/feature/generate_branch_name.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [check_agent_availability](../../../../functions/src/cli/pipeline/check_agent_availability.md)
- [agent_for_phase](../../../../functions/src/config/PipelineConfig/agent_for_phase.md)
- [should_skip](../../../../functions/src/core/pipeline/should_skip.md)
- [phase_type](../../../../functions/src/core/pipeline/phase_type.md)
- [supports_cli](../../../../functions/src/agents/invoker/supports_cli.md)
- [skip_reason](../../../../functions/src/cli/pipeline/skip_reason.md)
- [execute_phase](../../../../functions/src/cli/pipeline/execute_phase.md)
- [find_feature_dir_by_prefix](../../../../functions/src/core/feature/find_feature_dir_by_prefix.md)
- [write_log](../../../../functions/src/core/pipeline/write_log.md)
- [detect_completion](../../../../functions/src/core/artifact_graph/ArtifactGraph/detect_completion.md)
- [compute_states](../../../../functions/src/core/artifact_graph/ArtifactGraph/compute_states.md)
- [first_ready](../../../../functions/src/core/artifact_graph/ArtifactGraph/first_ready.md)