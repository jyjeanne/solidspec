---
type: Rust Function
title: with_claude_dir
resource: tests/init.rs#L20-L22
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/init/init_without_schema_defaults_to_minimal_and_persists_it
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/init/init_with_schema_flag_persists_the_chosen_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/init/init_with_unknown_schema_fails_before_writing_any_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/init/init_registers_spcx_commands_matching_the_chosen_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/init/init_registers_namespaced_spcx_commands_for_every_builtin_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/init/init_on_empty_directory_skips_knowledge_graph_generation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/init/init_on_existing_codebase_generates_knowledge_graph_and_mcp_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/init/init_on_existing_codebase_preserves_other_mcp_servers_already_configured
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/init/go_and_continue_use_the_projects_stored_default_schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn with_claude_dir(dir: &std::path::Path)`

# Called by

- [init_without_schema_defaults_to_minimal_and_persists_it](../../../functions/tests/init/init_without_schema_defaults_to_minimal_and_persists_it.md)
- [init_with_schema_flag_persists_the_chosen_schema](../../../functions/tests/init/init_with_schema_flag_persists_the_chosen_schema.md)
- [init_with_unknown_schema_fails_before_writing_any_files](../../../functions/tests/init/init_with_unknown_schema_fails_before_writing_any_files.md)
- [init_registers_spcx_commands_matching_the_chosen_schema](../../../functions/tests/init/init_registers_spcx_commands_matching_the_chosen_schema.md)
- [init_registers_namespaced_spcx_commands_for_every_builtin_schema](../../../functions/tests/init/init_registers_namespaced_spcx_commands_for_every_builtin_schema.md)
- [init_on_empty_directory_skips_knowledge_graph_generation](../../../functions/tests/init/init_on_empty_directory_skips_knowledge_graph_generation.md)
- [init_on_existing_codebase_generates_knowledge_graph_and_mcp_config](../../../functions/tests/init/init_on_existing_codebase_generates_knowledge_graph_and_mcp_config.md)
- [init_on_existing_codebase_preserves_other_mcp_servers_already_configured](../../../functions/tests/init/init_on_existing_codebase_preserves_other_mcp_servers_already_configured.md)
- [go_and_continue_use_the_projects_stored_default_schema](../../../functions/tests/init/go_and_continue_use_the_projects_stored_default_schema.md)