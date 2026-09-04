---
type: Rust Function
title: run
resource: src/cli/ship.rs#L10-L144
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/build_lanes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/run_fan_out
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/aggregate_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/fan_out/report/format_ship_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run( feature_id: Option<&str>, lane_filter: Vec<String>, fail_on_hold: bool, code_agent: Option<String>, tests_agent: Option<String>, security_agent: Option<String>, perf_agent: Option<String>, no_agent: bool, dry_run: bool, timeout: u64, ignore_timeout: bool, ) -> Result<()>`

# Calls

- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)
- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [build_lanes](../../../../functions/src/core/fan_out/build_lanes.md)
- [run_fan_out](../../../../functions/src/core/fan_out/run_fan_out.md)
- [aggregate_results](../../../../functions/src/core/fan_out/aggregate_results.md)
- [format_ship_report](../../../../functions/src/core/fan_out/report/format_ship_report.md)