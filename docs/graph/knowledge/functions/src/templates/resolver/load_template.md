---
type: Rust Function
title: load_template
resource: src/templates/resolver.rs#L117-L148
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/templates/resolver/resolve_cached
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/checklist/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/intent/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/write_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/resolver/load_template_from_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/resolver/load_template_embedded_default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/resolver/load_unknown_template_returns_error
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_template( template_name: &str, project_root: &Path, preset_priorities: &[(String, u32)], ) -> std::io::Result<(String, TemplateSource)>`

# Calls

- [resolve_cached](../../../../functions/src/templates/resolver/resolve_cached.md)

# Called by

- [run](../../../../functions/src/cli/checklist/run.md)
- [run](../../../../functions/src/cli/intent/run.md)
- [run](../../../../functions/src/cli/plan/run.md)
- [write_spec](../../../../functions/src/cli/specify/write_spec.md)
- [load_template_from_override](../../../../functions/src/templates/resolver/load_template_from_override.md)
- [load_template_embedded_default](../../../../functions/src/templates/resolver/load_template_embedded_default.md)
- [load_unknown_template_returns_error](../../../../functions/src/templates/resolver/load_unknown_template_returns_error.md)