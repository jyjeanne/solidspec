---
type: Rust Module
title: fan_out
resource: src/core/fan_out.rs#L1-L1414
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-lazylock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-instant
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-fanoutconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-review-self-dimension-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-report-format-ship-report
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-agents-invoker-self-invokeresult
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

- [ReviewLane](../../../classes/src/core/fan_out/ReviewLane.md)
- [LaneResult](../../../classes/src/core/fan_out/LaneResult.md)
- [LaneStatus](../../../classes/src/core/fan_out/LaneStatus.md)
- [ShipReport](../../../classes/src/core/fan_out/ShipReport.md)
- [ShipDecision](../../../classes/src/core/fan_out/ShipDecision.md)
- [FanOutFinding](../../../classes/src/core/fan_out/FanOutFinding.md)
- [score_from_heuristics](../../../functions/src/core/fan_out/score_from_heuristics.md)
- [run_lane_no_agent](../../../functions/src/core/fan_out/run_lane_no_agent.md)
- [lane_findings_from_report](../../../functions/src/core/fan_out/lane_findings_from_report.md)
- [lane_covers_dimension](../../../functions/src/core/fan_out/lane_covers_dimension.md)
- [penalty_weight](../../../functions/src/core/fan_out/penalty_weight.md)
- [apply_penalty_formula](../../../functions/src/core/fan_out/apply_penalty_formula.md)
- [LaneSpec](../../../classes/src/core/fan_out/LaneSpec.md)
- [lane_prompt](../../../functions/src/core/fan_out/lane_prompt.md)
- [lane_config](../../../functions/src/core/fan_out/lane_config.md)
- [build_lanes](../../../functions/src/core/fan_out/build_lanes.md)
- [extract_score](../../../functions/src/core/fan_out/extract_score.md)
- [derive_score_from_keywords](../../../functions/src/core/fan_out/derive_score_from_keywords.md)
- [parse_findings_from_output](../../../functions/src/core/fan_out/parse_findings_from_output.md)
- [parse_severity](../../../functions/src/core/fan_out/parse_severity.md)
- [run_lane_with_agent](../../../functions/src/core/fan_out/run_lane_with_agent.md)
- [run_lane](../../../functions/src/core/fan_out/run_lane.md)
- [run_fan_out](../../../functions/src/core/fan_out/run_fan_out.md)
- [print_lane_result](../../../functions/src/core/fan_out/print_lane_result.md)
- [aggregate_results](../../../functions/src/core/fan_out/aggregate_results.md)
- [make_lane](../../../functions/src/core/fan_out/make_lane.md)
- [write](../../../functions/src/core/fan_out/write.md)
- [penalty_formula_two_high_findings_scores_90](../../../functions/src/core/fan_out/penalty_formula_two_high_findings_scores_90.md)
- [penalty_formula_no_findings_scores_100](../../../functions/src/core/fan_out/penalty_formula_no_findings_scores_100.md)
- [penalty_formula_mixed_severities](../../../functions/src/core/fan_out/penalty_formula_mixed_severities.md)
- [penalty_formula_clamped_at_zero](../../../functions/src/core/fan_out/penalty_formula_clamped_at_zero.md)
- [code_lane_placeholder_spec_is_penalized](../../../functions/src/core/fan_out/code_lane_placeholder_spec_is_penalized.md)
- [security_lane_auth_spec_without_plan_security_section_is_penalized](../../../functions/src/core/fan_out/security_lane_auth_spec_without_plan_security_section_is_penalized.md)
- [clean_spec_code_lane_scores_at_or_above_threshold](../../../functions/src/core/fan_out/clean_spec_code_lane_scores_at_or_above_threshold.md)
- [missing_spec_returns_failed_status](../../../functions/src/core/fan_out/missing_spec_returns_failed_status.md)
- [score_from_heuristics_returns_err_when_no_spec](../../../functions/src/core/fan_out/score_from_heuristics_returns_err_when_no_spec.md)
- [tests_lane_covers_testability_only](../../../functions/src/core/fan_out/tests_lane_covers_testability_only.md)
- [duration_ms_is_recorded](../../../functions/src/core/fan_out/duration_ms_is_recorded.md)
- [extract_score_parses_score_line](../../../functions/src/core/fan_out/extract_score_parses_score_line.md)
- [extract_score_takes_last_score_line](../../../functions/src/core/fan_out/extract_score_takes_last_score_line.md)
- [extract_score_clamps_over_100](../../../functions/src/core/fan_out/extract_score_clamps_over_100.md)
- [extract_score_score_100_is_not_clamped](../../../functions/src/core/fan_out/extract_score_score_100_is_not_clamped.md)
- [extract_score_no_keywords_returns_100](../../../functions/src/core/fan_out/extract_score_no_keywords_returns_100.md)
- [extract_score_fallback_two_high_findings_scores_90](../../../functions/src/core/fan_out/extract_score_fallback_two_high_findings_scores_90.md)
- [extract_score_fallback_mixed_penalties](../../../functions/src/core/fan_out/extract_score_fallback_mixed_penalties.md)
- [parse_findings_extracts_two_findings](../../../functions/src/core/fan_out/parse_findings_extracts_two_findings.md)
- [parse_findings_empty_output_returns_empty](../../../functions/src/core/fan_out/parse_findings_empty_output_returns_empty.md)
- [parse_findings_unknown_severity_skips_block](../../../functions/src/core/fan_out/parse_findings_unknown_severity_skips_block.md)
- [parse_findings_problem_without_fix_still_captured](../../../functions/src/core/fan_out/parse_findings_problem_without_fix_still_captured.md)
- [build_lanes_creates_four_lanes](../../../functions/src/core/fan_out/build_lanes_creates_four_lanes.md)
- [build_lanes_uses_default_agent_when_no_override](../../../functions/src/core/fan_out/build_lanes_uses_default_agent_when_no_override.md)
- [build_lanes_uses_per_lane_agent_override](../../../functions/src/core/fan_out/build_lanes_uses_per_lane_agent_override.md)
- [build_lanes_uses_config_thresholds](../../../functions/src/core/fan_out/build_lanes_uses_config_thresholds.md)
- [build_lanes_prompts_contain_feature_name](../../../functions/src/core/fan_out/build_lanes_prompts_contain_feature_name.md)
- [build_lanes_prompts_contain_score_instruction](../../../functions/src/core/fan_out/build_lanes_prompts_contain_score_instruction.md)
- [make_done_result](../../../functions/src/core/fan_out/make_done_result.md)
- [make_critical_finding](../../../functions/src/core/fan_out/make_critical_finding.md)
- [all_lanes_pass_returns_ship](../../../functions/src/core/fan_out/all_lanes_pass_returns_ship.md)
- [one_lane_below_threshold_returns_hold](../../../functions/src/core/fan_out/one_lane_below_threshold_returns_hold.md)
- [critical_finding_in_security_lane_always_holds](../../../functions/src/core/fan_out/critical_finding_in_security_lane_always_holds.md)
- [timed_out_lane_returns_hold](../../../functions/src/core/fan_out/timed_out_lane_returns_hold.md)
- [ignore_timeout_uses_partial_results](../../../functions/src/core/fan_out/ignore_timeout_uses_partial_results.md)
- [all_lanes_failed_returns_hold_with_message](../../../functions/src/core/fan_out/all_lanes_failed_returns_hold_with_message.md)
- [all_lanes_timed_out_with_ignore_timeout_returns_hold](../../../functions/src/core/fan_out/all_lanes_timed_out_with_ignore_timeout_returns_hold.md)
- [format_ship_report_contains_machine_readable_hold_header](../../../functions/src/core/fan_out/format_ship_report_contains_machine_readable_hold_header.md)
- [format_ship_report_contains_machine_readable_ship_header](../../../functions/src/core/fan_out/format_ship_report_contains_machine_readable_ship_header.md)

# Imports

- `std::path::{Path, PathBuf}`
- `std::sync::LazyLock`
- `std::time::Instant`
- `anyhow::Result`
- `regex::Regex`
- `crate::config::FanOutConfig`
- `crate::core::review::{self, Dimension, Severity}`
- `pub use report::format_ship_report`
- `crate::agents::invoker::{self, InvokeResult}`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)