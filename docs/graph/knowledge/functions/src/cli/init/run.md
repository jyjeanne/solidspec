---
type: Rust Function
title: run
resource: src/cli/init.rs#L19-L131
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/cli/init/resolve_project_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/has_existing_codebase
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/by_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/create_directory_structure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/copy_embedded_templates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/copy_embedded_scripts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/generate_constitution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/generate_agent_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/IntentStatus/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/is_git_repo
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/git/init_repo
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/generate_knowledge_graph_and_mcp_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/manager/load_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/extensions/hooks/fire_hooks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run( name: Option<String>, here: bool, no_git: bool, _force: bool, agent: Option<String>, schema_name: Option<String>, ) -> Result<()>`

# Calls

- [resolve_project_dir](../../../../functions/src/cli/init/resolve_project_dir.md)
- [has_existing_codebase](../../../../functions/src/cli/init/has_existing_codebase.md)
- [by_name](../../../../functions/src/core/schema/by_name.md)
- [create_directory_structure](../../../../functions/src/cli/init/create_directory_structure.md)
- [copy_embedded_templates](../../../../functions/src/templates/copy_embedded_templates.md)
- [copy_embedded_scripts](../../../../functions/src/templates/copy_embedded_scripts.md)
- [generate_constitution](../../../../functions/src/cli/init/generate_constitution.md)
- [generate_agent_file](../../../../functions/src/cli/init/generate_agent_file.md)
- [register_all](../../../../functions/src/agents/registry/register_all.md)
- [as_str](../../../../functions/src/core/intent_parser/IntentStatus/as_str.md)
- [is_git_repo](../../../../functions/src/core/git/is_git_repo.md)
- [init_repo](../../../../functions/src/core/git/init_repo.md)
- [generate_knowledge_graph_and_mcp_config](../../../../functions/src/cli/init/generate_knowledge_graph_and_mcp_config.md)
- [load_registry](../../../../functions/src/extensions/manager/load_registry.md)
- [fire_hooks](../../../../functions/src/extensions/hooks/fire_hooks.md)