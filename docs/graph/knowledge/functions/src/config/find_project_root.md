---
type: Rust Function
title: find_project_root
resource: src/config/mod.rs#L369-L379
generated:
  by: okf-rs/0.7.0
relationships:
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
  - target: functions/src/cli/extension/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/implement/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/intent/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/preset/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/review/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/schemas/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/security_review/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/ship/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/run_for_existing
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
  - target: functions/src/cli/upgrade/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/project_default_schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn find_project_root(start: &Path) -> Option<PathBuf>`

# Called by

- [run](../../../functions/src/cli/analyze/run.md)
- [run](../../../functions/src/cli/apex/run.md)
- [run](../../../functions/src/cli/change/run.md)
- [run](../../../functions/src/cli/checklist/run.md)
- [run](../../../functions/src/cli/clarify/run.md)
- [run](../../../functions/src/cli/evidence/run.md)
- [run](../../../functions/src/cli/extension/run.md)
- [run](../../../functions/src/cli/implement/run.md)
- [run](../../../functions/src/cli/intent/run.md)
- [run](../../../functions/src/cli/pipeline/run.md)
- [run](../../../functions/src/cli/plan/run.md)
- [run](../../../functions/src/cli/preset/run.md)
- [run](../../../functions/src/cli/review/run.md)
- [run](../../../functions/src/cli/schemas/run.md)
- [run](../../../functions/src/cli/security_review/run.md)
- [run](../../../functions/src/cli/ship/run.md)
- [run](../../../functions/src/cli/specify/run.md)
- [run_for_existing](../../../functions/src/cli/specify/run_for_existing.md)
- [run](../../../functions/src/cli/status/run.md)
- [run](../../../functions/src/cli/tasks/run.md)
- [run](../../../functions/src/cli/tdd_refactor/run.md)
- [run](../../../functions/src/cli/tdd_tests/run.md)
- [run](../../../functions/src/cli/tests_cmd/run.md)
- [run](../../../functions/src/cli/upgrade/run.md)
- [project_default_schema](../../../functions/src/config/project_default_schema.md)