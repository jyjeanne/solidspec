---
type: Rust Module
title: resolver
resource: src/templates/resolver.rs#L1-L314
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-cell-refcell
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ResolvedTemplate](../../../classes/src/templates/resolver/ResolvedTemplate.md)
- [TemplateSource](../../../classes/src/templates/resolver/TemplateSource.md)
- [resolve_cached](../../../functions/src/templates/resolver/resolve_cached.md)
- [resolve](../../../functions/src/templates/resolver/resolve.md)
- [load_template](../../../functions/src/templates/resolver/load_template.md)
- [setup_project](../../../functions/src/templates/resolver/setup_project.md)
- [override_present_wins](../../../functions/src/templates/resolver/override_present_wins.md)
- [preset_wins_when_no_override](../../../functions/src/templates/resolver/preset_wins_when_no_override.md)
- [extension_wins_when_no_override_or_preset](../../../functions/src/templates/resolver/extension_wins_when_no_override_or_preset.md)
- [embedded_default_when_nothing_else](../../../functions/src/templates/resolver/embedded_default_when_nothing_else.md)
- [override_beats_preset](../../../functions/src/templates/resolver/override_beats_preset.md)
- [lower_priority_number_preset_wins](../../../functions/src/templates/resolver/lower_priority_number_preset_wins.md)
- [empty_override_file_still_wins](../../../functions/src/templates/resolver/empty_override_file_still_wins.md)
- [load_template_from_override](../../../functions/src/templates/resolver/load_template_from_override.md)
- [load_template_embedded_default](../../../functions/src/templates/resolver/load_template_embedded_default.md)
- [load_unknown_template_returns_error](../../../functions/src/templates/resolver/load_unknown_template_returns_error.md)

# Imports

- `std::path::{Path, PathBuf}`
- `std::cell::RefCell`
- `std::collections::HashMap`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)