---
type: Rust Module
title: ship
resource: tests/ship.rs#L1-L261
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

- [create_feature](../../functions/tests/ship/create_feature.md)
- [ship_dry_run_shows_all_lanes](../../functions/tests/ship/ship_dry_run_shows_all_lanes.md)
- [ship_no_agent_creates_report_with_real_scores](../../functions/tests/ship/ship_no_agent_creates_report_with_real_scores.md)
- [ship_lane_filter_runs_subset](../../functions/tests/ship/ship_lane_filter_runs_subset.md)
- [ship_fail_on_hold_exits_nonzero](../../functions/tests/ship/ship_fail_on_hold_exits_nonzero.md)
- [ship_report_written_to_feature_dir](../../functions/tests/ship/ship_report_written_to_feature_dir.md)
- [ship_unknown_lane_errors](../../functions/tests/ship/ship_unknown_lane_errors.md)
- [ship_fails_without_spec_md](../../functions/tests/ship/ship_fails_without_spec_md.md)
- [ship_decision_ship_when_all_lanes_pass](../../functions/tests/ship/ship_decision_ship_when_all_lanes_pass.md)
- [status_shows_ship_artifact_after_review](../../functions/tests/ship/status_shows_ship_artifact_after_review.md)

# Imports

- `predicates::prelude::*`
- `common::{first_feature_dir, init_project, solidspec}`

# Member of

- [solidspec](../../packages/solidspec.md)