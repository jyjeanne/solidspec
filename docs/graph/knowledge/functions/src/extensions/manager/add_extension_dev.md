---
type: Rust Function
title: add_extension_dev
resource: src/extensions/manager.rs#L10-L38
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/extensions/manager/copy_dir_safe
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/build_entry
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/extension/run
    resolved_by: tree-sitter
    confidence: exact
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

`pub fn add_extension_dev(project_root: &Path, source_dir: &Path) -> Result<String>`

# Calls

- [copy_dir_safe](../../../../functions/src/extensions/manager/copy_dir_safe.md)
- [build_entry](../../../../functions/src/extensions/manager/build_entry.md)

# Called by

- [run](../../../../functions/src/cli/extension/run.md)
- [add_dev_installs_extension](../../../../functions/src/extensions/manager/add_dev_installs_extension.md)
- [dev_flag_set_in_registry](../../../../functions/src/extensions/manager/dev_flag_set_in_registry.md)
- [remove_cleans_up](../../../../functions/src/extensions/manager/remove_cleans_up.md)
- [enable_disable_toggle](../../../../functions/src/extensions/manager/enable_disable_toggle.md)
- [disable_already_disabled_noop](../../../../functions/src/extensions/manager/disable_already_disabled_noop.md)
- [list_shows_all](../../../../functions/src/extensions/manager/list_shows_all.md)
- [search_filters](../../../../functions/src/extensions/manager/search_filters.md)
- [info_by_id](../../../../functions/src/extensions/manager/info_by_id.md)