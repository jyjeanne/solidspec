---
type: Rust Function
title: run
resource: src/cli/evidence.rs#L6-L66
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
  - target: functions/src/core/evidence/collect_evidence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/format_evidence_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/update_intent_status
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_id: Option<&str>, update: bool) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [collect_evidence](../../../../functions/src/core/evidence/collect_evidence.md)
- [format_evidence_report](../../../../functions/src/core/evidence/format_evidence_report.md)
- [update_intent_status](../../../../functions/src/core/evidence/update_intent_status.md)