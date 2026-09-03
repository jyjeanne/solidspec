---
type: Rust Method
title: from_str
resource: src/core/intent_parser.rs#L41-L48
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/RootConfig/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/round_trip_serialize_deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/change/ChangeMetadata/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/extract_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/WorkflowSchema/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/vscode/merge_settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/vscode/merge_into_empty_creates_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/vscode/merge_preserves_existing_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/vscode/deep_merge_nested_objects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/vscode/arrays_replaced_not_merged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manifest/ExtensionManifest/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/registry/ExtensionRegistry/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manifest/PresetManifest/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/registry/PresetRegistry/load
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn from_str(s: &str) -> Self`

# Calls

- [as_str](../../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)

# Called by

- [load](../../../../../functions/src/config/RootConfig/load.md)
- [round_trip_serialize_deserialize](../../../../../functions/src/config/round_trip_serialize_deserialize.md)
- [load](../../../../../functions/src/core/change/ChangeMetadata/load.md)
- [extract_status](../../../../../functions/src/core/intent_parser/extract_status.md)
- [parse](../../../../../functions/src/core/schema/WorkflowSchema/parse.md)
- [merge_settings](../../../../../functions/src/core/vscode/merge_settings.md)
- [merge_into_empty_creates_file](../../../../../functions/src/core/vscode/merge_into_empty_creates_file.md)
- [merge_preserves_existing_keys](../../../../../functions/src/core/vscode/merge_preserves_existing_keys.md)
- [deep_merge_nested_objects](../../../../../functions/src/core/vscode/deep_merge_nested_objects.md)
- [arrays_replaced_not_merged](../../../../../functions/src/core/vscode/arrays_replaced_not_merged.md)
- [parse](../../../../../functions/src/extensions/manifest/ExtensionManifest/parse.md)
- [load](../../../../../functions/src/extensions/registry/ExtensionRegistry/load.md)
- [parse](../../../../../functions/src/presets/manifest/PresetManifest/parse.md)
- [load](../../../../../functions/src/presets/registry/PresetRegistry/load.md)