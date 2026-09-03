---
type: Rust Function
title: run
resource: src/cli/analyze.rs#L6-L52
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/analyze_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/format_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_id: Option<&str>) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [analyze_feature](../../../../functions/src/core/analyzer/analyze_feature.md)
- [format_report](../../../../functions/src/core/analyzer/format_report.md)