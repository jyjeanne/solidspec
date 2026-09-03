---
type: Rust Function
title: search_presets
resource: src/presets/manager.rs#L73-L77
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/preset/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/search_filters_by_keyword
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn search_presets(project_root: &Path, query: &str) -> Result<Vec<PresetEntry>>`

# Called by

- [run](../../../../functions/src/cli/preset/run.md)
- [search_filters_by_keyword](../../../../functions/src/presets/manager/search_filters_by_keyword.md)