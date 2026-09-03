---
type: Rust Function
title: copy_embedded_templates
resource: src/templates/mod.rs#L102-L114
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/templates/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/copy_embedded_templates_creates_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/copy_embedded_templates_preserves_existing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn copy_embedded_templates(target_dir: &Path) -> Result<()>`

# Calls

- [all](../../../functions/src/templates/all.md)

# Called by

- [run](../../../functions/src/cli/init/run.md)
- [copy_embedded_templates_creates_files](../../../functions/src/templates/copy_embedded_templates_creates_files.md)
- [copy_embedded_templates_preserves_existing](../../../functions/src/templates/copy_embedded_templates_preserves_existing.md)