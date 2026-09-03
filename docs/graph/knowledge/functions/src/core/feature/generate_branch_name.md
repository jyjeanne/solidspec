---
type: Rust Function
title: generate_branch_name
resource: src/core/feature.rs#L58-L92
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/templates/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/sanitize
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/intent/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/generate_branch_name_from_description
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_branch_name(description: &str) -> Result<String>`

# Calls

- [all](../../../../functions/src/templates/all.md)
- [sanitize](../../../../functions/src/core/feature/sanitize.md)

# Called by

- [run](../../../../functions/src/cli/intent/run.md)
- [run](../../../../functions/src/cli/pipeline/run.md)
- [run](../../../../functions/src/cli/specify/run.md)
- [generate_branch_name_from_description](../../../../functions/src/core/feature/generate_branch_name_from_description.md)