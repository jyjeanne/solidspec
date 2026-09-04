---
type: Rust Function
title: resolve_cached
resource: src/templates/resolver.rs#L32-L50
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/templates/resolver/load_template
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_cached( template_name: &str, project_root: &Path, preset_priorities: &[(String, u32)], ) -> ResolvedTemplate`

# Called by

- [load_template](../../../../functions/src/templates/resolver/load_template.md)