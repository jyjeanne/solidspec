---
type: Rust Function
title: write_log
resource: src/core/pipeline.rs#L340-L353
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/pipeline/format_log_entry
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/pipeline/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/write_log_creates_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/pipeline/write_log_appends_to_existing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_log(feature_dir: &Path, feature_name: &str, results: &[PhaseResult]) -> Result<()>`

# Calls

- [format_log_entry](../../../../functions/src/core/pipeline/format_log_entry.md)

# Called by

- [run](../../../../functions/src/cli/pipeline/run.md)
- [write_log_creates_file](../../../../functions/src/core/pipeline/write_log_creates_file.md)
- [write_log_appends_to_existing](../../../../functions/src/core/pipeline/write_log_appends_to_existing.md)