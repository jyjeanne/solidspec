---
type: Rust Module
title: init
resource: src/cli/init.rs#L1-L167
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-agents-registry
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-initoptions-projectinternalconfig-rootconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-git
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-extensions
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-templates
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [run](../../../functions/src/cli/init/run.md)
- [resolve_project_dir](../../../functions/src/cli/init/resolve_project_dir.md)
- [create_directory_structure](../../../functions/src/cli/init/create_directory_structure.md)
- [generate_constitution](../../../functions/src/cli/init/generate_constitution.md)
- [generate_agent_file](../../../functions/src/cli/init/generate_agent_file.md)

# Imports

- `std::collections::HashMap`
- `std::path::{Path, PathBuf}`
- `anyhow::Result`
- `crate::agents::registry`
- `crate::config::{InitOptions, ProjectInternalConfig, RootConfig}`
- `crate::core::git`
- `crate::extensions`
- `crate::templates`

# Member of

- [solidspec](../../../packages/solidspec.md)