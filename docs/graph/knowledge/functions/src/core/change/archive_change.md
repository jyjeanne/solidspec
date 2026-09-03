---
type: Rust Function
title: archive_change
resource: src/core/change.rs#L350-L425
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/change/parse_delta_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/merge_deltas
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/change/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/create_and_archive_change_roundtrip
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn archive_change(feature_dir: &Path, slug: &str) -> Result<()>`

# Calls

- [parse_delta_spec](../../../../functions/src/core/change/parse_delta_spec.md)
- [merge_deltas](../../../../functions/src/core/change/merge_deltas.md)

# Called by

- [run](../../../../functions/src/cli/change/run.md)
- [create_and_archive_change_roundtrip](../../../../functions/src/core/change/create_and_archive_change_roundtrip.md)