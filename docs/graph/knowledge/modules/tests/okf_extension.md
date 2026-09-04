---
type: Rust Module
title: okf_extension
resource: tests/okf_extension.rs#L1-L122
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/assert-cmd-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/predicates-prelude
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

- [okf_extension_source](../../functions/tests/okf_extension/okf_extension_source.md)
- [solidspec_bin_dir](../../functions/tests/okf_extension/solidspec_bin_dir.md)
- [path_with_solidspec_on_it](../../functions/tests/okf_extension/path_with_solidspec_on_it.md)
- [install_okf_extension](../../functions/tests/okf_extension/install_okf_extension.md)
- [okf_extension_installs_and_registers_hook](../../functions/tests/okf_extension/okf_extension_installs_and_registers_hook.md)
- [okf_extension_hook_generates_a_real_bundle_when_solidspec_is_on_path](../../functions/tests/okf_extension/okf_extension_hook_generates_a_real_bundle_when_solidspec_is_on_path.md)
- [okf_extension_hook_is_a_harmless_no_op_when_init_already_generated_natively](../../functions/tests/okf_extension/okf_extension_hook_is_a_harmless_no_op_when_init_already_generated_natively.md)

# Imports

- `assert_cmd::Command`
- `predicates::prelude::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../packages/solidspec.md)