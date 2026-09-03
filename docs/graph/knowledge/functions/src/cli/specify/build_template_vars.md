---
type: Rust Function
title: build_template_vars
resource: src/cli/specify.rs#L212-L229
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/task_generator/Task/format
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_template_vars( config: &config::RootConfig, feature_id: &str, feature_name: &str, branch_name: &str, ) -> HashMap<String, String>`

# Calls

- [format](../../../../functions/src/core/task_generator/Task/format.md)