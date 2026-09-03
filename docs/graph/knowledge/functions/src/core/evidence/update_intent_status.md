---
type: Rust Function
title: update_intent_status
resource: src/core/evidence.rs#L145-L165
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/evidence/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/update_intent_status_rewrites_status_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/evidence/update_intent_status_preserves_trailing_newline
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn update_intent_status(intent_path: &Path, new_status: &IntentStatus) -> Result<()>`

# Called by

- [run](../../../../functions/src/cli/evidence/run.md)
- [update_intent_status_rewrites_status_line](../../../../functions/src/core/evidence/update_intent_status_rewrites_status_line.md)
- [update_intent_status_preserves_trailing_newline](../../../../functions/src/core/evidence/update_intent_status_preserves_trailing_newline.md)