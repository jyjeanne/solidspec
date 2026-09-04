---
type: Rust Function
title: run
resource: src/cli/completions.rs#L16-L28
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/completions/generate_completions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(shell: &str) -> Result<()>`

# Calls

- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)
- [generate_completions](../../../../functions/src/cli/completions/generate_completions.md)