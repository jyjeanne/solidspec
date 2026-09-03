---
type: Rust Function
title: install_okf_extension
resource: tests/okf_extension.rs#L33-L52
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/okf_extension/okf_extension_source
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/okf_extension/okf_extension_installs_and_registers_hook
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/okf_extension/okf_extension_hook_generates_a_real_bundle_when_solidspec_is_on_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/okf_extension/okf_extension_hook_never_fails_init_when_solidspec_is_not_on_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn install_okf_extension(dir: &std::path::Path)`

# Calls

- [okf_extension_source](../../../functions/tests/okf_extension/okf_extension_source.md)

# Called by

- [okf_extension_installs_and_registers_hook](../../../functions/tests/okf_extension/okf_extension_installs_and_registers_hook.md)
- [okf_extension_hook_generates_a_real_bundle_when_solidspec_is_on_path](../../../functions/tests/okf_extension/okf_extension_hook_generates_a_real_bundle_when_solidspec_is_on_path.md)
- [okf_extension_hook_never_fails_init_when_solidspec_is_not_on_path](../../../functions/tests/okf_extension/okf_extension_hook_never_fails_init_when_solidspec_is_not_on_path.md)