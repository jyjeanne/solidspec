---
type: Rust Module
title: manifest
resource: src/presets/manifest.rs#L1-L229
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-sync-lazylock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result-bail
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [PresetManifest](../../../classes/src/presets/manifest/PresetManifest.md)
- [PresetInfo](../../../classes/src/presets/manifest/PresetInfo.md)
- [PresetRequires](../../../classes/src/presets/manifest/PresetRequires.md)
- [PresetProvides](../../../classes/src/presets/manifest/PresetProvides.md)
- [PresetTemplate](../../../classes/src/presets/manifest/PresetTemplate.md)
- [load](../../../functions/src/presets/manifest/PresetManifest/load.md)
- [parse](../../../functions/src/presets/manifest/PresetManifest/parse.md)
- [validate](../../../functions/src/presets/manifest/PresetManifest/validate.md)
- [parse_valid_manifest](../../../functions/src/presets/manifest/parse_valid_manifest.md)
- [missing_schema_version_errors](../../../functions/src/presets/manifest/missing_schema_version_errors.md)
- [wrong_schema_version_errors](../../../functions/src/presets/manifest/wrong_schema_version_errors.md)
- [invalid_semver_errors](../../../functions/src/presets/manifest/invalid_semver_errors.md)
- [invalid_id_with_uppercase_errors](../../../functions/src/presets/manifest/invalid_id_with_uppercase_errors.md)
- [unknown_template_type_errors](../../../functions/src/presets/manifest/unknown_template_type_errors.md)
- [invalid_version_specifier_errors](../../../functions/src/presets/manifest/invalid_version_specifier_errors.md)
- [description_over_200_chars_errors](../../../functions/src/presets/manifest/description_over_200_chars_errors.md)

# Imports

- `std::sync::LazyLock`
- `anyhow::{Result, bail}`
- `serde::{Deserialize, Serialize}`
- `super::*`

# Member of

- [solidspec](../../../packages/solidspec.md)