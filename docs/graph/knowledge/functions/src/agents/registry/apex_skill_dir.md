---
type: Rust Function
title: apex_skill_dir
resource: src/agents/registry.rs#L209-L217
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/registry/register_apex_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_apex_skill
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn apex_skill_dir(agent_id: &str, project_root: &Path) -> Option<PathBuf>`

# Called by

- [register_apex_skill](../../../../functions/src/agents/registry/register_apex_skill.md)
- [unregister_apex_skill](../../../../functions/src/agents/registry/unregister_apex_skill.md)