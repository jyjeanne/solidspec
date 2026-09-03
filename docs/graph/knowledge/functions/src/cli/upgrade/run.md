---
type: Rust Function
title: run
resource: src/cli/upgrade.rs#L7-L72
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/copy_embedded_scripts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_all
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(force: bool) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [all](../../../../functions/src/templates/all.md)
- [copy_embedded_scripts](../../../../functions/src/templates/copy_embedded_scripts.md)
- [register_all](../../../../functions/src/agents/registry/register_all.md)