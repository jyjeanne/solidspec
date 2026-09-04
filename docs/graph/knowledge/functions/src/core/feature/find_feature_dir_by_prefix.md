---
type: Rust Function
title: find_feature_dir_by_prefix
resource: src/core/feature.rs#L145-L202
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/templates/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/intent/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/find_feature_dir_single_match
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/find_feature_dir_multiple_picks_latest
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn find_feature_dir_by_prefix(specs_dir: &Path, prefix: &str) -> Result<String>`

# Calls

- [all](../../../../functions/src/templates/all.md)

# Called by

- [run](../../../../functions/src/cli/intent/run.md)
- [run](../../../../functions/src/cli/pipeline/run.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [find_feature_dir_single_match](../../../../functions/src/core/feature/find_feature_dir_single_match.md)
- [find_feature_dir_multiple_picks_latest](../../../../functions/src/core/feature/find_feature_dir_multiple_picks_latest.md)