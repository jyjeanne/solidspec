---
type: Rust Module
title: security_first_minimal
resource: tests/security_first_minimal.rs#L1-L442
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/predicates-prelude
    resolved_by: tree-sitter
    confidence: exact
  - target: external/common-first-feature-dir-init-project-solidspec
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [minimal_pipeline_no_agent_scaffolds_all_four_artifacts](../../functions/tests/security_first_minimal/minimal_pipeline_no_agent_scaffolds_all_four_artifacts.md)
- [minimal_status_shows_only_four_artifacts_and_no_clarify_or_review](../../functions/tests/security_first_minimal/minimal_status_shows_only_four_artifacts_and_no_clarify_or_review.md)
- [minimal_tasks_require_only_spec_and_plan_no_security_review](../../functions/tests/security_first_minimal/minimal_tasks_require_only_spec_and_plan_no_security_review.md)
- [security_first_status_lists_security_review_between_plan_and_tasks](../../functions/tests/security_first_minimal/security_first_status_lists_security_review_between_plan_and_tasks.md)
- [security_first_tasks_blocked_until_security_review_md_exists](../../functions/tests/security_first_minimal/security_first_tasks_blocked_until_security_review_md_exists.md)
- [tasks_command_itself_blocks_without_security_review_md](../../functions/tests/security_first_minimal/tasks_command_itself_blocks_without_security_review_md.md)
- [tasks_without_schema_flag_defaults_to_spec_driven_and_is_unaffected](../../functions/tests/security_first_minimal/tasks_without_schema_flag_defaults_to_spec_driven_and_is_unaffected.md)
- [security_first_pipeline_no_agent_scaffolds_all_five_artifacts](../../functions/tests/security_first_minimal/security_first_pipeline_no_agent_scaffolds_all_five_artifacts.md)
- [security_review_command_is_idempotent](../../functions/tests/security_first_minimal/security_review_command_is_idempotent.md)
- [security_review_dry_run_prints_without_writing](../../functions/tests/security_first_minimal/security_review_dry_run_prints_without_writing.md)
- [security_review_fails_without_plan_md](../../functions/tests/security_first_minimal/security_review_fails_without_plan_md.md)
- [security_first_dry_run_previews_all_five_phases_without_executing](../../functions/tests/security_first_minimal/security_first_dry_run_previews_all_five_phases_without_executing.md)

# Imports

- `predicates::prelude::*`
- `common::{first_feature_dir, init_project, solidspec}`

# Member of

- [solidspec](../../packages/solidspec.md)