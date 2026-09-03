---
type: Rust Module
title: pipeline
resource: tests/pipeline.rs#L1-L446
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/assert-cmd-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/predicates-prelude
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  - target: external/common-first-feature-dir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [setup_project](../../functions/tests/pipeline/setup_project.md)
- [full_pipeline_scaffold_generates_all_artifacts](../../functions/tests/pipeline/full_pipeline_scaffold_generates_all_artifacts.md)
- [pipeline_status_shows_artifact_table](../../functions/tests/pipeline/pipeline_status_shows_artifact_table.md)
- [pipeline_dry_run_output_contains_dry_run_marker](../../functions/tests/pipeline/pipeline_dry_run_output_contains_dry_run_marker.md)
- [pipeline_idsd_generates_intent_before_spec](../../functions/tests/pipeline/pipeline_idsd_generates_intent_before_spec.md)
- [pipeline_new_ignores_stale_feature_env_var](../../functions/tests/pipeline/pipeline_new_ignores_stale_feature_env_var.md)
- [pipeline_intent_apex_uses_single_feature_dir](../../functions/tests/pipeline/pipeline_intent_apex_uses_single_feature_dir.md)
- [pipeline_sdd_unchanged_no_intent_md](../../functions/tests/pipeline/pipeline_sdd_unchanged_no_intent_md.md)
- [pipeline_dry_run_respects_custom_schema_generates_override](../../functions/tests/pipeline/pipeline_dry_run_respects_custom_schema_generates_override.md)

# Imports

- `assert_cmd::Command`
- `predicates::prelude::*`
- `tempfile::TempDir`
- `common::first_feature_dir`

# Member of

- [solidspec](../../packages/solidspec.md)