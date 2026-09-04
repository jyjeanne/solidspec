---
type: Rust Function
title: resolve_feature
resource: src/core/feature.rs#L111-L142
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/feature/find_feature_dir_by_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/current_branch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/is_valid_feature_branch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/latest_feature_dir
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/analyze/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/apex/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/change/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/checklist/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/clarify/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/evidence/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/implement/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/review/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/security_review/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/ship/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/status/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tasks/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tdd_refactor/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tdd_tests/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tests_cmd/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/resolve_explicit_arg_wins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/resolve_env_var_and_latest_fallback
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_feature(explicit_id: Option<&str>, project_root: &Path) -> Result<String>`

# Calls

- [find_feature_dir_by_prefix](../../../../functions/src/core/feature/find_feature_dir_by_prefix.md)
- [current_branch](../../../../functions/src/core/git/current_branch.md)
- [is_valid_feature_branch](../../../../functions/src/core/feature/is_valid_feature_branch.md)
- [latest_feature_dir](../../../../functions/src/core/feature/latest_feature_dir.md)

# Called by

- [run](../../../../functions/src/cli/analyze/run.md)
- [run](../../../../functions/src/cli/apex/run.md)
- [run](../../../../functions/src/cli/change/run.md)
- [run](../../../../functions/src/cli/checklist/run.md)
- [run](../../../../functions/src/cli/clarify/run.md)
- [run](../../../../functions/src/cli/evidence/run.md)
- [run](../../../../functions/src/cli/implement/run.md)
- [run](../../../../functions/src/cli/pipeline/run.md)
- [run](../../../../functions/src/cli/plan/run.md)
- [run](../../../../functions/src/cli/review/run.md)
- [run](../../../../functions/src/cli/security_review/run.md)
- [run](../../../../functions/src/cli/ship/run.md)
- [run](../../../../functions/src/cli/status/run.md)
- [run](../../../../functions/src/cli/tasks/run.md)
- [run](../../../../functions/src/cli/tdd_refactor/run.md)
- [run](../../../../functions/src/cli/tdd_tests/run.md)
- [run](../../../../functions/src/cli/tests_cmd/run.md)
- [resolve_explicit_arg_wins](../../../../functions/src/core/feature/resolve_explicit_arg_wins.md)
- [resolve_env_var_and_latest_fallback](../../../../functions/src/core/feature/resolve_env_var_and_latest_fallback.md)