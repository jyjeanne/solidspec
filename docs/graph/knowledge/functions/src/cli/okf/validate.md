---
type: Rust Function
title: validate
resource: src/cli/okf.rs#L56-L75
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/okf/validation_should_fail
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate(bundle: &Path, ci: bool) -> Result<()>`

# Calls

- [validation_should_fail](../../../../functions/src/core/okf/validation_should_fail.md)