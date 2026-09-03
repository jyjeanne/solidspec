---
type: Rust Function
title: init
resource: src/cli/security_review.rs#L59-L62
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/security_review/fails_when_feature_dir_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/security_review/writes_report_when_plan_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/security_review/dry_run_does_not_write_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/init_repo
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn init(dir: &std::path::Path)`

# Called by

- [fails_when_feature_dir_missing](../../../../functions/src/cli/security_review/fails_when_feature_dir_missing.md)
- [writes_report_when_plan_exists](../../../../functions/src/cli/security_review/writes_report_when_plan_exists.md)
- [dry_run_does_not_write_file](../../../../functions/src/cli/security_review/dry_run_does_not_write_file.md)
- [init_repo](../../../../functions/src/core/git/init_repo.md)
- [main](../../../../functions/src/main.md)