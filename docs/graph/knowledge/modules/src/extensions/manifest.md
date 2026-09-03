---
type: Rust Module
title: manifest
resource: src/extensions/manifest.rs#L1-L377
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
  - target: external/regex-regex
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

- [ExtensionManifest](../../../classes/src/extensions/manifest/ExtensionManifest.md)
- [ExtensionInfo](../../../classes/src/extensions/manifest/ExtensionInfo.md)
- [ExtensionRequires](../../../classes/src/extensions/manifest/ExtensionRequires.md)
- [ToolRequirement](../../../classes/src/extensions/manifest/ToolRequirement.md)
- [default_true](../../../functions/src/extensions/manifest/default_true.md)
- [ExtensionProvides](../../../classes/src/extensions/manifest/ExtensionProvides.md)
- [ExtensionCommand](../../../classes/src/extensions/manifest/ExtensionCommand.md)
- [ExtensionConfig](../../../classes/src/extensions/manifest/ExtensionConfig.md)
- [HookDef](../../../classes/src/extensions/manifest/HookDef.md)
- [load](../../../functions/src/extensions/manifest/ExtensionManifest/load.md)
- [parse](../../../functions/src/extensions/manifest/ExtensionManifest/parse.md)
- [validate](../../../functions/src/extensions/manifest/ExtensionManifest/validate.md)
- [parse_valid_manifest](../../../functions/src/extensions/manifest/parse_valid_manifest.md)
- [missing_required_fields_errors](../../../functions/src/extensions/manifest/missing_required_fields_errors.md)
- [invalid_version_specifier_errors](../../../functions/src/extensions/manifest/invalid_version_specifier_errors.md)
- [invalid_hook_trigger_errors](../../../functions/src/extensions/manifest/invalid_hook_trigger_errors.md)
- [valid_hooks_accepted](../../../functions/src/extensions/manifest/valid_hooks_accepted.md)
- [invalid_extension_id_errors](../../../functions/src/extensions/manifest/invalid_extension_id_errors.md)
- [invalid_command_name_errors](../../../functions/src/extensions/manifest/invalid_command_name_errors.md)
- [description_over_200_chars_errors](../../../functions/src/extensions/manifest/description_over_200_chars_errors.md)
- [empty_commands_list_errors](../../../functions/src/extensions/manifest/empty_commands_list_errors.md)
- [hook_referencing_undeclared_command_errors](../../../functions/src/extensions/manifest/hook_referencing_undeclared_command_errors.md)
- [hook_referencing_declared_command_passes](../../../functions/src/extensions/manifest/hook_referencing_declared_command_passes.md)

# Imports

- `std::sync::LazyLock`
- `anyhow::{Result, bail}`
- `regex::Regex`
- `serde::{Deserialize, Serialize}`
- `super::*`

# Member of

- [solidspec](../../../packages/solidspec.md)