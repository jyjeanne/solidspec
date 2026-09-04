---
type: Rust Function
title: search_extensions
resource: src/extensions/manager.rs#L80-L84
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/extension/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/search_filters
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn search_extensions(project_root: &Path, query: &str) -> Result<Vec<ExtensionEntry>>`

# Called by

- [run](../../../../functions/src/cli/extension/run.md)
- [search_filters](../../../../functions/src/extensions/manager/search_filters.md)