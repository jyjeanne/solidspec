---
type: Rust Function
title: fire_hooks
resource: src/extensions/hooks.rs#L7-L80
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/extensions/registry/ExtensionRegistry/enabled_hooks
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/implement/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/tasks/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/hooks/fire_hooks_skips_missing_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/hooks/fire_hooks_skips_disabled_extensions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn fire_hooks(trigger: &str, project_root: &Path, registry: &ExtensionRegistry)`

# Calls

- [enabled_hooks](../../../../functions/src/extensions/registry/ExtensionRegistry/enabled_hooks.md)

# Called by

- [run](../../../../functions/src/cli/implement/run.md)
- [run](../../../../functions/src/cli/init/run.md)
- [run](../../../../functions/src/cli/tasks/run.md)
- [fire_hooks_skips_missing_file](../../../../functions/src/extensions/hooks/fire_hooks_skips_missing_file.md)
- [fire_hooks_skips_disabled_extensions](../../../../functions/src/extensions/hooks/fire_hooks_skips_disabled_extensions.md)