---
type: Rust Function
title: is_git_repo
resource: src/core/git.rs#L8-L10
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/check/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/intent/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_git_repo(path: &Path) -> bool`

# Called by

- [run](../../../../functions/src/cli/check/run.md)
- [run](../../../../functions/src/cli/init/run.md)
- [run](../../../../functions/src/cli/intent/run.md)
- [run](../../../../functions/src/cli/specify/run.md)