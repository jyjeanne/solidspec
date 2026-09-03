---
type: Rust Function
title: create_feature
resource: tests/tdd.rs#L9-L42
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/common/first_feature_dir
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_feature(dir: &std::path::Path, name: &str) -> std::path::PathBuf`

# Calls

- [first_feature_dir](../../../functions/tests/common/first_feature_dir.md)