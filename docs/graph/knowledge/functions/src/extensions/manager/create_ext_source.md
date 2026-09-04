---
type: Rust Function
title: create_ext_source
resource: src/extensions/manager.rs#L177-L201
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/extensions/manager/add_dev_installs_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/dev_flag_set_in_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/remove_cleans_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/enable_disable_toggle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/disable_already_disabled_noop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/list_shows_all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/search_filters
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/info_by_id
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_ext_source(dir: &Path, id: &str) -> std::path::PathBuf`

# Called by

- [add_dev_installs_extension](../../../../functions/src/extensions/manager/add_dev_installs_extension.md)
- [dev_flag_set_in_registry](../../../../functions/src/extensions/manager/dev_flag_set_in_registry.md)
- [remove_cleans_up](../../../../functions/src/extensions/manager/remove_cleans_up.md)
- [enable_disable_toggle](../../../../functions/src/extensions/manager/enable_disable_toggle.md)
- [disable_already_disabled_noop](../../../../functions/src/extensions/manager/disable_already_disabled_noop.md)
- [list_shows_all](../../../../functions/src/extensions/manager/list_shows_all.md)
- [search_filters](../../../../functions/src/extensions/manager/search_filters.md)
- [info_by_id](../../../../functions/src/extensions/manager/info_by_id.md)