---
type: Rust Module
title: upgrade
resource: src/cli/upgrade.rs#L1-L158
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-agents-registry-as-agent-registry
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-templates
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
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

- [run](../../../functions/src/cli/upgrade/run.md)
- [setup_project](../../../functions/src/cli/upgrade/setup_project.md)
- [upgrade_refreshes_templates](../../../functions/src/cli/upgrade/upgrade_refreshes_templates.md)
- [upgrade_preserves_constitution](../../../functions/src/cli/upgrade/upgrade_preserves_constitution.md)
- [upgrade_preserves_overrides](../../../functions/src/cli/upgrade/upgrade_preserves_overrides.md)
- [upgrade_preserves_specs](../../../functions/src/cli/upgrade/upgrade_preserves_specs.md)

# Imports

- `anyhow::{Context, Result}`
- `crate::agents::registry as agent_registry`
- `crate::config`
- `crate::templates`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)