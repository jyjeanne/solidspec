---
type: Rust Function
title: resolve_github_token
resource: src/core/token.rs#L8-L34
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/token/cli_flag_wins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/token/empty_string_treated_as_none
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/token/whitespace_trimmed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/token/none_flag_falls_through
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_github_token(cli_flag: Option<&str>) -> Option<String>`

# Called by

- [cli_flag_wins](../../../../functions/src/core/token/cli_flag_wins.md)
- [empty_string_treated_as_none](../../../../functions/src/core/token/empty_string_treated_as_none.md)
- [whitespace_trimmed](../../../../functions/src/core/token/whitespace_trimmed.md)
- [none_flag_falls_through](../../../../functions/src/core/token/none_flag_falls_through.md)