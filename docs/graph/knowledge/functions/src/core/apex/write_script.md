---
type: Rust Function
title: write_script
resource: src/core/apex.rs#L132-L142
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/apex/extract_skill
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_script(path: &Path, bytes: &[u8]) -> Result<()>`

# Called by

- [extract_skill](../../../../functions/src/core/apex/extract_skill.md)