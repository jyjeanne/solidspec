---
type: Rust Function
title: cli_command_for
resource: src/agents/spcx.rs#L52-L58
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/spcx/render_new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/spcx/render_finalise
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn cli_command_for(artifact_id: &str) -> &str`

# Called by

- [render_new](../../../../functions/src/agents/spcx/render_new.md)
- [render_finalise](../../../../functions/src/agents/spcx/render_finalise.md)